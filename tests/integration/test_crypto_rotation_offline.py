"""
Phase 4 — rotation during offline peer (DTN interaction).

The recipient is partitioned from the mesh via a topology swap (the
link between node 3 and node 4 is removed). The sender forces a
rotation while the recipient is unreachable, sends traffic in both the
established and the in-flight-rotation windows, and the link is then
restored. The test asserts:

  * every chat message the sender emitted lands on the recipient after
    reconvergence (DTN / messaging buffers absorb the outage)
  * the RotateHandshakeFirst eventually reaches the recipient; both
    peers log a Rotated event keyed on the sender's previous primary
  * the rotation's new_session_id is reflected on both sides

Partitioning by topology swap (rather than `kill_node`) keeps qauld
running on both ends — the DTN path we want to exercise buffers on
the sender, not state reload on the recipient.
"""

import os
import time

import pytest

from conftest import discovery_wait_for
from lib.network import apply_topology
from lib.node import Node
from lib.topology import load_node_ids

HERE = os.path.dirname(__file__)
FULL_TOPOLOGY = os.path.join(HERE, "topologies", "line-5.json")
SPLIT_TOPOLOGY = os.path.join(HERE, "topologies", "line-5-isolated4.json")

POLL_INTERVAL = 2
MESSAGES_BEFORE = 5
MESSAGES_DURING = 5
MESSAGES_AFTER = 5
DELIVERY_TIMEOUT = 180
ROTATION_TIMEOUT = 120


def _discover_peer(sender: Node, peer_name: str, timeout: int) -> dict:
    deadline = time.time() + timeout
    while time.time() < deadline:
        for u in sender.known_users():
            if u.get("name") == peer_name:
                return u
        time.sleep(POLL_INTERVAL)
    known = [u.get("name") for u in sender.known_users()]
    raise AssertionError(f"{peer_name} not discovered after {timeout}s — known: {known}")


def _conversation_contents(node: Node, group_id: str) -> list[str]:
    try:
        conv = node.conversation(group_id)
    except Exception:
        return []
    return [c for msg in conv.get("messages", []) for c in (msg.get("content") or [])]


def _await_all_delivered(
    recipient: Node, group_id: str, expected: list[str], timeout: int
) -> None:
    deadline = time.time() + timeout
    missing = set(expected)
    while time.time() < deadline and missing:
        missing -= set(_conversation_contents(recipient, group_id))
        if not missing:
            return
        time.sleep(POLL_INTERVAL)
    raise AssertionError(
        f"not delivered within {timeout}s; missing {len(missing)}/{len(expected)}: "
        f"{sorted(missing)[:5]}..."
    )


def _await_rotated_event(node: Node, previous_session_id: int, timeout: int) -> dict:
    deadline = time.time() + timeout
    while time.time() < deadline:
        for e in node.crypto_events():
            if (
                e.get("kind") == "Rotated"
                and e.get("draining_session_id") == previous_session_id
            ):
                return e
        time.sleep(POLL_INTERVAL)
    raise AssertionError(
        f"no Rotated event with draining_session_id={previous_session_id} on {node.id} "
        f"after {timeout}s"
    )


@pytest.mark.correctness
@pytest.mark.parametrize("converged_network", [FULL_TOPOLOGY], indirect=True)
def test_rotation_during_offline_peer(converged_network):
    """Rotate while the recipient is partitioned off, then heal the mesh."""
    ids = converged_network["node_ids"]
    sender_id, recipient_id = ids[0], ids[-1]
    sender, recipient = Node(sender_id), Node(recipient_id)
    discovery_wait = discovery_wait_for(converged_network["file"])

    # Pin rotation config so only manual triggers fire.
    for node in (sender, recipient):
        node.set_crypto_config(enabled=True)
        node.set_crypto_config(
            period_seconds=365 * 24 * 3600,
            volume_messages=10**9,
            grace_period_seconds=3600,
            grace_volume_messages=1024,
        )

    recipient_name = f"test-{recipient_id}"
    recipient_user = _discover_peer(sender, recipient_name, discovery_wait)
    group_id = recipient_user["group_id"]
    recipient_user_id = recipient_user["id"]

    # Phase A — establish the session while fully connected.
    tag = int(time.time())
    phase_a = [f"off-A-{tag}-{i}" for i in range(MESSAGES_BEFORE)]
    for m in phase_a:
        sender.send_chat_message(group_id, m)
    _await_all_delivered(recipient, group_id, phase_a, DELIVERY_TIMEOUT)

    # Partition — isolate the recipient by swapping to a topology that
    # omits the last link. Neither qauld process dies; libp2p on the
    # sender side will simply fail to push to the recipient until the
    # full topology is restored.
    apply_topology(SPLIT_TOPOLOGY)

    # Phase B — sender rotates while recipient is unreachable, and
    # keeps emitting messages. rotate_initiate never enters HalfOutgoing
    # on the sender's *primary* row (it stays Transport until the
    # responder's RotateHandshakeSecond arrives), so these messages
    # encrypt under the still-primary old session.
    resp = sender.rotate_with(recipient_user_id)
    previous_session_id = resp["previous_session_id"]
    new_session_id = resp["new_session_id"]
    assert previous_session_id != new_session_id

    phase_b = [f"off-B-{tag}-{i}" for i in range(MESSAGES_DURING)]
    for m in phase_b:
        sender.send_chat_message(group_id, m)

    # Heal the mesh. Buffered frames (including the rotate_first) now
    # race toward the recipient; we don't prescribe an order.
    apply_topology(FULL_TOPOLOGY)

    # Phase C — post-heal traffic. Some of these may race against
    # rotate_first on the wire; they must still decrypt correctly.
    phase_c = [f"off-C-{tag}-{i}" for i in range(MESSAGES_AFTER)]
    for m in phase_c:
        sender.send_chat_message(group_id, m)

    all_messages = phase_a + phase_b + phase_c
    _await_all_delivered(recipient, group_id, all_messages, DELIVERY_TIMEOUT)

    _await_rotated_event(recipient, previous_session_id, ROTATION_TIMEOUT)
    sender_evt = _await_rotated_event(sender, previous_session_id, ROTATION_TIMEOUT)
    assert sender_evt["primary_session_id"] == new_session_id

    # exactly-once
    final = _conversation_contents(recipient, group_id)
    for m in all_messages:
        count = final.count(m)
        assert count == 1, f"{m!r} delivered {count} times, expected 1"
