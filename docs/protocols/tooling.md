# Operator Tooling — qauld & qauld-ctl

Running a node and inspecting/controlling it from the terminal.

## qauld: run detached

`qauld -d` / `--daemonize` forks the daemon into the background at startup and
returns the shell immediately. It keeps the storage directory as its working
directory, writes a pidfile there, and redirects stdout/stderr to log files.

```sh
qauld -d --name alice --port 9229   # start detached
```

The fork happens before the async runtime starts (forking a running
multithreaded process is unsafe), so `main` parses args, daemonizes, then builds
the runtime.

Code: `rust/clients/qauld/src/main.rs`.

## qauld-ctl: interactive shell

`qauld-ctl shell` runs commands against the daemon in a loop without re-typing
the binary name. It has line editing and history (↑/↓, ←/→ via rustyline) and
shell-style quoting (via shlex), so `feed send -m "hello there"` is a single
argument. `help` shows the command list; `quit` / `exit` / Ctrl-D leave.

Code: `rust/clients/qauld-ctl/src/shell/`.

## qauld-ctl: TUI dashboard

`qauld-ctl tui` is a live terminal dashboard, five tabs:

- **Users / Feed** — known users; feed messages (compose from the Feed tab).
- **DTN** — storage state, an unconfirmed-count sparkline, allowed custodians,
  and a live delivery-response feed.
- **Network** — per-module (LAN / Internet / BLE) peer counts with trend
  sparklines, a peers table, and a live connect/disconnect feed.
- **Crypto** — rotation config, event counts, and a live rotation-event table.

Keys: `Tab`/`BackTab` switch tabs, `↑/↓` move, `Enter` detail, `/` filter,
`r` refresh, `q` quit.

Fetching runs in a background task off the render loop, so input stays
responsive even if the daemon is slow or unreachable — the UI keeps painting the
last snapshot instead of freezing.

Code: `rust/clients/qauld-ctl/src/tui/`.

## qauld-ctl: transport switch

Enable or disable network transports (LAN / Internet / BLE) at runtime. This
actually starts/stops the transport and persists the choice, not just a flag.

```sh
qauld-ctl transports list
qauld-ctl transports disable -i lan
qauld-ctl transports enable  -i lan
```

Disabling a transport stops it carrying data. Note the routing view
(`router neighbours`) can lag behind — a peer may still appear for a while after
its transport is disabled, until the routing table's maintenance ages it out.
