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
            k != "cupertino_icons" and not isinstance(dependencies[k], Mapping)}


def form_command_with_dependency_name(name):
    bash_command = f"""if ! grep -qril "{name}" ../lib/*; then
      echo "{name} not used";
    fi"""
    return bash_command


def print_dependencies_not_found_in_lib(dependencies):
    for dependency in dependencies:
        cmd = form_command_with_dependency_name(dependency)
        subprocess.run(cmd,
                       shell=True, check=True,
                       executable='/bin/bash')


if __name__ == "__main__":
    deps = get_all_dependencies()
    deps = remove_internal_dependencies(deps)
    remove_internal_dependencies(deps)
    print_dependencies_not_found_in_lib(deps)
