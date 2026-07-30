#!/usr/bin/env bash
# Pull the newest qaul session log from every attached device into one folder.
#
#   ./pull-sessions.sh [outdir]
#
# Default outdir: ./runs/<timestamp>. Files land as <outdir>/<model>-<serial>.jsonl,
# ready to multi-select in mesh_replay.html. Pass -a to pull ALL session files per
# device instead of just the newest.
set -uo pipefail

PKG=net.qaul.qaul_app
REMOTE_DIR=/storage/emulated/0/Android/data/$PKG/files/sessions
INT_DIR=files/sessions        # internal fallback, relative to the app's home dir (run-as)
ALL=0
[[ "${1:-}" == "-a" ]] && { ALL=1; shift; }
OUT="${1:-runs/$(date +%Y%m%d-%H%M%S)}"
mkdir -p "$OUT"

# List the files to fetch for one device. $3 is a command prefix ("" for external,
# "run-as PKG " for the internal fallback).
list_files() {
  local serial="$1" dir="$2" pre="$3"
  if [[ $ALL -eq 1 ]]; then
    adb -s "$serial" shell "${pre}ls $dir 2>/dev/null" | tr -d '\r' | grep '\.jsonl$' \
      | while IFS= read -r n; do echo "$dir/$n"; done
  else
    # Newest of EACH kind — the BLE module writes session-*.jsonl and the Dart side writes
    # routing-*.jsonl into the same dir, and the replay tool wants both. (-t = newest first.)
    for kind in session routing; do
      adb -s "$serial" shell "${pre}ls -t $dir 2>/dev/null" | tr -d '\r' \
        | grep "^$kind-.*\.jsonl$" | head -1 | while IFS= read -r n; do echo "$dir/$n"; done
    done
  fi
}

# plain word-splitting, not mapfile — macOS ships bash 3.2
SERIALS=$(adb devices | awk 'NR>1 && $2=="device" {print $1}')
if [[ -z "$SERIALS" ]]; then
  echo "No devices attached (check: adb devices)." >&2
  exit 1
fi

echo "Pulling from $(echo "$SERIALS" | wc -l | tr -d ' ') device(s) → $OUT"
fail=0
for s in $SERIALS; do
  model=$(adb -s "$s" shell getprop ro.product.model 2>/dev/null | tr -d '\r' | tr ' /' '-')
  [[ -z "$model" ]] && model=device

  # Logs live in INTERNAL storage (read via run-as) — Android 11+ blocks adb from the external
  # /Android/data/<pkg>/ dir entirely, even under run-as. External is still checked as a fallback
  # so older runs, written before that change, are still pullable.
  MODE=int
  names=$(list_files "$s" "$INT_DIR" "run-as $PKG ")
  if [[ -z "$names" ]]; then
    MODE=ext
    names=$(list_files "$s" "$REMOTE_DIR" "")
    [[ -n "$names" ]] && echo "  · $model ($s): using external storage (legacy run)"
  fi

  if [[ -z "$names" ]]; then
    echo "  ! $model ($s): no session files found (checked internal + external)" >&2
    echo "    if this is Android 11+, confirm the app is a debug build: adb -s $s shell run-as $PKG ls files" >&2
    fail=1
    continue
  fi

  while IFS= read -r remote; do
    [[ -z "$remote" ]] && continue
    base=$(basename "$remote" .jsonl)
    kind=${base%%-*}          # "session" or "routing" — keeps the two from overwriting each other
    if [[ $ALL -eq 1 ]]; then
      dest="$OUT/$model-$s-$base.jsonl"
    else
      dest="$OUT/$kind-$model-$s.jsonl"
    fi
    ok=0
    if [[ $MODE == ext ]]; then
      adb -s "$s" pull -a "$remote" "$dest" >/dev/null 2>&1 && ok=1
    else
      # exec-out keeps the stream binary-clean (plain `shell` mangles \n into \r\n)
      adb -s "$s" exec-out run-as "$PKG" cat "$remote" > "$dest" 2>/dev/null \
        && [[ -s "$dest" ]] && ok=1
    fi
    if [[ $ok -eq 1 ]]; then
      lines=$(wc -l < "$dest" | tr -d ' ')
      echo "  ✓ $model ($s): $base — $lines lines"
    else
      rm -f "$dest"
      echo "  ! $model ($s): pull failed for $remote" >&2
      fail=1
    fi
  done <<< "$names"
done

echo
echo "Done → $OUT"
[[ $fail -eq 0 ]] || echo "(some devices had errors — see above)" >&2
