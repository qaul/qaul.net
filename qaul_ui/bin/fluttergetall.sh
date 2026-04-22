#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

while IFS= read -r package_dir; do
  echo "==> flutter pub get (${package_dir#"${PROJECT_DIR}/"})"
  (
    cd "${package_dir}"
    flutter pub get
  )
done < <(
  find "${PROJECT_DIR}" -name "pubspec.yaml" -not -path "*/\.*" -type f \
    -exec dirname {} \; \
    | sort -u
)
