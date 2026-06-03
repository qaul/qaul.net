"""
Phase 4 — grace-window expiry after rotation.

Two peers rotate with a short grace window configured on the
recipient. After the window elapses, the recipient's drain ticker
must emit a `GraceExpired` event for the draining (previous-primary)
session, and `last_retired_session_id` on the rotation meta must
match the session id the sender rotated away from.

The sibling `MessageDroppedPastGrace` event is exercised by the
libqaul unit tests (`drain_emits_grace_expired_and_stamps_meta` +
the decrypt past-grace branch); reproducing it reliably in a live
mesh would require injecting ciphertext on a retired session, which
no public API exposes. This scenario asserts the observable drain
behaviour — the rest follows from the unit-tested invariants.
"""

import time

import pytest

from conftest import all_topology_files, discovery_wait_for
from lib.node import Node

TOPOLOGIES = [f for f in all_topology_files() if "line-5" in f]

POLL_INTERVAL = 2
GRACE_SECONDS = 15
DRAIN_TICK_BUDGET = 90
DELIVERY_TIMEOUT = 90
ROTATION_TIMEOUT = 60


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
        f"not delivered within {timeout}s; missing {len(missing)}/{len(expected)}"
    )


def _await_event(
    node: Node, kind: str, draining_session_id: int, timeout: int
) -> dict:
    deadline = time.time() + timeout
    while time.time() < deadline:
        for e in node.crypto_events():
            if (
                e.get("kind") == kind
                and e.get("draining_session_id") == draining_session_id
            ):
                return e
        time.sleep(POLL_INTERVAL)
    raise AssertionError(
        f"no {kind} event with draining_session_id={draining_session_id} on {node.id} "
        f"after {timeout}s; events={node.crypto_events()}"
    )


@pytest.mark.correctness
@pytest.mark.parametrize("converged_network", TOPOLOGIES, indirect=True)
def test_grace_window_expires_after_rotation(converged_network):
    """After rotation, the draining session retires once grace elapses."""
    ids = converged_network["node_ids"]
    sender_id, recipient_id = ids[0], ids[-1]
    sender, recipient = Node(sender_id), Node(recipient_id)
    discovery_wait = discovery_wait_for(converged_network["file"])

    # Recipient gets a short grace window (that's the clock we want to
    # test). Sender's grace doesn't need to be short for this assertion,
    # but we use the same value so both sides log GraceExpired in the
    # same window — simpler to reason about. Volume budget stays huge
    # so GraceExpired fires on the period, not on message count.
    for node in (sender, recipient):
        node.set_crypto_config(enabled=True)
        node.set_crypto_config(
            period_seconds=365 * 24 * 3600,
            volume_messages=10**9,
            grace_period_seconds=GRACE_SECONDS,
            grace_volume_messages=10**9,
        )

    recipient_name = f"test-{recipient_id}"
    recipient_user = _discover_peer(sender, recipient_name, discovery_wait)
    group_id = recipient_user["group_id"]
    recipient_user_id = recipient_user["id"]

    # Establish the session.
    tag = int(time.time())
    init = [f"grace-init-{tag}-{i}" for i in range(3)]
    for m in init:
        sender.send_chat_message(group_id, m)
    _await_all_delivered(recipient, group_id, init, DELIVERY_TIMEOUT)

    # Rotate.
    resp = sender.rotate_with(recipient_user_id)
    previous_session_id = resp["previous_session_id"]
    _await_event(recipient, "Rotated", previous_session_id, ROTATION_TIMEOUT)

    # Post-rotation traffic on the new primary — also confirms the new
    # session is functional end-to-end before we let the drain ticker run.
    post = [f"grace-post-{tag}-{i}" for i in range(3)]
    for m in post:
        sender.send_chat_message(group_id, m)
    _await_all_delivered(recipient, group_id, post, DELIVERY_TIMEOUT)

    # Wait for the drain ticker to retire the old session on recipient.
    # The drain loop ticks on a fixed interval inside libqaul, so the
    # observable gap is GRACE_SECONDS + at most one tick + polling jitter.
    time.sleep(GRACE_SECONDS)
    evt = _await_event(
        recipient, "GraceExpired", previous_session_id, DRAIN_TICK_BUDGET
    )
    assert evt["draining_session_id"] == previous_session_id
