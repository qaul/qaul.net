import concurrent.futures
import json
import subprocess


def batch_query(nodes: dict, method: str, *args, max_workers: int = 32) -> dict:
    """Call a Node method on multiple nodes in parallel.

    Args:
        nodes: {node_id: Node} dict
        method: name of a Node method (e.g. "feed_message_contents")
        *args: positional args forwarded to the method
        max_workers: max concurrent threads

    Returns: {node_id: result} dict
    """
    results = {}
    errors = {}

    with concurrent.futures.ThreadPoolExecutor(max_workers=min(len(nodes), max_workers)) as pool:
        futures = {
            pool.submit(getattr(node, method), *args): nid
            for nid, node in nodes.items()
        }
        for future in concurrent.futures.as_completed(futures):
            nid = futures[future]
            try:
                results[nid] = future.result()
            except Exception as e:
                errors[nid] = e

    if errors:
        raise RuntimeError(
            f"batch_query({method}) failed on {len(errors)} node(s): "
            + ", ".join(f"{nid}: {e}" for nid, e in errors.items())
        )
    return results


class Node:
    def __init__(self, id: str):
        self.id = id
        self.socket = f"/tmp/qaul-{id}/qauld.sock"

    def _run(self, *args) -> str:
        """Execute a qauld-ctl command, return stdout."""
        cmd = ["qauld-ctl", "--socket", self.socket] + list(args)
        res = subprocess.run(cmd, capture_output=True, text=True, timeout=30)

        if res.returncode != 0:
            raise RuntimeError(
                f"qauld-ctl failed on node {self.id}: {res.stderr.strip()}"
            )

        return res.stdout.strip()

    def _run_json(self, *args):
        """Execute qauld-ctl command with --json, return parsed JSON."""
        res = self._run("--json", *args)
        #print(f"{res}")
        return json.loads(res)

    def is_reachable(self) -> bool:
        """Check if qauld is reachable on this node."""
        try:
            self._run_json("node", "info")
            return True
        except Exception:
            return False

    def node_info(self) -> dict:
        """get node info"""
        return self._run_json("node", "info")

    def node_id(self) -> str:
        """get peerID of node"""
        return self.node_info()["node_id"]

    def known_users(self) -> list[dict]:
        """get list of known users"""
        return self._run_json("users", "list")

    def known_user_ids(self) -> list[str]:
        """get just the IDs of all known users."""
        return [u["id"] for u in self.known_users()]

    def get_user(self, user_id: str) -> dict:
        """get a user by ID"""
        return self._run_json("users", "get", "--user-id", user_id)

    def send_feed_message(self, message: str):
        """Send a feed message"""
        self._run("feed", "send", "--message", message)

    def feed_messages(self) -> list[dict]:
        """get feed messages"""
        return self._run_json("feed", "list")

    def feed_message_contents(self) -> list[str]:
        """get content of feed messages."""
        return [m["content"] for m in self.feed_messages()]

    def create_account(self, name: str, password: str = "") -> dict:
        """create a new account."""
        args = ["account", "create", "--username", name]
        if password:
            args += ["--password", password]
        return self._run_json(*args)

    def groups(self) -> list[dict]:
        """get all groups this node is a member of."""
        return self._run_json("group", "list")

    def group_info(self, group_id: str) -> dict:
        """Return info for a specific group."""
        return self._run_json("group", "info", "--id", group_id)

    def conversation(self, group_id: str, index: int = 0) -> dict:
        """Return conversation history for a group."""
        return self._run_json(
            "chat", "conversation", "--group-id", group_id, "--index", str(index)
        )

    def send_chat_message(self, group_id: str, message: str):
        """Send a direct/group chat message."""
        self._run("chat", "send", "--group-id", group_id, "--message", message)

    def router_table(self) -> list[dict]:
        """Return the current routing table: best route per known user."""
        return self._run_json("router", "table")

    def router_neighbours(self) -> dict:
        """Return direct neighbours split by module: {lan: [...], internet: [...], ble: [...]}."""
        return self._run_json("router", "neighbours")

    def router_connections(self) -> dict:
        """Return all candidate routes per user, pre best-route selection."""
        return self._run_json("router", "connections")

    def local_user_id(self) -> str:
        """Return the qaul user ID of this node's own local account."""
        # try users list first (connections[].module == "LOCAL")
        for user in self.known_users():
            for c in user.get("connections", []):
                if c.get("module") == "LOCAL":
                    return user["id"]
        # fallback: router table (connections[0].module == "Local")
        for entry in self.router_table():
            if entry["connections"] and entry["connections"][0]["module"].lower() == "local":
                return entry["user_id"]
        raise ValueError(f"No LOCAL user found on node {self.id}")

    # ---------------- crypto session rotation ----------------

    def crypto_config(self) -> dict:
        """Return the current CryptoRotation config on this node."""
        return self._run_json("crypto", "config")

    def set_crypto_config(
        self,
        enabled: bool | None = None,
        period_seconds: int | None = None,
        volume_messages: int | None = None,
        grace_period_seconds: int | None = None,
        grace_volume_messages: int | None = None,
    ) -> dict:
        """Update one or more CryptoRotation fields.

        `enabled=True/False` maps to the `enable`/`disable` subcommands; all
        other fields are sent via `crypto set`. The response is the updated
        config (so tests can assert success without a follow-up read)."""
        if enabled is True:
            return self._run_json("crypto", "enable")
        if enabled is False:
            return self._run_json("crypto", "disable")

        args: list[str] = ["crypto", "set"]
        if period_seconds is not None:
            args += ["--period-seconds", str(period_seconds)]
        if volume_messages is not None:
            args += ["--volume-messages", str(volume_messages)]
        if grace_period_seconds is not None:
            args += ["--grace-period-seconds", str(grace_period_seconds)]
        if grace_volume_messages is not None:
            args += ["--grace-volume-messages", str(grace_volume_messages)]
        if len(args) == 2:
            raise ValueError("set_crypto_config: provide at least one field to update")
        return self._run_json(*args)

    def rotate_with(self, user_id: str) -> dict:
        """Force a rotation with the peer identified by `user_id` (base58).

        Returns the TriggerRotationResponse JSON. Raises AssertionError if the
        daemon reports `success=false` so callers don't silently proceed on a
        failed rotation."""
        resp = self._run_json("crypto", "rotate", "--user-id", user_id)
        if not resp.get("success", False):
            raise AssertionError(
                f"rotate_with({user_id}) failed on {self.id}: {resp.get('error')}"
            )
        return resp

    def crypto_events(self, limit: int = 0, since_ms: int = 0) -> list[dict]:
        """Return the rotation event log, ordered oldest → newest.

        `limit=0` means "no cap"; `since_ms=0` means "no lower bound"."""
        return self._run_json(
            "crypto",
            "events",
            "--limit",
            str(limit),
            "--since-ms",
            str(since_ms),
        )
