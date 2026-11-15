# Shared by run.sh and egress_check.sh; source it, don't run it.
# Needs SUITE (this directory), INT (tests/integration) and LAB (meshnet-lab).

# Archive the running lab, then scan that archive for sphere crossings.
# Returns the scan's status (0 also when there was nothing to archive).
archive_and_scan() {
  local out dest rc=0
  out="$(mktemp)"
  "$INT/lab/archive_lab.sh" | tee "$out"
  dest="$(sed -n 's/^archive: [0-9]* node logs -> \(.*\) ([^)]*)$/\1/p' "$out")"
  rm -f "$out"
  [[ -n $dest ]] || return 0
  LAST_ARCHIVE="$dest"
  "$SUITE/sphere_scan.py" "$dest" --json "$dest/sphere-scan.json" || rc=$?
  [[ -n ${SUDO_UID:-} && -f $dest/sphere-scan.json ]] &&
    chown "$SUDO_UID:${SUDO_GID:-$SUDO_UID}" "$dest/sphere-scan.json"
  return $rc
}

# Remove the lab, its daemons and any egress a hybrid run left behind.
teardown_lab() {
  (cd "$LAB" && ./network.py clear >/dev/null 2>&1)
  pkill -SIGKILL -x qauld 2>/dev/null
  rm -f /tmp/qaul-*.pid 2>/dev/null
  "$SUITE/egress.sh" down
  sleep 1
}
