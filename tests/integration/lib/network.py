import os
import subprocess
import time

MESHNET_LAB = os.environ.get("MESHNET_LAB")


def apply_topology(topology_file: str):
    """Create the network from a topology JSON file."""
    subprocess.run(
        ["sudo", f"{MESHNET_LAB}/network.py", "apply", topology_file], check=True
    )


def clear_topology():
    """Remove all network namespaces."""
    subprocess.run(["sudo", f"{MESHNET_LAB}/network.py", "apply", "none"], check=True)


def start_qaul():
    """Start qauld in all namespaces.

    Kills any stale qauld processes first. software.py start qaul is a no-op
    when processes are already running (PID file check), and stop_qaul() cannot
    kill processes whose namespaces have already been deleted (get_remote_mapping
    returns empty). A hard pkill here guarantees a clean slate every time.
    """
    subprocess.run(["sudo", "pkill", "-SIGKILL", "-x", "qauld"], capture_output=True)
    time.sleep(2)
    subprocess.run(["sudo", f"{MESHNET_LAB}/software.py", "start", "qaul"], check=True)


def stop_qaul():
    """Stop all qauld instances."""
    subprocess.run(["sudo", f"{MESHNET_LAB}/software.py", "stop", "qaul"], check=True)


def kill_node(node_id: str):
    """Kill the qauld process for a single node.

    meshnet-lab starts qauld with --name=test-<node_id> and writes no PID
    file, so we find the process via pgrep.
    """
    result = subprocess.run(
        ["pgrep", "-f", f"qauld --name=test-{node_id}"],
        capture_output=True, text=True, check=True,
    )
    pid = result.stdout.strip()
    subprocess.run(["sudo", "kill", pid], check=True)


def wait_for_nodes(node_ids: list[str], timeout: int = 30):
    """Block until all nodes are reachable via qauld-ctl, or raise on timeout."""
    from lib.node import Node

    deadline = time.time() + timeout
    remaining = list(node_ids)

    while remaining and time.time() < deadline:
        still_down = []
        for nid in remaining:
            if not Node(nid).is_reachable():
                still_down.append(nid)
        remaining = still_down
        if remaining:
            time.sleep(1)

    if remaining:
        raise TimeoutError(f"Nodes still not reachable after {timeout}s: {remaining}")
