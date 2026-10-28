#!/usr/bin/env bash
# Keep the lab's per-node logs before a teardown destroys them.
#
# qauld recreates /tmp/qaul-XXXX/qauld.err on every bring-up, so tearing a lab
# down loses the previous run's evidence. That is how the stranded-gateway run
# of 2026-10-27 was lost (docs/research/router-v2-cross-community-messaging.md).
# This copies every node's log, gzipped, into runs/<tag>-<start>/ together with
# the run's results and a log_counters.py summary.
#
# The directory is named by the moment the run came up, read from the logs
# themselves, so archiving one run twice - when it finishes, and again just
# before the next teardown - refreshes a single directory rather than making
# two. The tag comes from the marker communities/run.sh writes at bring-up.
#
# config.yaml is deliberately not copied: it holds the node's and the users'
# private keys and session tokens. Only the PeerIds are kept, in ids.txt.
#
# Reads /tmp/qaul-* only, so it is safe against a running lab. Run it as root
# when the logs are root-owned; the archive is handed back to $SUDO_USER.
#
#   sudo lab/archive_lab.sh            # tag from the marker, else "untagged"
#   sudo lab/archive_lab.sh my-tag     # explicit tag
#   RUNS=/elsewhere lab/archive_lab.sh # archive somewhere other than runs/
#
# runs/ is git-ignored: the archives are local evidence, not repository content.
set -uo pipefail

# runs/ and results/ are shared by every suite in tests/integration, one level up.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INT="$(dirname "$HERE")"
RUNS="${RUNS:-$INT/runs}"
RESULTS="$INT/results"
MARKER="${MARKER:-/tmp/qaul_lab_run.env}"

shopt -s nullglob
logs=(/tmp/qaul-*/qauld.err)
if (( ${#logs[@]} == 0 )); then
  echo "archive: no lab logs under /tmp/qaul-*/, nothing to keep"
  exit 0
fi

# The earliest first-line timestamp across the nodes is when the run came up.
START="$(for f in "${logs[@]}"; do head -n 1 "$f" | awk '{print $1}'; done |
  grep -E '^[0-9]{4}-[0-9]{2}-[0-9]{2}T' | sort | head -n 1)"
if [[ -z $START ]]; then
  START="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "archive: no timestamped log line yet, naming the run by the current time"
fi
STAMP="$(date -u -d "$START" +%Y%m%dT%H%M%SZ)"
START_EPOCH="$(date -u -d "$START" +%s)"

# communities/run.sh writes the marker just before bring-up. One written long
# before these logs began belongs to an earlier run and would mislabel a lab
# that something else started, so it is ignored.
MARKER_FRESH=""
if [[ -r $MARKER ]]; then
  written="$(stat -c %Y "$MARKER")"
  if (( written <= START_EPOCH + 5 && START_EPOCH - written < 600 )); then
    MARKER_FRESH=yes
  else
    echo "archive: $MARKER was not written just before these logs began ($START), ignoring it"
  fi
fi

# The marker is KEY=value lines; read it rather than sourcing it.
marker() { [[ -n $MARKER_FRESH ]] && sed -n "s/^$1=//p" "$MARKER" | head -n 1; }

TAG="${1:-$(marker TAG)}"
TAG="${TAG:-untagged}"

DEST="$RUNS/$TAG-$STAMP"
mkdir -p "$DEST" || exit 1

for f in "${logs[@]}"; do
  dir="$(dirname "$f")"
  node="${dir##*/qaul-}"
  mkdir -p "$DEST/$node"
  # Through a pipe: the log may still be growing, and gzip warns about any
  # regular file - even a redirected one - that changes size under it.
  cat "$f" | gzip -c > "$DEST/$node/qauld.err.gz"
  for keep in qauld.out manifest_state.yaml; do
    [[ -f $dir/$keep ]] && cp "$dir/$keep" "$DEST/$node/"
  done
  # In config.yaml the first indented `id:` is the node's, the rest its users'.
  [[ -f $dir/config.yaml ]] &&
    awk '/^  id: /{print (n++ ? "user_id=" : "node_id=") $2}' "$dir/config.yaml" \
      > "$DEST/$node/ids.txt"
done

# Results written since the run came up belong to it.
if [[ -d $RESULTS ]]; then
  since="$(mktemp)"
  touch -d "$START" "$since"
  mkdir -p "$DEST/results"
  for f in "$RESULTS"/*; do
    [[ -f $f && $f -nt $since ]] && cp "$f" "$DEST/results/"
  done
  rm -f "$since"
fi

roles="$(marker ROLES)"
[[ -n $roles && -f $roles ]] && cp "$roles" "$DEST/roles.json"

{
  echo "TAG=$TAG"
  echo "STARTED=$START"
  echo "NODES=${#logs[@]}"
  [[ -n $MARKER_FRESH ]] && grep -v '^TAG=' "$MARKER"
} > "$DEST/run.txt"
date -u +%Y-%m-%dT%H:%M:%SZ >> "$DEST/archived_at"

python3 "$HERE/log_counters.py" "$DEST" --json "$DEST/counters.json" > "$DEST/counters.txt" 2>&1

if [[ -n ${SUDO_UID:-} ]]; then
  chown "$SUDO_UID:${SUDO_GID:-$SUDO_UID}" "$RUNS"
  chown -R "$SUDO_UID:${SUDO_GID:-$SUDO_UID}" "$DEST"
fi

echo "archive: ${#logs[@]} node logs -> $DEST ($(du -sh "$DEST" | cut -f1))"
cat "$DEST/counters.txt"
