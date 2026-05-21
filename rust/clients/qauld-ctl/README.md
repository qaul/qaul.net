# qauld-ctl

A CLI client for the `qauld` daemon. Single-shot RPC, an interactive shell, and
a long-running event subscriber — all over the same Unix socket (TCP on
Windows). Designed to be usable interactively, by admins on headless nodes, and
by automated test harnesses.

## Build

```sh
cargo build -p qauld-ctl
```

The binary is placed at `target/debug/qauld-ctl` (or `target/release/...`).

## Connecting

`qauld-ctl` needs to find the daemon's socket. In order of precedence:

| Source          | Form                                       |
| --------------- | ------------------------------------------ |
| `--socket`      | explicit path: `--socket /run/qauld.sock`  |
| `$QAULD_SOCKET` | same as `--socket`, but from the env       |
| `--dir`         | directory containing `qauld.sock`          |
| _(default)_     | `./qauld.sock` in the current directory    |

On Windows, `--socket host:port` (default `127.0.0.1:9199`).

## Script-friendly defaults

- **Silent by default.** Connection banners go to stderr only when `--verbose`
  is set, so `--json` output is always clean and pipeable.
- **`--json` / `-j`** produces machine-readable output for every subcommand.
- **`--timeout <secs>` / `-t`** (default `10`) caps how long the CLI will wait
  for a response. A non-responding daemon exits with a non-zero status instead
  of hanging.
- **Non-zero exit codes** on RPC-level failures, malformed responses, closed
  connections, and timeouts. Diagnostics go to stderr.

## Modes

| Mode        | Invocation                          | What it does                                                  |
| ----------- | ----------------------------------- | ------------------------------------------------------------- |
| Single-shot | `qauld-ctl <subcommand> ...`        | One RPC round-trip, print response, exit.                     |
| Shell       | `qauld-ctl shell`                   | REPL — type subcommands, each dispatched against the daemon.  |
| Subscribe   | `qauld-ctl subscribe`               | Streams events (chat, peers, etc.) until Ctrl-C.              |

## Command groups

Run `qauld-ctl <group> --help` for the full surface. The top-level groups are:

- `node` — node identity/details
- `account` — local user account: create, password, login/logout, status, update profile
- `users` — directory of network-known users (list, get, verify, block, security number)
- `feed` — public broadcast feed
- `group` — group chat management
- `chat` — send / list / search messages
- `file` — chat file transfer
- `router` — routing table info
- `connections` — statically configured internet peers
- `transports` — toggle LAN / Internet / BLE
- `dtn` — Delay-Tolerant Networking storage + DTN V2 source routing (custody / send-routed)
- `crypto` — Noise session rotation config + manual rotate + event log
- `debug` — daemon diagnostics (heartbeat, path, log toggles)

## Recipes

Smoke-test a fresh daemon:

```sh
qauld-ctl --dir /var/lib/qauld account create -u alice
qauld-ctl --dir /var/lib/qauld --json account default
qauld-ctl --dir /var/lib/qauld --json users list
```

Update a profile (name, bio, avatar, preferred custody route):

```sh
qauld-ctl --dir /var/lib/qauld account update -n Alice2 -b "hi" \
  --avatar /path/to/avatar.png -c <peer-id>
# wipe the custody route again:
qauld-ctl --dir /var/lib/qauld account update -c clear
```

Inspect transports and toggle one off:

```sh
qauld-ctl --dir /var/lib/qauld --json transports list
qauld-ctl --dir /var/lib/qauld transports disable -i internet
```

Crypto session rotation:

```sh
qauld-ctl --dir /var/lib/qauld --json crypto config
```

DTN V2 custody routing:

```sh
qauld-ctl --dir /var/lib/qauld dtn custody enable
qauld-ctl --dir /var/lib/qauld dtn send-routed --route <hop1>,<hop2>,<dest> ...
```

Watch live events in one terminal, drive the daemon from another:

```sh
# terminal 1
qauld-ctl --dir /var/lib/qauld subscribe

# terminal 2
qauld-ctl --dir /var/lib/qauld shell
```

## Logging

Logs from `qauld-ctl` itself respect `RUST_LOG` (standard `env_logger` syntax,
e.g. `RUST_LOG=debug`). Daemon-side logging is controlled by `qauld`.
