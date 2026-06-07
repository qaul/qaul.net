# Testing the qauld-ctl TUI

The terminal UI is the `tui` subcommand of `qauld-ctl`, compiled in by
default. Drop it from a scripts-only build with
`cargo build -p qauld-ctl --no-default-features`.

## 1. Build

```sh
cd rust
cargo build -p qauld -p qauld-ctl
```

## 2. Single-node smoke (covers most features)

In one terminal, start a daemon in a fresh dir:

```sh
mkdir -p /tmp/qauld-A && cd /tmp/qauld-A
RUST_LOG=info /path/to/rust/target/debug/qauld
```

It auto-creates a default `Community Node <ts>` account. Leave it running.

In a second terminal, launch the TUI:

```sh
/path/to/rust/target/debug/qauld-ctl --dir /tmp/qauld-A tui
```

Top-level flags (`--socket` / `--dir` / `--timeout`) go before the
`tui` subcommand; TUI-specific flags like `--refresh` go after it.

You should see the header `node: Community Node 17... (12D3KooW…)`
populate within ~3 s.

### Feature checklist (single node)

| What to try | How | Expected |
|---|---|---|
| Tab switching | `Tab` / `Shift-Tab` | Cycles Users → Feed → Users. |
| Cursor move | `↑` / `↓` | Highlights rows; clamps at top/bottom. |
| Manual refresh | `r` | Re-fetches Users / Feed. |
| Send feed (the fix) | go to Feed, `s`, type, `Enter` | Events panel logs `feed sent: <text>`. Within 3 s the message appears in the Feed tab. **qauld.log must contain zero `couldn't be decoded` errors.** |
| Empty feed body | `s`, `Enter` without typing | Silent dismiss, no send. |
| Quit | `q` or `Ctrl-C` | Returns to the shell with terminal restored. |

### Per-tab data you should see

- **Users:** the auto-created `Community Node …` row.
- **Feed:** empty until you press `s` and send a message; then your
  own message appears.

## 3. Two-node setup (peer-dependent features)

In a third terminal, run a second node:

```sh
mkdir -p /tmp/qauld-B && cd /tmp/qauld-B
RUST_LOG=info /path/to/rust/target/debug/qauld
```

On macOS / LAN the two daemons should mDNS-discover each other within
~5 s.

| What to try | Where | Expected |
|---|---|---|
| **Users** tab populates | TUI on node A | The B node's `Community Node …` appears. |
| **Feed** federates | From B: `qauld-ctl --dir /tmp/qauld-B feed send -m "from B"` | The message appears on A's Feed tab within a few seconds. |

## 4. Regression test for the feed-send fix

```sh
cargo test -p qauld-ctl --bin qauld-ctl send_feed_refuses_empty_user_id
```

Should pass with `test result: ok. 1 passed`.

## 5. Scripts-only build (no TUI deps)

```sh
cargo build -p qauld-ctl --no-default-features
```

The resulting binary has no ratatui / crossterm deps and no `tui`
subcommand. Every other subcommand is unaffected.

## 6. Cleanup

```sh
pkill -9 -f 'target/debug/qauld'
rm -rf /tmp/qauld-A /tmp/qauld-B
```

## Known gaps

- Chat tab doesn't exist yet — only Feed for messaging.
- The TUI needs a real terminal — running it without a TTY exits
  with `Device not configured`.
</content>
</invoke>