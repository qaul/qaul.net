#!/usr/bin/env bash
# Copyright (c) 2021 Open Community Project Association https://ocpa.ch
# This software is published under the AGPLv3 license.
#
# Demo driver for the qauld-ctl TUI analytics tabs.
#
# Boots three local qauld nodes (alice / bob / carol), wires them together,
# seeds every analytics tab, then opens the TUI and runs a looping sequence
# of scripted "beats" — each one makes a specific tab visibly move.
#
# Two terminals, two points of view:
#
#   window 1:  ./demo-analytics.sh alice    # storage node / admin POV
#   window 2:  ./demo-analytics.sh carol    # field device POV
#
# Window 1 owns the demo: it boots the nodes, drives the beats and tears
# everything down when you quit its TUI. Window 2 just attaches a second
# TUI and can be opened/closed at any time.
#
#   ./demo-analytics.sh --check      headless: run one cycle, assert tab data
#   ./demo-analytics.sh --no-build   skip cargo build
#   ./demo-analytics.sh --clean      tear down a previous run

set -uo pipefail

DEMO_DIR="${DEMO_DIR:-/tmp/qauld-demo}"
PORT_A="${PORT_A:-9010}"
PORT_B="${PORT_B:-9011}"
PORT_C="${PORT_C:-9012}"

# Beat timing. Lead-in gives you time to open the second window.
LEAD_SECS="${LEAD_SECS:-25}"
GAP_SECS="${GAP_SECS:-25}"
BOB_OFFLINE_SECS="${BOB_OFFLINE_SECS:-45}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RUST_DIR="$REPO_ROOT/rust"
QAULD="${QAULD:-$RUST_DIR/target/debug/qauld}"
CTL="${CTL:-$RUST_DIR/target/debug/qauld-ctl}"

MODE=alice
DO_BUILD=1
for arg in "$@"; do
  case "$arg" in
    alice|stage) MODE=alice ;;
    carol|join)  MODE=carol ;;
    --check)     MODE=check ;;
    --clean)     MODE=clean ;;
    --no-build)  DO_BUILD=0 ;;
    -h|--help)   sed -n '4,24p' "$0"; exit 0 ;;
    *) echo "unknown argument: $arg" >&2; exit 2 ;;
  esac
done

say()  { printf '\033[36m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[33m !!\033[0m %s\n' "$*"; }

# ctl <node> <args...>
ctl()  { "$CTL" --dir "$DEMO_DIR/$1" -t 5 "${@:2}"; }
ctlj() { "$CTL" --dir "$DEMO_DIR/$1" -t 5 --json "${@:2}"; }
q()    { "$CTL" --dir "$DEMO_DIR/$1" -t 5 "${@:2}" >/dev/null 2>&1; }

# ---------------------------------------------------------------- carol POV

if [ "$MODE" = carol ]; then
  say "waiting for the demo in $DEMO_DIR (start window 1 first)"
  for i in $(seq 1 120); do
    [ -S "$DEMO_DIR/carol/qauld.sock" ] && ctlj carol account default >/dev/null 2>&1 && break
    sleep 1
  done
  [ -S "$DEMO_DIR/carol/qauld.sock" ] || { warn "no demo running"; exit 1; }
  exec "$CTL" --dir "$DEMO_DIR/carol" tui --refresh 2
fi

# ------------------------------------------------------------------ cleanup

BG_PIDS=()

kill_pidfile() {
  local f="$DEMO_DIR/$1.pid"
  [ -f "$f" ] && kill "$(cat "$f")" 2>/dev/null
  rm -f "$f"
}

cleanup() {
  for p in "${BG_PIDS[@]:-}"; do [ -n "$p" ] && kill "$p" 2>/dev/null; done
  # PID-based only: pkill -f 'qauld --name alice' also matches this script.
  kill_pidfile alice; kill_pidfile bob; kill_pidfile carol
  wait 2>/dev/null
}

teardown_previous() {
  local stale
  stale="$(lsof -ti "tcp:$PORT_A" -i "tcp:$PORT_B" -i "tcp:$PORT_C" 2>/dev/null)"
  [ -n "$stale" ] && printf '%s\n' "$stale" | xargs kill -9 2>/dev/null
  rm -rf "$DEMO_DIR"
}

if [ "$MODE" = clean ]; then
  teardown_previous
  say "removed $DEMO_DIR and freed ports $PORT_A/$PORT_B/$PORT_C"
  exit 0
fi

trap cleanup EXIT INT TERM

# ------------------------------------------------------------------ helpers

