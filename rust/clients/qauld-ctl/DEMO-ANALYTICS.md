# Demoing the TUI analytics tabs

`./demo-analytics.sh` boots three local qauld nodes (alice / bob / carol),
wires them into a mesh, seeds every analytics tab, then opens the TUI and
runs a looping sequence of scripted **beats** — each beat makes one specific
tab visibly move, and announces itself in the feed a few seconds beforehand
so you have time to switch to that tab.

## Two terminals, two points of view

```sh
# window 1 — storage node / admin POV. Owns the demo.
cd rust/clients/qauld-ctl && ./demo-analytics.sh alice

# window 2 — field device POV. Attach any time.
cd rust/clients/qauld-ctl && ./demo-analytics.sh carol
```

Window 1 builds, boots the three nodes, seeds the tabs, starts the beat
driver and opens alice's TUI; quitting it (`q`) tears the whole demo down.
Window 2 waits for the demo to come up and attaches carol's TUI — open and
close it as often as you like.

Give both terminals ≥120 columns; the Network cards and the DTN sparkline
need the width. Setup takes ~30 s, then the first beat fires after a 25 s
lead-in (`LEAD_SECS`), and the cycle repeats forever.

## The beats

Narration arrives as feed messages (`▶ BEAT 3/6 — …`), so it shows up inside
both TUIs on the Feed tab.

| # | Tab | What happens | What you watch |
|---|---|---|---|
| 1 | **Users** | alice and carol each re-sign their own profile | your own row's `Bio` changes and `Ver` bumps v1→v2→v3 |
| 2 | **Feed** | burst of messages from all three nodes | rows arriving on both windows within a couple of seconds |
| 3 | **Network** | bob is killed | Internet peer card drops a peer, trend sparkline steps down, peer event log gets a line |
| 4 | **DTN** | carol keeps messaging offline bob; the messages park on alice | `unconfirmed` goes yellow and climbs, rolling sparkline rises |
| 5 | **DTN** + **Network** | bob restarts | delivery responses land in the live panel, `unconfirmed` drains back to green 0, the peer card recovers |
| 6 | **Crypto** | forced rotations plus a volume-triggered one (`volume_messages` dropped to 5, then 7 messages sent) | new newest-first rows in the rotation log, `rotated=` counter climbing |

Between beats a chatter loop keeps feed telemetry, chat pings and periodic
rotations flowing, so no panel ever goes flat.

Keys: `Tab` / `Shift-Tab` switch tabs, `↑`/`↓` move, `Enter` opens the detail
drawer (full untruncated ids, both session ids on a rotation row), `/`
filters the current tab — the table title switches to `filtered 1/3` — `r`
forces a refresh, `q` quits.

## One caveat worth knowing before you present

Profile updates (beat 1) are **not** propagated to peers that already know
you: signed profiles only travel in the `UserResponse` for *missing* user
ids (`router/info.rs` → `Users::get_missed_ids`). Verified on this branch —
after carol bumps her bio to v2, alice's Users row for carol still shows the
v1 profile with an empty bio. So beat 1 is a per-window observation: alice's
row changes in window 1, carol's row changes in window 2. Don't promise a
cross-node profile sync during the demo.

## Verifying before you present

```sh
./demo-analytics.sh --check
```

Runs one compressed beat cycle headless and asserts the data behind each tab
(it does not render the TUI — that needs a real terminal):

```
PASS Users tab   : 3 users known to alice
PASS Users tab   : 1 bios set, 1 profiles re-signed
PASS Feed tab    : 39 feed messages
PASS DTN tab     : 2 custodians configured
PASS DTN tab     : unconfirmed peaked at 2
PASS DTN tab     : drained back to 0
PASS Network tab : 4 peer rows
PASS Crypto tab  : 4 rotation events
```

## Knobs

| Env var | Default | Meaning |
|---|---|---|
| `DEMO_DIR` | `/tmp/qauld-demo` | where the three node dirs live |
| `PORT_A/B/C` | `9010/9011/9012` | internet-transport listen ports |
| `LEAD_SECS` | `25` | delay before the first beat (time to open window 2) |
| `GAP_SECS` | `25` | pause between beats |
| `BOB_OFFLINE_SECS` | `45` | how long bob stays offline in each cycle |

## Gotchas

- Quit with `q` in window 1 (or Ctrl-C) — the trap kills the daemons and the
  driver. If a run is orphaned: `./demo-analytics.sh --clean`.
- mDNS discovers *any* qauld node on your LAN, so an unrelated node of yours
  may appear in Users / Network. Filter with `/`.
- `--no-build` skips cargo when you are re-running the demo.
- Driver and chatter output land in `$DEMO_DIR/driver.log` and
  `$DEMO_DIR/chatter.log`, not on your terminal (it would corrupt the TUI).
