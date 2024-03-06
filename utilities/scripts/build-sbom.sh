#!/bin/bash
set +eo pipefail

#-----------------------------------------------------------------NoticeStart-
# Utilities
#
# Copyright (c) 2024 Open Community Project Association https://ocpa.ch
# This software is published under the AGPLv3 license.
#-----------------------------------------------------------------NoticeEnd---
#
# build-sbom
#
# DESCRIPTION
#   Creates consolidated SBOM reports for the repository.
#

# Pre-step: assert that the script is being run from the correct path
PATTERN="^.*/utilities/scripts\$"
if [[ ! $(pwd) =~ $PATTERN ]]; then
  echo "This script uses relative paths. Please run directly from it's directory."
  exit 1
fi

cd ../../qaul_ui/android || exit 1

flutter pub get
./gradlew cyclonedxBom -info --init-script init.gradle

cd ../../rust || exit 1

cargo install cargo-sbom
cargo sbom | tee sbom.spdx.json

cd .. || exit 1

syft . --config .syft/config.yaml

# +eo pipefail + exit 0 >> step succeeds regardless of a command failing
# This is used because of the cyclonedx gradle plugin, which errors even though it succeeds in
# generating the output file. See:
# - https://github.com/CycloneDX/cyclonedx-gradle-plugin/issues/194
# - https://github.com/CycloneDX/cyclonedx-gradle-plugin/issues/256
exit 0