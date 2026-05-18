# qauld-ctl

A CLI client for the `qauld` daemon. Single-shot RPC, an interactive shell, and
a long-running event subscriber — all over the same Unix socket (TCP on
Windows).

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

## Output

Add `--json` (alias `-j`) for machine-readable output. Without it, you get a
plain-text layout meant for humans.

## Modes

| Mode        | Invocation                          | What it does                                                  |
| ----------- | ----------------------------------- | ------------------------------------------------------------- |
| Single-shot | `qauld-ctl <subcommand> ...`        | One RPC round-trip, print response, exit.                     |
| Shell       | `qauld-ctl shell`                   | REPL — type subcommands, each dispatched against the daemon.  |
| Subscribe   | `qauld-ctl subscribe`               | Streams events (chat, peers, etc.) until Ctrl-C.              |

## Command groups

Run `qauld-ctl <group> --help` for the full surface. The top-level groups are:

- `node` — node identity/details
- `account` — local user account: create, password, login/logout, status
- `users` — directory of network-known users (list, get, verify, block)
- `feed` — public broadcast feed
- `group` — group chat management
- `chat` — send / list messages
- `file` — chat file transfer
- `router` — routing table info
- `connections` — statically configured internet peers
- `dtn` — Delay-Tolerant Networking storage (V1)
- `debug` — daemon diagnostics (heartbeat, path, log toggles)

## Recipes

Smoke-test a fresh daemon:

```sh
qauld-ctl --dir /var/lib/qauld account create -u alice
qauld-ctl --dir /var/lib/qauld --json account default
qauld-ctl --dir /var/lib/qauld --json users list
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
