#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

while IFS= read -r package_dir; do
  echo "==> flutter pub get (${package_dir})"
  (
    cd "${PROJECT_DIR}/${package_dir}"
    flutter pub get
  )
done < <(
  find "${PROJECT_DIR}" -name "pubspec.yaml" -type f \
    | sed "s#${PROJECT_DIR}/##" \
    | xargs -I{} dirname "{}" \
    | sort -u
)
