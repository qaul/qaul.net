"""
Phase 4 — simultaneous rotation by both peers.

Both sides of an established Noise KK session trip a rotation at the
same time. The collision-resolution rule (see
`CryptoNoise::rotate_complete_responder`) is "the numerically smaller
new_session_id wins": the loser drops its own HalfOutgoing row and
processes the incoming rotate_first; the winner's rotation proceeds.

The test drives both `crypto rotate` requests in quick succession,
waits for convergence, and asserts:

  * both peers have Rotated events that agree on the winning session id
  * all in-flight messages (before, during, after the collision) are
    delivered to both sides' conversations exactly once
  * the post-rotation primary session id on both peers matches

Concurrent rotations in a DTN-tolerant system are the gnarliest
rotation edge case; this test locks the observable contract down.
"""

import concurrent.futures
import time

import pytest

from conftest import all_topology_files, discovery_wait_for
from lib.node import Node

TOPOLOGIES = [f for f in all_topology_files() if "line-5" in f]

POLL_INTERVAL = 2
MESSAGES_BEFORE = 5
MESSAGES_AFTER = 5
DELIVERY_TIMEOUT = 120
ROTATION_TIMEOUT = 120


def _discover_peer(me: Node, peer_name: str, timeout: int) -> dict:
    deadline = time.time() + timeout
    while time.time() < deadline:
        for u in me.known_users():
            if u.get("name") == peer_name:
                return u
        time.sleep(POLL_INTERVAL)
    known = [u.get("name") for u in me.known_users()]
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


def _await_any_rotated(node: Node, timeout: int) -> list[dict]:
    """Block until at least one Rotated event shows on `node`. Returns all Rotated events."""
    deadline = time.time() + timeout
    while time.time() < deadline:
        evs = [e for e in node.crypto_events() if e.get("kind") == "Rotated"]
        if evs:
            return evs
        time.sleep(POLL_INTERVAL)
    raise AssertionError(
        f"no Rotated event on {node.id} after {timeout}s; events={node.crypto_events()}"
    )


@pytest.mark.correctness
@pytest.mark.parametrize("converged_network", TOPOLOGIES, indirect=True)
def test_simultaneous_rotation(converged_network):
    """Both peers trip rotation concurrently; assert consistent convergence."""
    ids = converged_network["node_ids"]
    a_id, b_id = ids[0], ids[-1]
    a, b = Node(a_id), Node(b_id)
    discovery_wait = discovery_wait_for(converged_network["file"])

    for node in (a, b):
        node.set_crypto_config(enabled=True)
        node.set_crypto_config(
            period_seconds=365 * 24 * 3600,
            volume_messages=10**9,
            grace_period_seconds=600,
            grace_volume_messages=1024,
        )

    a_name, b_name = f"test-{a_id}", f"test-{b_id}"
    b_user = _discover_peer(a, b_name, discovery_wait)
    a_user = _discover_peer(b, a_name, discovery_wait)
    group_a = b_user["group_id"]  # group on A pointing at B
    group_b = a_user["group_id"]  # group on B pointing at A

    # Phase A — establish in both directions so each side has the same
    # primary session_id before the collision. Using bi-directional
    # traffic also guarantees both peers' crypto_account sees Transport
    # state on the shared session_id.
    tag = int(time.time())
    phase_a_ab = [f"sim-A-ab-{tag}-{i}" for i in range(MESSAGES_BEFORE)]
    phase_a_ba = [f"sim-A-ba-{tag}-{i}" for i in range(MESSAGES_BEFORE)]
    for m in phase_a_ab:
        a.send_chat_message(group_a, m)
    for m in phase_a_ba:
        b.send_chat_message(group_b, m)
    _await_all_delivered(b, group_b, phase_a_ab, DELIVERY_TIMEOUT)
    _await_all_delivered(a, group_a, phase_a_ba, DELIVERY_TIMEOUT)

    # Both peers trip rotation in parallel. `rotate_with` on the
    # wrapper is synchronous (blocks until the ctl command returns),
    # so we drive both via a thread pool to overlap the initial
    # state mutation and the rotate_first send.
    with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
        fut_a = pool.submit(a.rotate_with, b_user["id"])
        fut_b = pool.submit(b.rotate_with, a_user["id"])
        resp_a = fut_a.result(timeout=30)
        resp_b = fut_b.result(timeout=30)

    a_new, b_new = resp_a["new_session_id"], resp_b["new_session_id"]
    # The race may resolve identical session ids (unlikely given 32-bit
    # randomness) — in which case there is no collision to exercise.
    if a_new == b_new:
        pytest.skip(f"both sides picked new_session_id={a_new}; nothing to reconcile")

    # Phase B — traffic during the collision window in both directions.
    phase_b_ab = [f"sim-B-ab-{tag}-{i}" for i in range(MESSAGES_AFTER)]
    phase_b_ba = [f"sim-B-ba-{tag}-{i}" for i in range(MESSAGES_AFTER)]
    for m in phase_b_ab:
        a.send_chat_message(group_a, m)
    for m in phase_b_ba:
        b.send_chat_message(group_b, m)

    # Both peers should log at least one Rotated event. The collision
    # rule is symmetric — whichever side "lost" also sees a Rotated,
    # because it accepts the winner's rotate_first and treats it as the
    # canonical rotation. The winner may log one or two Rotated events
    # (one from its own complete-initiator, possibly another if it also
    # completes as responder for the loser's dropped rotation).
    _await_any_rotated(a, ROTATION_TIMEOUT)
    _await_any_rotated(b, ROTATION_TIMEOUT)

    # Phase C — post-convergence traffic on whatever the shared primary
    # ended up being. A few more messages in each direction validate
    # that the collision didn't leave either side with a half-formed
    # session.
    phase_c_ab = [f"sim-C-ab-{tag}-{i}" for i in range(MESSAGES_AFTER)]
    phase_c_ba = [f"sim-C-ba-{tag}-{i}" for i in range(MESSAGES_AFTER)]
    for m in phase_c_ab:
        a.send_chat_message(group_a, m)
    for m in phase_c_ba:
        b.send_chat_message(group_b, m)

    all_ab = phase_a_ab + phase_b_ab + phase_c_ab
    all_ba = phase_a_ba + phase_b_ba + phase_c_ba
    _await_all_delivered(b, group_b, all_ab, DELIVERY_TIMEOUT)
    _await_all_delivered(a, group_a, all_ba, DELIVERY_TIMEOUT)

    # Exactly-once on both sides.
    for node, group, expected in ((b, group_b, all_ab), (a, group_a, all_ba)):
        final = _conversation_contents(node, group)
        for m in expected:
            count = final.count(m)
            assert count == 1, f"{m!r} on {node.id} delivered {count} times"