start_node() {
  local node="$1" port="$2"
  mkdir -p "$DEMO_DIR/$node"
  ( cd "$DEMO_DIR/$node" && exec "$QAULD" --name "$node" --port "$port" \
      >> "$DEMO_DIR/$node/qauld.log" 2>&1 ) &
  echo $! > "$DEMO_DIR/$node.pid"
}

wait_ready() {
  local node="$1" i
  for i in $(seq 1 60); do
    ctlj "$node" account default >/dev/null 2>&1 && return 0
    sleep 0.5
  done
  warn "$node never answered on its socket; see $DEMO_DIR/$node/qauld.log"
  return 1
}

own_id()     { ctlj "$1" account default | jq -r '.id'; }
peer_id()    { ctlj "$1" users list | jq -r --arg n "$2" '.[] | select(.name==$n) | .id' | head -1; }
peer_group() { ctlj "$1" users list | jq -r --arg n "$2" '.[] | select(.name==$n) | .group_id' | head -1; }

wait_peer() {
  local node="$1" peer="$2" i
  for i in $(seq 1 60); do
    [ -n "$(peer_id "$node" "$peer")" ] && return 0
    sleep 1
  done
  warn "$node never discovered $peer"
  return 1
}

# Narration goes into the public feed, so it shows up inside both TUIs on
# the Feed tab — no third window needed to follow along.
narrate() { q alice feed send -m "$*"; echo "$(date +%H:%M:%S) $*"; }

# -------------------------------------------------------------------- build

if [ "$DO_BUILD" = 1 ]; then
  say "building qauld + qauld-ctl"
  ( cd "$RUST_DIR" && cargo build -p qauld -p qauld-ctl ) || exit 1
fi
[ -x "$QAULD" ] || { warn "missing $QAULD"; exit 1; }
[ -x "$CTL" ]   || { warn "missing $CTL"; exit 1; }
command -v jq >/dev/null || { warn "jq is required"; exit 1; }

# --------------------------------------------------------------------- boot

teardown_previous
say "booting alice ($PORT_A) / bob ($PORT_B) / carol ($PORT_C) under $DEMO_DIR"
start_node alice "$PORT_A"
start_node bob   "$PORT_B"
start_node carol "$PORT_C"
wait_ready alice || exit 1
wait_ready bob   || exit 1
wait_ready carol || exit 1

ID_A="$(own_id alice)"; ID_B="$(own_id bob)"; ID_C="$(own_id carol)"
say "alice=$ID_A"
say "bob=$ID_B"
say "carol=$ID_C"

say "connecting the three nodes over the internet transport"
q alice connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_B" -n bob
q alice connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_C" -n carol
q bob   connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_A" -n alice
q bob   connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_C" -n carol
q carol connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_A" -n alice
q carol connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_B" -n bob

wait_peer alice bob   || exit 1
wait_peer alice carol || exit 1
wait_peer carol bob   || exit 1
say "mesh is up"

# --------------------------------------------------------------------- seed

say "seeding feed / DTN / crypto so every tab has content on open"
q alice feed send -m "alice: storage node online"
q bob   feed send -m "bob: relay ready"
q carol feed send -m "carol: field device checking in"

# alice is the storage node (64 MB cap, bob + carol may deposit); carol and
# bob name alice as their storage node, so anything they send to an
# unreachable peer parks on alice instead of being dropped.
q alice dtn size --size 64
q alice dtn add --user-id "$ID_C"
q alice dtn add --user-id "$ID_B"
q carol dtn add --user-id "$ID_A"
q bob   dtn add --user-id "$ID_A"

GROUP_CB="$(peer_group carol bob)"
GROUP_CA="$(peer_group carol alice)"
GROUP_AC="$(peer_group alice carol)"

for n in alice bob carol; do q "$n" crypto enable; done
q alice crypto set --volume-messages 10
# Warm a KK session with carol (a chat — feed is floodsub, not a session)
# before rotating. Rotating with no established session no-ops and can
# leave a dangling pending-rotation that blocks the later crypto beat; that
# was why the Crypto tab under-counted. This seed gives the tab a real
# rotation (2 events) on open.
q alice chat send -g "$GROUP_AC" -m "alice: session warmup"
sleep 3
q alice crypto rotate --user-id "$ID_C"

# -------------------------------------------------------------------- beats
#
# One cycle ≈ 3 min. Each beat announces itself in the feed first, so you
# have a few seconds to switch to the tab it is about to move.

