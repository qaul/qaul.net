#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

while IFS= read -r package_dir; do
  if [ ! -d "${package_dir}/test" ]; then
    echo "==> skipping tests (${package_dir#"${PROJECT_DIR}/"})"
    continue
  fi

  echo "==> flutter test (${package_dir#"${PROJECT_DIR}/"})"
  (
    cd "${package_dir}"
    flutter test --reporter expanded
  )
done < <(
  find "${PROJECT_DIR}" -name "pubspec.yaml" -not -path "*/\.*" -type f \
    -exec dirname {} \; \
    | sort -u
)
