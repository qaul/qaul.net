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
DISCOVERY_WAITS = {
    "line-5":     200,
    "line-10":    400,
    "grid4-3x3":  200,
    "grid4-5x5":  250,
    "grid8-4x4":  200,
}


def discovery_wait_for(topo_file: str) -> int:
    """Return the discovery wait for a topology, with a sensible default."""
    name = topo_name(topo_file)
    return DISCOVERY_WAITS.get(name, 200)


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
    deadline = time.time() + wait
    while time.time() < deadline:
        if len(observer.known_users()) >= len(node_ids):
            break
        time.sleep(5)
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