# Each node re-signs its OWN profile. A peer that already knows you keeps
# the profile it learned at discovery time (signed profiles only travel in
# the UserResponse for *missing* ids), so watch your own row: alice's row
# in window 1, carol's row in window 2.
beat_users() {
  narrate "▶ BEAT 1/6 — USERS tab: each node re-signs its own profile (watch YOUR row: Bio + Ver)"
  sleep 4
  q carol account update --bio "field device · battery 62% · GPS lock"
  sleep 6
  q alice account update --bio "storage node · 64 MB DTN cap"
  sleep 6
  q carol account update --bio "field device · battery 41% · moving"
  sleep 4
  q alice account update --bio "storage node · draining queue"
}

beat_feed() {
  narrate "▶ BEAT 2/6 — FEED tab: burst of federated messages"
  sleep 4
  q bob   feed send -m "bob: repeater on the hill, 3 links up"
  sleep 2
  q carol feed send -m "carol: rain starting, moving to shelter"
  sleep 2
  q bob   feed send -m "bob: ack, holding position"
  sleep 2
  q carol feed send -m "carol: sensor pack redeployed"
}

beat_network_down() {
  narrate "▶ BEAT 3/6 — NETWORK tab: bob drops off the mesh now"
  sleep 5
  kill_pidfile bob
}

beat_dtn_fill() {
  narrate "▶ BEAT 4/6 — DTN tab: carol keeps messaging offline bob"
  sleep 4
  local i
  for i in 1 2 3 4 5 6; do
    q carol chat send -g "$GROUP_CB" -m "carol -> bob (parked) #$i"
    sleep 2
  done
}

beat_dtn_drain() {
  narrate "▶ BEAT 5/6 — DTN + NETWORK: bob is back, storage drains"
  sleep 3
  start_node bob "$PORT_B"
  wait_ready bob
  # Re-dial the restarted bob from both peers so delivery resumes promptly;
  # without this the parked messages only drain if mDNS happens to
  # rediscover him inside the window, which is flaky.
  q alice connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_B" -n bob
  q carol connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_B" -n bob
  q bob   connections nodes add -a "/ip4/127.0.0.1/tcp/$PORT_A" -n alice
  wait_peer alice bob || warn "bob did not re-join in time"
}

beat_crypto() {
  narrate "▶ BEAT 6/6 — CRYPTO tab: forced + volume-triggered rotations"
  sleep 4
  # A rotation needs a live session to rotate. carol is always online, so
  # warm a session with a chat first, then rotate — otherwise the rotate
  # no-ops and no event is emitted.
  q alice chat send -g "$GROUP_AC" -m "alice: crypto warmup"
  sleep 3
  q alice crypto rotate --user-id "$ID_C"
  sleep 4
  q alice chat send -g "$GROUP_AC" -m "alice: post-rotate traffic"
  sleep 2
  q alice crypto rotate --user-id "$ID_C"
  sleep 4
  # then a volume-triggered rotation
  q alice crypto set --volume-messages 5
  local i
  for i in 1 2 3 4 5 6 7; do
    q alice chat send -g "$GROUP_AC" -m "alice: rotation traffic $i"
    sleep 1
  done
  q alice crypto set --volume-messages 10
}

run_cycle() {
  beat_users;        sleep "$GAP_SECS"
  beat_feed;         sleep "$GAP_SECS"
  beat_network_down; sleep 10
  beat_dtn_fill;     sleep "$BOB_OFFLINE_SECS"
  beat_dtn_drain;    sleep "$GAP_SECS"
  beat_crypto
}

# Low-level chatter so the sparklines and event panels never go flat.
chatter_loop() {
  local tick=0
  while true; do
    tick=$((tick + 1))
    q carol feed send -m "carol: telemetry tick $tick"
    q alice chat send -g "$GROUP_AC" -m "alice ping $tick"
    q carol chat send -g "$GROUP_CA" -m "carol pong $tick"
    [ $((tick % 4)) = 0 ] && q alice crypto rotate --user-id "$ID_C"
    sleep 8
  done
}

driver_loop() {
  sleep "$LEAD_SECS"
  local cycle=0
  while true; do
    cycle=$((cycle + 1))
    narrate "── demo cycle $cycle starting ──"
    run_cycle
    sleep "$GAP_SECS"
  done
}

# -------------------------------------------------------------- check mode

