import json
import subprocess


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
        """execut qauld-ctl command with --json, return jso output."""
        res = self._run("--json", *args)
        #print(f"{res}")
        return json.loads(res)

    def is_reachable(self) -> bool:
        """checks if qauld is reachanle on current node"""
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
        if password.__len__() != 0:
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
