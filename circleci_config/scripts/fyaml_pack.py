#!/usr/bin/env python3
"""Pack an FYAML directory tree into a single JSON document on stdout.

FYAML rules (https://github.com/CircleCI-Public/fyaml):
  - a directory becomes a mapping key
  - a file "X.yml" / "X.yaml" becomes mapping key "X"
  - a name beginning with "@" is merged into the enclosing map

`pack.sh` pipes the json output through `yq` to emit the final yaml.
"""

import json
import os
import subprocess
import sys

YAML_EXTENSIONS = (".yml", ".yaml")


def read_yaml(path):
    """Parse one yaml file into python data via yq"""
    result = subprocess.run(
        ["yq", "--output-format=json", "--indent=0", ".", path],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        sys.exit("failed to parse {}:\n{}".format(path, result.stderr.strip()))
    payload = result.stdout.strip()
    return json.loads(payload) if payload else None


def pack(directory):
    """Apply the FYAML rules to one directory"""
    packed = {}
    for name in sorted(os.listdir(directory)):
        path = os.path.join(directory, name)
        if os.path.isdir(path):
            key, value = name, pack(path)
        elif name.endswith(YAML_EXTENSIONS):
            key, value = os.path.splitext(name)[0], read_yaml(path)
        else:
            continue

        if name.startswith("@"):
            if not isinstance(value, dict):
                sys.exit("cannot merge non-mapping {} into its parent".format(path))
            packed.update(value)
        else:
            packed[key.lstrip("@")] = value
    return packed


def go_yaml_key_less(a, b):
    """Port of gopkg.in/yaml.v3's keyList.Less (sorter.go).

    Digit runs compare numerically, and where a letter meets a non-letter the
    non-letter sorts first.
    """
    digits = False
    i = 0
    while i < len(a) and i < len(b):
        if a[i] == b[i]:
            digits = a[i].isdigit()
            i += 1
            continue

        a_is_alpha, b_is_alpha = a[i].isalpha(), b[i].isalpha()
        if a_is_alpha and b_is_alpha:
            return a[i] < b[i]
        if a_is_alpha or b_is_alpha:
            return a_is_alpha if digits else b_is_alpha

        # Both are non-letters: compare the digit runs starting here.
        an = bn = 0
        if a[i] == "0" or b[i] == "0":
            j = i - 1
            while j >= 0 and a[j].isdigit():
                if a[j] != "0":
                    an = bn = 1
                    break
                j -= 1
        ai = i
        while ai < len(a) and a[ai].isdigit():
            an = an * 10 + int(a[ai])
            ai += 1
        bi = i
        while bi < len(b) and b[bi].isdigit():
            bn = bn * 10 + int(b[bi])
            bi += 1
        if an != bn:
            return an < bn
        if ai != bi:
            return ai < bi
        return a[i] < b[i]
    return len(a) < len(b)


def sort_deep(value):
    """Recursively order mapping keys consistently"""
    import functools

    if isinstance(value, dict):
        keys = sorted(
            value,
            key=functools.cmp_to_key(
                lambda x, y: -1 if go_yaml_key_less(x, y) else (1 if go_yaml_key_less(y, x) else 0)
            ),
        )
        return {k: sort_deep(value[k]) for k in keys}
    if isinstance(value, list):
        return [sort_deep(item) for item in value]
    return value


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: fyaml_pack.py <fyaml-directory>")
    directory = sys.argv[1]
    if not os.path.isdir(directory):
        sys.exit("not a directory: {}".format(directory))
    json.dump(sort_deep(pack(directory)), sys.stdout)


if __name__ == "__main__":
    main()
