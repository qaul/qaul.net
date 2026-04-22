#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

while IFS= read -r package_dir; do
  TEST_DIR="${PROJECT_DIR}/${package_dir}/test"
  if [ ! -d "${TEST_DIR}" ]; then
    echo "==> skipping tests (${package_dir})"
    continue
  fi

  echo "==> flutter test (${package_dir})"
  (
    cd "${PROJECT_DIR}/${package_dir}"
    flutter test
  )
done < <(
  find "${PROJECT_DIR}" -name "pubspec.yaml" -type f \
    | sed "s#${PROJECT_DIR}/##" \
    | xargs -I{} dirname "{}" \
    | sort -u
)
