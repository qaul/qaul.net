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
    """
    subprocess.run(["sudo", "pkill", "-SIGKILL", "-x", "qauld"], capture_output=True)
    subprocess.run(["sudo", "sh", "-c", "rm -f /tmp/qaul-*.pid"], capture_output=True)
    time.sleep(2)
    
    subprocess.run(
        ["sudo", "-E", f"{MESHNET_LAB}/software.py", "start", "qaul"], check=True
    )


def stop_qaul():
    """Stop all qauld instances."""
    subprocess.run(["sudo", f"{MESHNET_LAB}/software.py", "stop", "qaul"], check=True)


def wait_for_local_accounts(node_ids: list[str], timeout: int = 60):
    """Block until every node publishes its own account over the Local transport.
    """
    from lib.node import Node, SudoUnavailable

    deadline = time.time() + timeout
    remaining = set(node_ids)
    max_workers = min(len(node_ids), 32)

    def ready(nid: str) -> bool:
        Node(nid).local_user_id()
        return True

    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as pool:
        while remaining and time.time() < deadline:
            futures = {pool.submit(ready, nid): nid for nid in sorted(remaining)}
            for future in concurrent.futures.as_completed(futures):
                nid = futures[future]
                try:
                    if future.result():
                        remaining.discard(nid)
                except SudoUnavailable:
                    raise
                except Exception:
                    pass
            if remaining:
                time.sleep(1)

    if remaining:
        raise TimeoutError(
            f"{len(remaining)} node(s) never published a local account after "
            f"{timeout}s: {sorted(remaining)}"
        )


def kill_node(node_id: str):
    """Kill the qauld process for a single node.
    """
    result = subprocess.run(
        ["pgrep", "-f", f"qauld --name=test-{node_id}"],
        capture_output=True, text=True, check=True,
    )
    pid = result.stdout.strip()
    subprocess.run(["sudo", "kill", pid], check=True)


def wait_for_nodes(node_ids: list[str], timeout: int = 30):
    """Block until all nodes are reachable via qauld-ctl, checking in parallel."""
    from lib.node import Node, SudoUnavailable

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
                    except SudoUnavailable:
                        raise  # not a dead node; do not burn the whole timeout
                    except Exception:
                        pass
            except concurrent.futures.TimeoutError:
                pass
            if remaining:
                time.sleep(1)

    if remaining:
        raise TimeoutError(f"Nodes still not reachable after {timeout}s: {sorted(remaining)}")
