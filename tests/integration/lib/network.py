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
    """Start qauld in all namespaces."""
    subprocess.run(["sudo", f"{MESHNET_LAB}/software.py", "start", "qaul"], check=True)


def stop_qaul():
    """Stop all qauld instances."""
    subprocess.run(["sudo", f"{MESHNET_LAB}/software.py", "stop", "qaul"], check=True)


def kill_node(node_id: str):
    """Kill the qauld process for a single node by reading its PID file."""
    import subprocess
    pid_file = f"/tmp/qaul-{node_id}.pid"
    with open(pid_file) as f:
        pid = f.read().strip()
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
