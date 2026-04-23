"""
Phase 4 — clean rotation under load.

Two peers in a converged line-5 mesh establish an end-to-end Noise KK
session, exchange chat traffic, force a rotation mid-stream via the
manual `crypto rotate` RPC, and continue sending. The test asserts:

  * every chat message sent from either side is delivered exactly once
    (pre-rotation, post-rotation, and in-flight messages straddling the
    rotation all land in the recipient's conversation)
  * both peers emit a `Rotated` event with matching session ids
  * the new primary session id differs from the old one

Configuration is set so the automatic time/volume triggers cannot
fire — the only rotation in the run is the one the test asks for.
"""

import time

import pytest

from conftest import all_topology_files, discovery_wait_for
from lib.node import Node
from lib.topology import load_node_ids

TOPOLOGIES = [f for f in all_topology_files() if "line-5" in f]

POLL_INTERVAL = 2
MESSAGES_PER_PHASE = 10
DELIVERY_TIMEOUT = 90
ROTATION_TIMEOUT = 60


def _discover_peer(sender: Node, peer_name: str, timeout: int) -> dict:
    """Block until `sender` sees a user named `peer_name`; return that user dict."""
    deadline = time.time() + timeout
    while time.time() < deadline:
        for u in sender.known_users():
            if u.get("name") == peer_name:
                return u
        time.sleep(POLL_INTERVAL)
    known = [u.get("name") for u in sender.known_users()]
    raise AssertionError(f"{peer_name} not discovered after {timeout}s — known: {known}")


def _conversation_contents(node: Node, group_id: str) -> list[str]:
    """Return all message content strings for `group_id` on `node`."""
    try:
        conv = node.conversation(group_id)
    except Exception:
        return []
    return [c for msg in conv.get("messages", []) for c in (msg.get("content") or [])]


def _await_all_delivered(
    recipient: Node, group_id: str, expected: list[str], timeout: int
) -> None:
    """Block until every item in `expected` appears in the recipient conversation."""
    deadline = time.time() + timeout
    missing = set(expected)
    while time.time() < deadline and missing:
        seen = set(_conversation_contents(recipient, group_id))
        missing -= seen
        if not missing:
            return
        time.sleep(POLL_INTERVAL)
    raise AssertionError(
        f"not delivered within {timeout}s; missing {len(missing)}/{len(expected)}: "
        f"{sorted(missing)[:5]}..."
    )


def _await_rotated_event(node: Node, previous_session_id: int, timeout: int) -> dict:
    """Poll crypto events on `node` until a Rotated event references `previous_session_id`."""
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
        f"after {timeout}s; events={node.crypto_events()}"
    )


@pytest.mark.correctness
@pytest.mark.parametrize("converged_network", TOPOLOGIES, indirect=True)
def test_clean_rotation_under_load(converged_network):
    """Rotate mid-stream; assert no message loss and matching events on both peers."""
    ids = converged_network["node_ids"]
    sender_id, recipient_id = ids[0], ids[-1]
    sender, recipient = Node(sender_id), Node(recipient_id)
    discovery_wait = discovery_wait_for(converged_network["file"])

    # Pin rotation config so only manual triggers fire.
    #   * enabled=True so the manual rotate request is accepted
    #   * period/volume well above anything this test can produce
    #   * grace window long enough to cover in-flight messages
    for node in (sender, recipient):
        node.set_crypto_config(enabled=True)
        node.set_crypto_config(
            period_seconds=365 * 24 * 3600,
            volume_messages=10**9,
            grace_period_seconds=300,
            grace_volume_messages=1024,
        )

    recipient_name = f"test-{recipient_id}"
    sender_name = f"test-{sender_id}"
    recipient_user = _discover_peer(sender, recipient_name, discovery_wait)
    _ = _discover_peer(recipient, sender_name, discovery_wait)

    group_id = recipient_user["group_id"]
    recipient_user_id = recipient_user["id"]

    # Phase A — establish the session and send load.
    tag = int(time.time())
    phase_a = [f"rot-A-{tag}-{i}" for i in range(MESSAGES_PER_PHASE)]
    for m in phase_a:
        sender.send_chat_message(group_id, m)
    _await_all_delivered(recipient, group_id, phase_a, DELIVERY_TIMEOUT)

    # Rotate. With no pre-rotation events, the sender's own event log
    # will hold the ground truth for (previous, new) session ids.
    resp = sender.rotate_with(recipient_user_id)
    previous_session_id = resp["previous_session_id"]
    new_session_id = resp["new_session_id"]
    assert previous_session_id != 0
    assert new_session_id != 0
    assert previous_session_id != new_session_id

    # Phase B — traffic through the rotation. Send without waiting so
    # some messages encrypt under the old session and arrive after the
    # peer has promoted the new one.
    phase_b = [f"rot-B-{tag}-{i}" for i in range(MESSAGES_PER_PHASE)]
    for m in phase_b:
        sender.send_chat_message(group_id, m)

    # The recipient's Rotated event is what we actually care about —
    # it proves the handshake round-trip completed on the other side.
    _await_rotated_event(recipient, previous_session_id, ROTATION_TIMEOUT)
    sender_evt = _await_rotated_event(sender, previous_session_id, ROTATION_TIMEOUT)
    assert sender_evt["primary_session_id"] == new_session_id, (
        f"sender Rotated event should point at new_session_id={new_session_id}; "
        f"got {sender_evt}"
    )

    # Phase C — post-rotation traffic on the new primary.
    phase_c = [f"rot-C-{tag}-{i}" for i in range(MESSAGES_PER_PHASE)]
    for m in phase_c:
        sender.send_chat_message(group_id, m)

    _await_all_delivered(
        recipient, group_id, phase_a + phase_b + phase_c, DELIVERY_TIMEOUT
    )

    # Every message should appear exactly once in the conversation.
    final = _conversation_contents(recipient, group_id)
    for m in phase_a + phase_b + phase_c:
        count = final.count(m)
        assert count == 1, f"{m!r} delivered {count} times, expected 1"
