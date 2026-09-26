# DTN v2 — Custody Store-and-Forward

Delivers a message when the recipient is offline: instead of failing, it is
handed to **custodian** nodes that hold it and carry it forward until it
reaches the recipient. qaul's delay-tolerant layer, version 2.

Code: `rust/libqaul/src/services/dtn/mod.rs`; wire messages under
`protobuf/proto_definitions/services/messaging/`.

## Model

Three roles, all PeerIds: the **sender**, one or more **custodians** (opt-in
nodes that store-and-forward for others), and the **recipient**. A message
travels a **custody route** — an ordered list of custodian hops toward the
recipient. Each custodian holds the still-encrypted message and forwards it to
the next reachable hop, or delivers directly if the recipient is reachable. The
body stays sealed end to end; custodians store ciphertext and see only routing
headers.

## Lifecycle

1. **Send** — sender wraps its signed container in a `DtnRoutedV2` envelope
   (route + sender public key) and hands it to the first reachable custodian.
2. **Accept** — the custodian checks admission (custody enabled? signature
   valid? sender not blocked? within quota?) and stores + accepts, or rejects
   with a reason. Duplicates (same signature) are accepted silently.
3. **Forward** — try the recipient directly, else the next reachable custodian.
4. **Confirm** — once the next hop accepts, the holder releases its copy; the
   recipient's receipt unwinds the chain.

A periodic sweep re-attempts forwarding and garbage-collects expired entries.

## Storage, quota, retention

- **Per-account total** — `storage.size_total` (default 1024 MB).
- **Per-sender quota** — one sender can hold at most a fixed slice (currently
  10 MB), so no sender crowds out others.
- **Retention** — kept until delivered or aged out, measured from the
  custodian's **own receive time** (`accepted_at`), not a sender timestamp; even
  a no-expiry message is bounded by a max retention (currently 7 days). This is
  deliberate — a sender-supplied absolute expiry breaks on a wrong clock (expiry
  in the past → rejected everywhere; far future → never clears). Measuring from
  receipt is clock-independent.

Quota accounting self-heals: the entry and its counter are separate writes, so
the counter is recomputed from stored entries on startup.

## Rejections

Custodians decline with a reason so the sender can react instead of blind-retry:

- **`USER_NOT_ACCEPTED`** — custody off, sender blocked, or account unknown.
- **`OVERALL_QUOTA`** — account storage full.
- **`USER_QUOTA`** — this sender's slice full.

Intended sender-side handling (direction, not fully built): retry transient
reasons (full/error) with backoff; drop and surface hard denials (blocked).

## Operation

Custody is **opt-in** — a fresh node stores nothing for others until enabled.

```sh
qauld-ctl dtn custody enable      # become a custodian
qauld-ctl dtn state               # storage used, message + unconfirmed counts
qauld-ctl dtn config              # max size + allowed custodian users
qauld-ctl dtn add  -u <peer-id>   # allow a user to deposit here
qauld-ctl dtn size -s <MB>        # set the account storage cap
```

## Rationale

- **Opt-in, admission keyed on the signed sender** — identity is unforgeable, so
  admission/quota can trust "who sent this." (Per-sender quota alone doesn't
  stop a Sybil flood of many cheap identities — that needs an aggregate cap on
  the untrusted tier, a design direction, not yet built.)
- **Custodian-local retention, not a sender TTL** — clock-independent; hence the
  max-retention safety net even without a declared expiry.
- **Body never decrypted in transit** — custodians route ciphertext;
  confidentiality doesn't rely on trusting them.

## Route format — in flux

The custody-route wire structure is under active redesign (hop numbering, route
signing, stateless vs. cursor traversal). The mechanics above are the stable
core; the route message shape is expected to change. See the DTN structures
feedback thread.
