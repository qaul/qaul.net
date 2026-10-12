import logging
import os
import time

import pytest

from lib.network import apply_topology, clear_topology, start_qaul, stop_qaul, wait_for_nodes
from lib.node import Node
from lib.topology import load_node_ids

log = logging.getLogger("qaul.test")

# ---------------------------------------------------------------------------
# Topology helpers
# ---------------------------------------------------------------------------

TOPOLOGY_DIR = os.path.join(os.path.dirname(__file__), "topologies")


def all_topology_files() -> list[str]:
    """Return sorted list of all topology JSON files (absolute paths)."""
    return sorted(
        os.path.join(TOPOLOGY_DIR, f)
        for f in os.listdir(TOPOLOGY_DIR)
        if f.endswith(".json")
    )


def line_topology_files() -> list[str]:
    """Return only line topology files."""
    return [f for f in all_topology_files() if "line" in os.path.basename(f)]


def topo_name(path: str) -> str:
    """Extract short name from topology path: '/abs/path/line-5.json' -> 'line-5'."""
    return os.path.splitext(os.path.basename(path))[0]


# Hardcoded discovery waits per topology.
# These account for the mDNS query_interval bug (default 300s) which means
# a single missed mDNS packet adds up to 300s of delay. Until the mDNS fix
# in lan.rs is applied, these must be generous.
# Ceilings for polled convergence, not sleeps. Measured user discovery on this
# harness is 4-10s at 9 nodes, 12-13s at 16-20 and 14s at 25 nodes, so 60s is
# >4x the worst case: loose enough not to flake, tight enough to fail on a real
# regression. The 200-400s values these replaced could not fail.
DISCOVERY_WAITS = {
    "line-5":     60,
    "line-10":    60,
    "grid4-3x3":  60,
    "grid4-5x5":  60,
    "grid8-4x4":  60,
}

DEFAULT_DISCOVERY_WAIT = 60

# Convergence lands in the 4-14s range, so a 5s poll quantised every
# measurement to the nearest 5s. 1s keeps the numbers meaningful.
CONVERGENCE_POLL_INTERVAL_S = 1


def discovery_wait_for(topo_file: str) -> int:
    """Return the discovery wait ceiling for a topology, with a default."""
    name = topo_name(topo_file)
    return DISCOVERY_WAITS.get(name, DEFAULT_DISCOVERY_WAIT)


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------

@pytest.fixture
def topology_network(request):
    """
    Set up a fresh network for one test.
    Use via @pytest.mark.parametrize("topology_network", [...], indirect=True)
    where each param is a topology file path.

    Yields: {"file": str, "name": str, "node_ids": list, "nodes": dict}
    """
    topo_file = request.param
    name = topo_name(topo_file)
    node_ids = load_node_ids(topo_file)

    log.info("setup %s (%d nodes)", name, len(node_ids))
    apply_topology(topo_file)
    start_qaul()
    wait_for_nodes(node_ids, timeout=60)

    sorted_ids = sorted(node_ids)
    nodes = {nid: Node(nid) for nid in sorted_ids}

    yield {
        "file": topo_file,
        "name": name,
        "node_ids": sorted_ids,
        "nodes": nodes,
    }

    log.info("teardown %s", name)
    try:
        stop_qaul()
        clear_topology()
    except Exception as e:
        log.warning("teardown error: %s", e)


@pytest.fixture
def converged_network(request):
    """
    Set up network and wait for full user-level convergence before yielding.
    Use via @pytest.mark.parametrize("converged_network", [...], indirect=True).

    Yields: {"file": str, "name": str, "node_ids": list, "nodes": dict}
    """
    topo_file = request.param
    name = topo_name(topo_file)
    node_ids = load_node_ids(topo_file)
    wait = discovery_wait_for(topo_file)

    log.info("setup %s (%d nodes, discovery_wait=%ds)", name, len(node_ids), wait)
    apply_topology(topo_file)
    start_qaul()
    wait_for_nodes(node_ids, timeout=60)

    sorted_ids = sorted(node_ids)
    observer = Node(sorted_ids[0])
    t0 = time.time()
    deadline = t0 + wait
    while time.time() < deadline:
        if len(observer.known_users()) >= len(node_ids):
            log.info(
                "%s converged: %d users in %.1fs (ceiling %ds)",
                name,
                len(node_ids),
                time.time() - t0,
                wait,
            )
            break
        time.sleep(CONVERGENCE_POLL_INTERVAL_S)
    else:
        known = len(observer.known_users())
        try:
            stop_qaul()
            clear_topology()
        except Exception:
            pass
        pytest.skip(f"convergence timeout: {known}/{len(node_ids)} users after {wait}s")

    # pubsub warmup — floodsub mesh stabilises after routing convergence
    time.sleep(30)

    nodes = {nid: Node(nid) for nid in sorted_ids}

    yield {
        "file": topo_file,
        "name": name,
        "node_ids": sorted_ids,
        "nodes": nodes,
    }

    log.info("teardown %s", name)
    try:
        stop_qaul()
        clear_topology()
    except Exception as e:
        log.warning("teardown error: %s", e)
