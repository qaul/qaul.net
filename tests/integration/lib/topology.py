import json


def load_node_ids(topology_file: str) -> list[str]:
    """Return all node IDs from a meshnet-lab topology JSON file."""
    with open(topology_file) as f:
        data = json.load(f)
    return [n["id"] for n in data["nodes"]]
