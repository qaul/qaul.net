#!/usr/bin/env python

import yaml
import os
from collections.abc import Mapping
import subprocess


def get_all_dependencies():
    cwd = os.path.dirname(os.path.realpath(__file__))
    os.chdir(cwd)

    with open("../pubspec.yaml", "r") as stream:
        try:
            yaml_file = yaml.safe_load(stream)

            dependencies = yaml_file['dependencies']
            dependencies.pop('flutter', None)
            return dependencies
        except yaml.YAMLError as exc:
            print(exc)


def remove_internal_dependencies(dependencies):
    return {k: dependencies[k] for k in dependencies if
            k != isinstance(dependencies[k], Mapping)}


if __name__ == "__main__":
    deps = get_all_dependencies()
    deps = remove_internal_dependencies(deps)

    print('Dependencies sorted alphabetically (excluding internal dependencies):')
    for dep in sorted(deps):
        print(f'\t{dep}: {deps[dep]}')

    print('\nCopy-Paste this value onto pubspec if you\'d like! ❤️')
