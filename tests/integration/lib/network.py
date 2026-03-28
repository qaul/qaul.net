import concurrent.futures
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
    subprocess.run(["sudo", "sh", "-c", "rm -f /tmp/qaul-*.pid"], capture_output=True)
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
    """Block until all nodes are reachable via qauld-ctl, checking in parallel."""
    from lib.node import Node

    deadline = time.time() + timeout
    remaining = set(node_ids)
    max_workers = min(len(node_ids), 32)

    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as pool:
        while remaining and time.time() < deadline:
            futures = {
                pool.submit(Node(nid).is_reachable): nid
                for nid in remaining
            }
            try:
                for future in concurrent.futures.as_completed(futures, timeout=max(1, deadline - time.time())):
                    nid = futures[future]
                    try:
                        if future.result():
                            remaining.discard(nid)
                    except Exception:
                        pass
            except concurrent.futures.TimeoutError:
                pass
            if remaining:
                time.sleep(1)

    if remaining:
        raise TimeoutError(f"Nodes still not reachable after {timeout}s: {sorted(remaining)}")