if [ "$MODE" = check ]; then
  say "headless check: running one beat cycle"
  LEAD_SECS=3 GAP_SECS=6 BOB_OFFLINE_SECS=25
  ( sleep 3; run_cycle ) > "$DEMO_DIR/driver.log" 2>&1 &
  cycle_pid=$!
  BG_PIDS+=("$cycle_pid")
  # Sample DTN's unconfirmed count until the cycle finishes — waiting on
  # the cycle (not a fixed count of iterations) so the assertions run after
  # every beat has executed. The last beat drives the rotations, so a fixed
  # window could end before it and undercount crypto events. Capped at 600s.
  DTN_PEAK=0
  for i in $(seq 1 600); do
    kill -0 "$cycle_pid" 2>/dev/null || break
    u="$(ctlj alice dtn state 2>/dev/null | jq -r '.unconfirmed_count // 0')"
    if [ -n "$u" ] && [ "$u" -gt "$DTN_PEAK" ] 2>/dev/null; then DTN_PEAK="$u"; fi
    sleep 1
  done
  sleep 3   # let the final beat's rotation events land before asserting

  fail=0
  check() { # check <label> <0|1>
    if [ "$2" = 1 ]; then printf '  \033[32mPASS\033[0m %s\n' "$1"
    else printf '  \033[31mFAIL\033[0m %s\n' "$1"; fail=1; fi
  }
  ge() { [ "${1:-0}" -ge "$2" ] && echo 1 || echo 0; }

  USERS="$(ctlj alice users list | jq 'length')"
  BIOS="$(ctlj alice users list | jq '[.[] | select(.bio != "" and .bio != null)] | length')"
  VERS="$(ctlj alice users list | jq '[.[] | select(.profile_version > 1)] | length')"
  FEED="$(ctlj alice feed list | jq 'length')"
  CUSTODIANS="$(ctlj alice dtn config | jq '.users | length')"
  PEERS="$(ctlj alice router connections | jq '[.. | objects | select(has("user_id"))] | length')"
  # Delivery and rotation land asynchronously, so poll (bounded) for the
  # expected end-state rather than sampling once the instant the cycle ends.
  DTN_NOW="$DTN_PEAK"
  for _ in $(seq 1 40); do
    DTN_NOW="$(ctlj alice dtn state | jq -r '.unconfirmed_count // 0')"
    [ "${DTN_NOW:-1}" -lt "${DTN_PEAK:-0}" ] 2>/dev/null && break
    sleep 1
  done
  ROTATIONS=0
  for _ in $(seq 1 30); do
    ROTATIONS="$(ctlj alice crypto events | jq 'length')"
    [ "${ROTATIONS:-0}" -ge 3 ] 2>/dev/null && break
    sleep 1
  done

  echo; say "tab data check"
  check "Users tab   : $USERS users known to alice"             "$(ge "$USERS" 3)"
  check "Users tab   : $BIOS bios set, $VERS profiles re-signed" "$(ge "$VERS" 1)"
  check "Feed tab    : $FEED feed messages"                     "$(ge "$FEED" 8)"
  check "DTN tab     : $CUSTODIANS custodians configured"       "$(ge "$CUSTODIANS" 2)"
  check "DTN tab     : unconfirmed peaked at $DTN_PEAK"         "$(ge "$DTN_PEAK" 1)"
  check "DTN tab     : drained back to $DTN_NOW"                "$([ "${DTN_NOW:-1}" -lt "${DTN_PEAK:-0}" ] && echo 1 || echo 0)"
  check "Network tab : $PEERS peer rows"                        "$(ge "$PEERS" 1)"
  check "Crypto tab  : $ROTATIONS rotation events"              "$(ge "$ROTATIONS" 3)"
  echo
  exit "$fail"
fi

# --------------------------------------------------------------- alice POV

chatter_loop > "$DEMO_DIR/chatter.log" 2>&1 &
BG_PIDS+=($!)
driver_loop > "$DEMO_DIR/driver.log" 2>&1 &
BG_PIDS+=($!)

cat <<EOF

  Demo is up. Open a SECOND terminal now and run:

      $SCRIPT_DIR/demo-analytics.sh carol

  That is the field-device POV; this window is the storage node (alice).

  The first beat starts in ${LEAD_SECS}s and the cycle repeats forever.
  Each beat announces itself in the FEED tab a few seconds before it
  happens, so you can switch to the tab it is about to move:

    1 USERS    each node re-signs its own profile — your own row's Bio
               and Ver bump (peers keep the profile they learned first)
    2 FEED     burst of federated messages from all three nodes
    3 NETWORK  bob leaves the mesh: peer card + peer event log
    4 DTN      carol's messages to bob park on alice: unconfirmed
               climbs, sparkline rises
    5 DTN      bob returns: delivery responses land, count drains to 0
    6 CRYPTO   forced + volume-triggered rotations, counters move

  Tab / Shift-Tab switches tabs, Enter opens the detail drawer,
  / filters, r refreshes, q quits (and tears the whole demo down).

EOF
sleep 8
"$CTL" --dir "$DEMO_DIR/alice" tui --refresh 2
