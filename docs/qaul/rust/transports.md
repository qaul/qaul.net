# Transports

qaul talks to the network through three transports — LAN, the
Internet overlay, and Bluetooth LE. They share a common `Transport`
trait so the runtime can list them, query their status, and toggle
them on or off without knowing the concrete type. The persisted
on/off choice lives in `config.yaml` next to each transport's
existing settings.

## The `active` flag

Each transport carries an `active` boolean in `config.yaml`:

```yaml
lan:
  active: true
  listen:
    - /ip4/0.0.0.0/udp/0/quic-v1
    - /ip4/0.0.0.0/tcp/0
    - /ip6/::/udp/0/quic-v1
    - /ip6/::/tcp/0
internet:
  active: true
  peers:
    - address: /ip4/144.91.74.192/udp/9229/quic-v1
      name: qaul Community Node [IPv4]
      enabled: false
    - address: /ip6/2a02:c207:3004:3887::1/udp/9229/quic-v1
      name: qaul Community Node [IPv6]
      enabled: false
  do_listen: false
  listen:
    - /ip4/0.0.0.0/udp/0/quic-v1
    - /ip4/0.0.0.0/tcp/0
    - /ip6/::/udp/0/quic-v1
    - /ip6/::/tcp/0
ble:
  active: true
```

The flag is read once at startup and decides whether the transport
opens any listeners (LAN/Internet) or asks the OS to power on its
radio (BLE). When false, the transport is initialised in a
`Disabled` `TransportStatus` and stays out of the way: no listeners,
no peer dials, no inbound traffic.

The flag is also rewritten whenever the running daemon receives a
toggle command (see below). That makes the runtime choice durable —
disabling LAN once means LAN stays off across restarts until somebody
explicitly re-enables it.

## RPC surface (`Modules::Transports`)

The wire shape lives in
[`connections/transports.proto`](../../../protobuf/proto_definitions/connections/transports.proto).
Two requests, two responses:

| Request | Response | What it does |
|---|---|---|
| `TransportsListRequest` | `TransportsList` (repeated `TransportInfo`) | Lists every registered transport with its current status. |
| `TransportSetEnabled { id, enabled }` | `TransportSetEnabledResult { id, success, error }` | Starts or stops the transport identified by `id`. |

The `id` is a stable string: `lan`, `internet`, or `ble`. The set of
ids and their order is stable across the lifetime of a daemon
instance — clients can cache them.

`TransportInfo` reports per-transport metadata that the UI can use
without hard-coding transport names:

- `id`, `label` — programmatic and human-readable name.
- `status` — `running`, `disabled`, or an error string.
- `enabled` — convenience boolean derived from `status`.
- `supports_runtime_toggle` — whether `TransportSetEnabled` is
  meaningful for this transport. (Today every transport reports
  `true`; the field is in place for future read-only transports.)
- `is_local_only` — `true` for transports that never reach beyond
  the immediate physical neighbourhood (LAN, BLE). The UI uses this
  to decide whether traffic on this transport counts as internet
  traffic for billing or routing-policy purposes.

## CLI

`qaul-cli` has a `transports` subcommand:

```text
transports list
transports enable <id>
transports disable <id>
```

`transports list` prints a table:

```
id         | label                | status     | enabled | runtime_toggle |  local_only
lan        | LAN                  | running    |    true |           true |        true
internet   | Internet             | running    |    true |           true |       false
ble        | Bluetooth LE         | disabled   |   false |           true |        true
```

`transports enable <id>` and `transports disable <id>` both write
through to `config.yaml` on the daemon side. A failed call (unknown
id, transport not available on this build, OS rejected the BLE
request) prints `transport '<id>' update FAILED: <reason>`.

## Implementation notes

- The shared `Transport` trait lives in
  `rust/libqaul/src/connections/transport.rs`. Each concrete transport
  (`Lan`, `Internet`, `BleTransport`) implements it and updates its
  own `status` field.
- LAN and Internet write `config.{lan,internet}.active` from inside
  their `start` / `stop` impls. BLE does the same now that
  `config.ble.active` exists.
- `Connections::transports_rpc` is the dispatch glue between the
  proto messages and the trait. It receives `Option<&mut Lan>`,
  `Option<&mut Internet>`, and `Option<&mut BleTransport>` from the
  event loop so each toggle goes to the live instance, not a
  cached snapshot.
- `BleTransport::new` takes a `&QaulState` and reads
  `config.ble.active` to choose its initial `TransportStatus`. The
  BLE module's normal boot path (`Ble::info_received`) also gates
  the auto-start on the same flag, so a node that booted with
  `ble.active = false` does not silently turn the radio back on.

## Forward compatibility

The set of transports is closed today (`lan` / `internet` / `ble`).
Adding a fourth transport is a matter of:

1. Implementing the `Transport` trait on the new type.
2. Plumbing it through `Libqaul::run` and the RPC dispatch the same
   way `BleTransport` already is.
3. Adding the corresponding `active` flag to `Configuration`.

The CLI does not need updating: it forwards whatever id the daemon
exposes through `TransportsListRequest`.
