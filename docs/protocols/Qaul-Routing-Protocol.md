# Qaul Routing Protocol

**Version:** 2.0
**Status:** Draft Specification
**Date:** May 2026

## Abstract

The Qaul Routing Protocol is a distance-vector routing protocol for the
qaul.net mesh. It carries reachability information for users and for
nodes across heterogeneous transports including LAN, Internet, and
Bluetooth Low Energy. The protocol scales from village-sized deployments
of a few dozen nodes to networks on the order of one hundred thousand
nodes connected across many regions. It tolerates partitioned operation
and supports gateway-based delegation across network boundaries.

This document specifies online distance-vector routing only. Storage of
undeliverable messages for later delivery is the responsibility of a
separate delay-tolerant networking layer that operates above this
protocol.

## Status of This Document

This is a working specification for the qaul.net project. The wire
format, timing parameters, and protocol invariants described here are
intended to be stable for a v2 implementation. Items deferred to future
versions are listed in Section 13.3.

## Table of Contents

1. Introduction
2. Network Model
3. Identifiers
4. Transports
5. Routing Metric
6. Sequence Numbers
7. Routing Updates
8. Wire Format
9. Routing Table
10. Delegation
11. Network Management Sub-Protocol
12. Partition Merge
13. Security Considerations
14. Timing Parameters
Appendix A. Implementation Notes (Non-Normative)

---

## 1. Introduction

### 1.1. Requirements Language

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD
NOT, RECOMMENDED, MAY, and OPTIONAL in this document are to be
interpreted as described in RFC 2119 when they appear in capital letters.

### 1.2. Terminology

**Node**
A physical device running qaul software. Nodes own routing state and
exchange routing messages.

**User**
A cryptographic identity hosted on a node. A node MAY host one or more
users. A user exists on at most one node at any given time.

**Host Node**
The node currently hosting a given user.

**Neighbour**
A node reachable in one hop over a single transport.

**Gateway**
A node with an active INTERNET transport connection. See Section 2.3.

**Origin**
The node from which a routing entry begins propagating. For a user
entry, the origin is the user's host node. For a node entry, the origin
is the node itself.

**Manifest**
The signed, versioned list of users delegated to a particular node.
See Section 10.

**Sphere**
A grouping of transports that share propagation rules. Two spheres are
defined: Local and Internet. See Section 2.3.

**Index**
A 16-bit local identifier used in place of a full public key in routing
messages.

---

## 2. Network Model

### 2.1. Assumptions

The protocol operates over a mobile ad-hoc mesh whose topology cannot
be predetermined. Typical deployments are villages connected by gateways,
in linear chains, irregular grids, or mixed. The network is not always
connected; partitions are normal and expected. The protocol is designed
to function without dependence on internet or other external
infrastructure.

A node MAY connect to multiple transports simultaneously, including
combinations such as LAN with BLE and INTERNET. The ratio of gateway
nodes to non-gateway nodes is not predictable in advance and varies
between deployments.

### 2.2. Maximum Diameter

The protocol's design maximum is **63 hops** within a single sphere.

The cap is sphere-local. A Local sphere is a village-scale BLE+LAN
mesh; realistic worst-case diameter is on the order of 20-30 hops in
sparse rural BLE-chain deployments, well under the cap. An Internet
sphere connects gateways via direct overlay links and typically
diameters at 2-3 hops, with longer paths plausible only when relay
overlays chain. 63 hops gives generous headroom against either sphere
in isolation.

A routing entry whose hop count would exceed 63 after the receiver's
increment SHALL be dropped without further processing. This acts as a
bounded TTL and prevents indefinite propagation in pathological chain
topologies.

The cap applies within a single sphere. End-to-end packet paths that
traverse multiple spheres MAY exceed 63 cumulative hops. Each gateway
re-originates routes on the far side of a sphere boundary; the hop
counter is sphere-local. No global cap is defined.

The hop count occupies the lower six bits of a one-byte field on
the wire (Section 8.3). Bit 7 is the `local_only` flag indicating
whether the entry has traveled exclusively over Local-sphere
transports along its propagation path so far (see Section 7.4 for
the propagation rule and Section 9.1 for how receivers store it).
Bit 6 is reserved for future use, MUST be set to zero by senders
in this version of the protocol, and SHALL be masked off by
receivers before any hop-count comparison.

At a relay interval of one second, a 63-hop path takes approximately
thirty-two seconds to fill its propagation pipeline initially, after
which a node at the far end receives consecutive updates with a mean
inter-arrival gap of ten seconds and a 99th-percentile gap of
approximately seventeen seconds. The route expiry timeout
(Section 7.5) is chosen to accommodate this distribution with margin
for missed updates.

### 2.3. Spheres

Two spheres are defined for the purposes of update propagation:

| Sphere   | Transports                                   |
|----------|----------------------------------------------|
| Local    | BLE 1M, BLE Coded, LAN                       |
| Internet | INTERNET, plus any future overlay or relay   |

Sphere membership is a static property of each transport, declared
at compile time as part of the transport module. A transport's
sphere reflects its role: transports that connect physical,
locally-discovered peers (LAN, BLE) belong to the Local sphere;
transports that connect wide-area peers through infrastructure
(INTERNET, and future overlay or relay transports) belong to the
Internet sphere. This role is intrinsic to the transport;
assignment does not change at runtime.

Propagation rules differ by sphere. Both are applied by the sender,
per outgoing transport:

* Over a **Local-sphere transport**, a node propagates routing
  entries (both user entries and node entries) it learned from 
  within the Local sphere. It SHALL NOT propagate entries it 
  learned from the Internet sphere. A Local sphere therefore 
  carries only its own members' routing state.
* Over an **Internet-sphere transport**, a node propagates node
  entries for gateways only, node entries whose target has
  `is_gateway = 1` (Section 10.1). User entries and non-gateway
  node entries SHALL be filtered out.

A gateway therefore sends different content to its Local-sphere
neighbours than to its Internet-sphere peers, even though it
maintains a single routing table internally.

#### Hierarchical scoping

The propagation rules make a gateway a membrane between its Local
sphere and the Internet sphere, dividing nodes into two tiers by
the routing state they hold:

* A **leaf node** (no INTERNET transport) holds only its Local
  sphere which are the users, nodes, and gateways of its own village, and
  their manifests. It holds nothing about foreign villages.
* A **gateway** holds its Local sphere *and* the Internet sphere:
  every other gateway and every gateway's manifest. The union of
  all gateway manifests is the global directory of which user is
  delegated to which gateway.

Cross-sphere reachability is preserved without a leaf node ever
holding foreign state. A leaf node that cannot resolve a recipient
within its own Local sphere forwards the packet toward its nearest
gateway (Section 9.2); the gateway, holding the global directory,
resolves the recipient and forwards across the Internet sphere.
This keeps a leaf node's routing state bounded by its own village
regardless of total network size.

A node SHALL act as a gateway whenever it has an active INTERNET
transport connection. The gateway role takes effect immediately on
Internet connectivity and SHALL be relinquished when no INTERNET
transport remains active. While acting as a gateway, a node:

1. Composes a manifest of self-delegated and cross-delegated users.
2. Announces itself in its Local sphere as a node entry rather than as
   a user entry.
3. Propagates the manifest in both spheres.

When every node in a network is a gateway, the sphere model provides no
compression and the protocol behaves as a flat distance-vector protocol
with no penalty. The compression benefit applies in the typical case
where gateways are a minority of nodes.

### 2.4. Delegation by Default

Every user is associated with a *delegation target*, that is, the node
that carries the user in its manifest and thereby makes the user
reachable across the Internet sphere. By default, every user has
one delegation target on first login. Two cases:

* **When the user's host is a gateway** (the host has an active
  INTERNET transport): the host itself is the delegation target.
  The user signs a self-delegation to the host. The host's own
  manifest carries the user; the user is reachable globally via
  the host's gateway role. This case requires no wire
  communication, the delegation entry is generated locally and
  flows out through the host's own manifest propagation.

* **When the user's host is not a gateway:** the user selects a
  local-reachable gateway (a node with `local_only = 1` in the
  user's routing-table view and an active manifest) and delegates
  to it (cross-host delegation, Section 10.3). The user transmits
  the signed delegation entry to the chosen gateway via the
  network management sub-protocol (Section 11); the gateway
  includes the user in its next manifest version. The user is
  reachable globally via that gateway.

Selection mechanics, lifetime, and revocation are specified in
Section 10. The user-side state machine monitors the chosen
target's presence, manifest inclusion, and `local_only` flag, and
re-delegates if any of these signals change.

A user MAY opt out of default delegation. An opt-out user is
reachable only from inside their Local sphere; they do not appear
in any manifest and are invisible to the Internet sphere. This is
a privacy affordance for sensitive deployments.

If no local-reachable gateway is available, a non-gateway-hosted
user is local-only-reachable for the same reason an opt-out user
is: no delegation target means no manifest entry means no
cross-sphere visibility. This is expected and acceptable if no
gateway is present in the local sphere, the local sphere is not
connected to the Internet anyway.

---

## 3. Identifiers

### 3.1. Nodes and Users

Nodes and users are distinct concepts. A node is a device; a user is an
identity hosted on a node. The relationship is many-to-one: one node MAY
host many users, but each user has exactly one host node at any given
time. The routing protocol provides reachability for both.

### 3.2. Routing Targets

A routing entry references a target. The target is either a user or a
node. The choice is determined by the host's situation:

* A node hosting exactly one user, with no INTERNET transport active
  and no other users delegated to it, propagates that user as a
  **user entry**.
* A node that hosts more than one user, OR that has an active INTERNET
  transport, OR that holds delegations from any other user, propagates
  itself as a **node entry**, accompanied by a manifest (Section 10)
  listing the users it represents.

Both kinds of entry use the same wire shape (Section 8). The kind is
distinguished by which section of a routing message the entry appears
in (Section 8.3); routing messages carry user entries and node entries
in separate sections, with separate index spaces.

### 3.3. Keys and Identifiers

This protocol distinguishes three layers of identity:

1. **Public key.** A node or user's cryptographic public key, encoded
   in the libp2p multikey format. The multikey carries a small header
   that names the key algorithm, allowing future migration to
   post-quantum algorithms with longer key material without changing
   the rest of the protocol.

2. **Identifier (ID).** An 8-byte hash of the multikey-encoded public
   key. The ID is the canonical wire-level reference to a node or user.
   It has a fixed length regardless of the underlying key algorithm.

3. **Index.** A 16-bit local handle (Section 3.5) that further
   compresses references inside routing messages by replacing the
   8-byte ID with 2 bytes once a binding has been introduced.

The ID is used for routing, lookup, and addressing. It MUST NOT be
used as the sole input to a security decision. At 8 bytes, an
adversary can construct a colliding key with approximately 2^32 work,
which is well below cryptographic acceptability for trust anchoring.
Authentication of any signed artefact (manifest entries, manifest
chunks) SHALL use the full multikey-encoded public key resolved
through the user profile (Section 3.4).


### 3.4. User Profiles

A user profile is a small signed artefact that binds a user's ID to
the user's full public key, along with auxiliary information. Each
profile carries a `profile_version` counter (Section 8) incremented
whenever the profile content changes.

The profile is the authoritative source of a user's public key. A
node that needs to verify a signature by user U MUST first obtain
U's profile and use the public key from it.

Each node similarly publishes a node profile binding its ID to its
public key. Node profiles are required for verifying manifest
signatures (Section 10.1).

The on-wire format of a profile and the operation that fetches it
are defined in Section 11.5 (the network management sub-protocol).
Each routing-update introduction (Section 8.3) carries the latest
known `profile_version` for the introduced ID, so receivers can
detect when their cached profile is stale and issue a profile
fetch.

### 3.5. Indexes

A routing entry references its target by a 16-bit local index rather
than by the target's 8-byte ID. Indexes compress the wire format and
are reused across many updates after a single introduction.

A node maintains **two independent index spaces**, one for users and
one for nodes. Within each space, an index is a 16-bit value in the
range 0 through 65,535. The kind of a given index is determined by
the section of a routing message in which it appears (Section 8.3),
not by any flag bit within the index value.

Index assignment is **node-global**. A node uses the same index for
a given target when communicating with all its neighbours. Indexes
are not per-link.

### 3.6. Index Dictionary

Each node maintains two dictionaries—one per index space—mapping
each assigned index to the corresponding 8-byte ID. Receivers
maintain mirror dictionaries, one per index space per neighbour they
have learned indexes from.

Two mechanisms populate a receiver's view of a sender's dictionaries:

1. **Full dump on neighbour connect.** On every neighbour (re)connect
   over a LAN or INTERNET transport, the sender SHALL transmit an
   `INDEX_DUMP` message (Section 8.4) listing every index it currently
   uses and the corresponding 8-byte ID. The receiver SHALL replace
   any cached mappings for that sender's index spaces with the
   contents of the dump.

2. **Inline introduction on first reference or rebinding.** The
   sender SHALL introduce a binding inline in the routing message
   (Section 8.3) in either of two situations: (a) the receiver
   does not yet have a binding for the index, or (b) the sender
   has rebound the index to a different target since it last
   referenced the index (see Section 3.8). The introduction
   carries the 8-byte ID and the originator's current
   `profile_version` (for users) or `manifest_version` (for
   nodes). The receiver SHALL replace any existing binding for
   the index with the contents of the introduction. After the
   introduction, subsequent references in the same and following
   messages use the index alone.

Over BLE transports, `INDEX_DUMP` is not transmitted; mappings rely
exclusively on inline introductions. This is a bandwidth concession
to BLE's constrained data rate.

A receiver that encounters an index for which it holds no mapping
SHALL treat the entry as a transient error: drop the entry, log if
appropriate, and proceed. Such conditions resolve once the next
inline introduction or `INDEX_DUMP` arrives.

#### Index translation on relay

An index identifies a target only within the dictionary of the
node that assigned it. Node A's index for a target and node B's
index for the same target are independent values; there is no
mesh-wide index namespace. The 8-byte ID (Section 3.3) is the
stable identifier, constant across the mesh; an index is each
node's private shorthand for it.

A node therefore holds two distinct kinds of dictionary:

* Its **own dictionaries** (one per index space): the index → ID
  bindings the node itself has assigned. These index values
  appear in the node's own outgoing messages and serve as the
  slot numbers of its routing tables (Section 9.1).
* A **mirror dictionary** per neighbour (one per index space per
  neighbour): the index → ID bindings the node has learned
  *from* that neighbour, used to interpret routing messages
  arriving from it. The neighbour a message arrived from is
  identified by the transport (Section 8.2), which selects which
  mirror to consult.

When a node relays a routing entry, it re-indexes:

1. Resolve the incoming entry's index through the mirror
   dictionary for the neighbour it arrived from, obtaining the
   8-byte ID.
2. Look up that ID in the node's own dictionary. If the node has
   not yet assigned an index to the ID, allocate one
   (Section 3.8).
3. Emit the relayed entry referencing the node's *own* index,
   introducing the own-index → ID binding to downstream
   neighbours (per the two mechanisms above) the first time that
   index is used.

The incoming index is never forwarded unchanged. A target
reachable through a chain of nodes is referenced by a different
index at each hop, every one of them bound to the same 8-byte ID.

### 3.7. Index Lifecycle

When an entry's last update timestamp exceeds the route expiry timeout
(Section 7.5), the entry is removed and its index slot enters a
**cooldown** state. During cooldown, the slot remains empty and SHALL
NOT be allocated to any other target.

Cooldown duration is **60 seconds**: 35 seconds for the route expiry
window, plus 25 seconds for in-flight propagation margin. After the
cooldown expires, the slot becomes eligible for new allocation.

The cooldown rule prevents stale references on the wire from binding
to a freshly-allocated target. A receiver that holds a stale index
mapping during the cooldown period sees no entries and no introductions
for the slot; the mapping ages out via normal expiry.

### 3.8. Index Allocation

A node maintains an independent allocator for each of the two index
spaces. Within each space, allocation uses a monotonic cursor over the
16-bit index range, wrapping from 65,535 back to zero.

To allocate an index, the allocator advances the cursor and SHALL
skip any position whose routing-table slot is currently occupied or
whose slot is in cooldown (Section 3.7). The first eligible position
is claimed; the resulting index is returned and the binding recorded
in the dictionary (Section 3.6).

If a full sweep of the 16-bit space yields no eligible position, the
allocator SHALL refuse the allocation and the routing layer SHALL
drop the entry whose propagation requested it. This condition is
logged and is not a protocol error. With a design target of tens of
thousands of visible targets against a per-space capacity of 65,536,
exhaustion is not expected to occur in practice.

Allocator state is not persisted across restarts. On startup, every
slot is initially free and the cursor begins at zero. The
INDEX_DUMP-on-reconnect rule (Section 3.6) ensures that any cached
mappings from a previous session are replaced before routing updates
from the new session are processed.

An index is marked as **needing re-introduction** in three
situations:

1. The index has just been allocated for the first time (a target
   the sender has not yet referenced over the wire).
2. The allocator has returned an index that has previously been
   through cooldown, that is, the slot was bound to a different
   target earlier in the current session.
3. The originator's `profile_version` or `manifest_version` for
   the target bound to the index has incremented since the last
   inline mapping was sent for that index.

When an index is marked, the next outgoing `ROUTING_UPDATE` that
references the index SHALL include an inline mapping for it in
the appropriate `n_user_mappings` or `n_node_mappings` section
(Section 8.3). Once the introduction has been sent, the flag is
cleared. The same mechanism serves all three cases: the receiver
replaces any existing binding for the index with the contents of
the mapping (Section 3.6), so a first-time introduction, a
rebinding to a new target, and a version refresh are handled
identically on the wire and at the receiver.

The index value `0x0000` is reserved within each space for the
node's self-reference and SHALL NOT be returned by the allocator:

* In the **user index space**, `0x0000` is bound to the hosted user
  when the node propagates as a user entry.
* In the **node index space**, `0x0000` is bound to the node itself
  when the node propagates as a node entry.

The two reservations do not collide because they live in separate
index spaces. When the node's propagation form changes
(Section 3.2), the previously used reserved index is released and
enters cooldown like any other freed slot. The newly used reserved
index is bound and put into use in the next 1-second relay batch.

A node hosting more than one user assigns additional user indexes
through the allocator for users other than the one bound to
`0x0000` in the user index space.

---

## 4. Transports

### 4.1. Defined Transports

The following transports are defined.

* **LAN.** Local area network connectivity over libp2p TCP, typically
  with mDNS discovery. Local sphere.
* **INTERNET.** Wide-area connectivity over libp2p TCP or QUIC,
  typically with a configured peer list. Internet sphere.
* **BLE 1M.** Bluetooth Low Energy on the 1 Mbps physical layer.
  Local sphere.
* **BLE Coded.** Bluetooth Low Energy on the Coded physical layer
  (long range, lower data rate). Local sphere.
* **Local.** Loopback for users hosted on the receiving node itself.
  Local sphere; trivial.

Future transports MAY be added. New transport modules SHALL declare
their sphere as part of the module definition.

### 4.2. Multi-Transport Neighbours

A node MAY connect to the same neighbour over multiple transports
simultaneously. The neighbour table (Section 4.3) tracks which
transports a given neighbour is reachable on. A node MAY receive
routing-update entries for the same target via multiple
`(neighbour, transport)` pairs; the relay-inclusion rule
(Section 7.2) determines which candidate is accepted and stored
in the routing table. The routing table itself holds at most one
entry per target (Section 9.1); the per-`(neighbour, transport)`
information surfaces only during relay-inclusion comparison.

### 4.3. Discovery and Loss

Neighbour discovery and neighbour-loss detection are properties of the
transport layer and are not specified by this protocol. Each transport
module is responsible for maintaining the list of currently reachable
neighbours and for notifying the routing layer when a neighbour becomes
unreachable. The routing layer consumes the neighbour list and expires
routing entries via the standard route expiry timeout when a transport
withdraws a neighbour.

---

## 5. Routing Metric

### 5.1. Per-Hop Cost

The path metric for a routing entry is the sum of per-hop costs along
the path. Per-hop cost is computed by the receiving node when an entry
arrives:

```
hop_cost = transport_weight(T) + quality_penalty(T, Q)
```

where `T` is the transport over which the entry was received and `Q`
is the link quality at the receiver.

Transport weights:

| Transport  | Weight |
|------------|--------|
| LAN        | 10     |
| INTERNET   | 15     |
| BLE 1M     | 50     |
| BLE Coded  | 70     |

Quality penalty is zero on LAN and INTERNET in this version of the
protocol. On BLE transports, quality penalty derives from the radio's
Received Signal Strength Indication (RSSI), bucketed:

| RSSI               | Penalty |
|--------------------|---------|
| -60 dBm or greater | 0       |
| -75 to -60 dBm     | 5       |
| -85 to -75 dBm     | 10      |
| Below -85 dBm      | 20      |

These bucket boundaries and penalty values are subject to refinement
based on deployment measurements.

### 5.2. Computation Model

The origin of a routing entry sets the metric to zero. Each receiver
adds its own per-hop cost to the metric before storing or relaying the
entry. All metric manipulation occurs at receive time.

This model has two consequences. First, a sender does not need to know
the path that an outgoing entry will eventually take. Second, the
receiver's measurement of link quality is fresher than any sender-side
measurement could be, because it is the receiver that just observed the
link.

### 5.3. Encoding

The metric is a 16-bit unsigned integer, both in routing messages
and in the receiver's internal representation. Any conforming metric
formula MUST produce values that fit in 16 bits across the longest
legal path (63 hops, per Section 2.2). The design rule is
`63 × worst_hop_cost ≤ 65,535`.

The current formula (Section 5.1) has a worst-case path metric of
63 × (70 + 20) = 5,670, well under the 16-bit cap.

Implementations SHALL saturate accumulation at `u16::MAX` if a
metric formula would otherwise overflow. The saturated value
remains comparable but loses precision at the edge; the design
constraint above is the primary safeguard against overflow.

### 5.4. Tie-Breaking

When two routes to the same destination have identical metrics, a
node SHALL prefer the currently-selected route. If no route is
currently selected (this is the first arrival), the node SHALL prefer
the first-arrived route.

### 5.5. Metric Evolution

A change to the metric formula does not require a wire-protocol
version bump. The wire format carries the accumulated metric value,
not the formula. Two nodes in the same deployment running different
formulas produce accumulated values that no longer reflect any
single formula; routing remains loop-free and monotonically
non-decreasing within each receiver's view, but path quality may be
transiently sub-optimal during a rolling change.

This trades operational simplicity (drop in a new formula and let
the mesh re-converge) for transient quality degradation.

---

## 6. Sequence Numbers

### 6.1. Specification

A node maintains a single sequence number that applies to all routing
entries originated by that node. The sequence number is a 16-bit
unsigned integer.

On startup, a node SHALL initialize its sequence number to a uniformly
random value in the range [0, 65535].

The sequence number is incremented by one at each origin update cycle
(every ten seconds, see Section 7.1). Wraparound occurs after
approximately 7.5 days of continuous operation; comparison
(Section 6.2) handles wraparound correctly.

### 6.2. Comparison

Sequence numbers are compared using unsigned circular arithmetic. A
sequence number `new` is fresher than `old` if and only if:

```
(new - old) mod 65536 < 32768
```

This rule treats the 16-bit space as a circle and considers `new` to
be the more recent value when the forward distance from `old` to `new`
is less than half of the space.

### 6.3. Reboot Detection

A rebooted node restarts at a fresh random sequence number, which MAY
fall on either side of the value its peers last observed. Without
correction, circular comparison alone could reject the rebooted node's
new sequence number as stale.

A receiver SHALL apply the following rule:

> If the forward circular distance `(new - old) mod 65536` exceeds
> 100, the receiver SHALL accept `new` as a reboot indicator and
> replace its stored sequence number with `new` unconditionally.

A forward distance of 100 corresponds to approximately sixteen minutes
of normal operation at the ten-second origin interval. False positives
(a node legitimately offline for more than sixteen minutes whose
sequence has advanced past the threshold) are benign: accepting the
new sequence as if it were a reboot produces the same end state as
normal acceptance.

---

## 7. Routing Updates

### 7.1. Propagation

Propagation has two phases:

**Origin (every ten seconds).** A node increments its sequence number
and sends an update describing itself: a single entry with metric zero,
hop count zero, and the new sequence number. The entry is a user entry
or node entry per Section 3.2.

**Relay (every one second).** A node collects all routing entries
received over the past second that satisfy the relay inclusion rule
(Section 7.2), batches them into a single message, and sends the batch
to all direct neighbours.

Per-hop propagation delay averages 0.5 seconds, since an update arrives
at a node at a uniformly distributed point in the receiver's relay
window. End-to-end propagation across N hops therefore averages 0.5 × N
seconds for the initial pipeline fill.

### 7.2. Relay Inclusion

A node SHALL relay a received entry under either of the following
conditions:

1. The entry's sequence number is fresher than the stored sequence
   number for that target (per Section 6.2).
2. The entry's sequence number equals the stored sequence number AND
   the entry's metric is strictly less than the stored metric.

In all other cases (older sequence, equal sequence with equal or worse
metric), the entry SHALL be dropped without storage and without relay.

This rule allows a better path to overtake a worse path within a single
sequence cycle, which matters in heterogeneous-transport meshes where
the first-arriving copy of an update is not always the lowest-cost
path. Within any given sequence cycle, metric is monotonically
non-increasing, so the rule cannot oscillate and cannot loop.

### 7.3. Split Horizon

A node SHALL NOT relay a routing entry back to the neighbour that
would be its next hop for that target. This is plain split horizon
without poisoned reverse: no special "infinity" advertisement is sent;
the entry simply is not transmitted on the reverse direction.

The relay inclusion rule (Section 7.2) prevents forwarding loops on
its own; split horizon is a bandwidth optimisation that avoids
predictably rejected back-relays.

### 7.4. TTL Enforcement and Local-Only Flag

Each routing entry carries a hop count in the lower six bits of a
one-byte field. Bit 7 is the `local_only` flag; bit 6 is reserved
(Section 2.2).

The receiver SHALL mask off bit 6 before any hop-count comparison
and SHALL increment the hop count by one along with the metric
(Section 5.2). After incrementing, an entry whose hop count exceeds
63 SHALL be dropped.

#### Local-only flag propagation

The `local_only` flag tracks whether the entry has traveled
exclusively over Local-sphere transports along its propagation path
so far. The rule has three parts:

1. **Origin.** When a node originates its own routing entry (the
   self-hello sent every 10 s), it sets `local_only = 1` if the
   outgoing message is being sent over a Local-sphere transport
   (LAN, BLE 1M, BLE Coded). If the outgoing message is being
   sent over an Internet-sphere transport (INTERNET), it sets
   `local_only = 0`.

2. **Relayer.** When a node relays a stored entry to a neighbour,
   it consults the entry's stored `local_only` value:
   - If the outgoing transport is Local-sphere: the relayer sends
     the stored flag value unchanged.
   - If the outgoing transport is Internet-sphere: the relayer
     sends `local_only = 0` regardless of the stored value.

3. **Monotonicity.** Once an entry is stored with
   `local_only = 0` at a receiver, the flag never reverts to 1
   for that entry. Subsequent updates carrying `local_only = 1`
   only apply if they arrive over a Local-sphere transport and
   the relay-inclusion rule (Section 7.2) accepts them; under
   normal flow a node that has already stored a 0-flagged entry
   for the target will not receive a 1-flagged update for the
   same target on the same transport without the entry first
   expiring (Section 7.5) and a fresh route being learned.

A node's own self-entry has `local_only = 1` in its routing-table
representation; the value is applied per outgoing transport per
the origin rule above.

### 7.5. Route Expiry

A route is considered expired when no update with a current or fresher
sequence number has been received for **35 seconds**. An expired route
is removed from the routing table; its index slot enters cooldown
(Section 3.7).

The 35-second value covers the 99th-percentile inter-arrival gap at the
diameter (~22 seconds), plus one missed origin cycle (10 seconds), plus
margin. If real-world deployment shows route flapping at the diameter
edge, this value MAY be increased to 45-50 seconds.

---

## 8. Wire Format

### 8.1. Conventions

All multi-byte fields are big-endian (network byte order). Identities
are referenced on the wire by their 8-byte ID (Section 3.3); full
public keys are not carried in routing messages and are obtained
through user and node profiles (Section 3.4).

Four message types are defined:

| Type byte | Name             | Frequency                                                                |
|-----------|------------------|--------------------------------------------------------------------------|
| 0x01      | ROUTING_UPDATE   | Every 1 second (relay) or origin                                         |
| 0x02      | INDEX_DUMP       | On neighbour connect                                                     |
| 0x03      | NODE_MANIFEST    | Full manifest; on bootstrap, on transition, or as periodic re-sync       |
| 0x04      | MANIFEST_DELTA   | Incremental manifest update; default emission for changes, rate-limited  |

### 8.2. Common Header

Every message begins with a four-byte header:

```
  0                   1                   2                   3
  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |    version    |     type      |          payload_len          |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

* `version` (1 byte) is set to 0x01 for this version of the protocol.
  A receiver that encounters a version it does not understand SHALL
  skip `payload_len` bytes and continue with the next message.
* `type` (1 byte) selects the message kind from the table above.
* `payload_len` (2 bytes) is the length in bytes of the message
  payload following this header. Maximum payload size is 65,535 bytes;
  messages that would exceed this size are split across multiple
  messages with chunking semantics defined per type.

The sender's identity is conveyed by the transport layer
(Section 4): every received message is associated with the
`(peer, transport)` pair it arrived on. The receiver resolves this
to its own node-space index for the sender via its neighbour
table.

### 8.3. ROUTING_UPDATE

The `ROUTING_UPDATE` message carries inline index introductions and
routing entries. Both are organised into four sections; user
mappings, node mappings, user entries, node entries with each addressing
its own index space (Section 3.5). Within each section, indexes are
encoded as one-byte deltas relative to the previous entry's absolute
index, with an explicit escape for larger jumps.

```
  Common header (4 bytes)

  +-+-+-+-+-+-+-+-+
  | n_user_mappings |                                (1 byte)
  +-+-+-+-+-+-+-+-+
  For each user mapping (variable):
      [1]  delta             (0x01-0xFF, or 0x00 escape; see below)
      [0|2] absolute_index   (present iff delta == 0x00)
      [8]  target_id
      [4]  profile_version

  +-+-+-+-+-+-+-+-+
  | n_node_mappings |                                (1 byte)
  +-+-+-+-+-+-+-+-+
  For each node mapping (variable):
      [1]  delta
      [0|2] absolute_index
      [8]  target_id
      [4]  manifest_version

  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |       n_user_entries          |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  For each user entry (variable):
      [1]  delta
      [0|2] absolute_index
      [2]  seq_num
      [2]  metric
      [1]  hop_count

  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |       n_node_entries          |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  For each node entry (variable):
      [1]  delta
      [0|2] absolute_index
      [2]  seq_num
      [2]  metric
      [1]  hop_count
```

#### Delta encoding

Within each section, the sender SHALL transmit entries in ascending
order of absolute index. Each entry begins with a one-byte `delta`
giving the difference between the current entry's index and the
previous entry's index in the same section.

* Within a section, the first entry's "previous index" is treated as
  zero; the delta value equals the first entry's absolute index.
* A `delta` value of `0x01` through `0xFF` SHALL be added to the
  running cursor to obtain the current absolute index; no
  `absolute_index` field follows.
* A `delta` value of `0x00` is the **escape**: the next two bytes
  carry the current entry's absolute index directly, and the running
  cursor is set to that value.

The escape covers gaps wider than 255. After processing each entry
(escape or not), the receiver updates the cursor to the resolved
absolute index for use with the next entry in the same section.

#### Field semantics

`target_id` is the 8-byte hash of the target's multikey-encoded
public key (Section 3.3). It is included only on first introduction
of an index; subsequent references in the entries sections omit it
and rely on the dictionary binding.

`profile_version` and `manifest_version` are 32-bit unsigned
counters maintained by the originating user or host. They are
compared using circular arithmetic scaled to 32 bits; a fresher
value indicates the receiver's cached profile or manifest is
stale. A stale profile SHOULD be refreshed through the
profile-fetch operation of the network management sub-protocol
(Section 11.5). A stale manifest is refreshed by the scoped-push
manifest propagation (Section 8.5).

`seq_num` is the origin's current sequence number for the entry.
`metric` is the path cost as accumulated by senders up to (and not
including) the current receiver. `hop_count` is a one-byte field
whose lower six bits hold the path length, bit 7 holds the
`local_only` flag (Section 7.4), and bit 6 is reserved
(Section 2.2). The path length is accumulated up to but not
including the receiver; the receiver applies its own per-hop cost
(Section 5.1) and increments the hop count on receipt. Bit 6 MUST
be set to zero by senders in this version of the protocol and
SHALL be masked off by receivers before any hop-count comparison.
The `local_only` flag is set by the sender per the rule in
Section 7.4 and stored by the receiver as received.

Routing entries do not carry a version field. Profile and manifest
versions travel only with inline mappings (this section's
`n_user_mappings` / `n_node_mappings`) and with `INDEX_DUMP`. When
the originator's `profile_version` or `manifest_version` for one
of its assigned indexes increments, the originator's allocator
marks the index as needing re-introduction (Section 3.8) and the
next outgoing `ROUTING_UPDATE` includes a fresh inline mapping
carrying the new version. Between version changes, receivers
treat their cached version as current.

The maximum number of mappings per kind is 255 (limited by the
1-byte count). The maximum number of entries per kind is 65,535;
in practice messages are far smaller and bounded by the overall
64 KiB payload cap.

### 8.4. INDEX_DUMP

The `INDEX_DUMP` message lists every index a sender currently has
allocated, along with the bound 8-byte ID and current version. It is
organised into a user section and a node section, mirroring the two
index spaces (Section 3.5). It is sent on every neighbour (re)connect
over LAN or INTERNET transports. It is not sent over BLE transports.

```
  Common header (4 bytes)

  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |       n_user_mappings         |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  For each user mapping (variable):
      [1]  delta             (0x01-0xFF, or 0x00 escape; see §8.3)
      [0|2] absolute_index   (present iff delta == 0x00)
      [8]  target_id
      [4]  profile_version

  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |       n_node_mappings         |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  For each node mapping (variable):
      [1]  delta
      [0|2] absolute_index
      [8]  target_id
      [4]  manifest_version
```

`INDEX_DUMP` uses the same delta encoding as ROUTING_UPDATE
mappings (Section 8.3): within each section, the sender SHALL
transmit mappings in ascending order of absolute index; each
mapping's `delta` field is the difference from the previous
mapping's absolute index in the same section;the first mapping
treats the previous index as zero; the escape value `0x00` is
followed by a 2-byte absolute index. The receiver processes
mappings sequentially within each section.

If a sender's full dictionary would exceed the 64 KiB payload
limit, the sender splits it across multiple `INDEX_DUMP` messages.
Each chunk independently resets the delta cursor to zero and
independently carries both sections, with section counts and
deltas reflecting only the contents of that chunk. Ordering
between chunks is not significant; ordering within a chunk's
section is fixed by the delta encoding.

### 8.5. NODE_MANIFEST

The `NODE_MANIFEST` message carries a node's manifest. It is sent by
every multi-user host and every gateway. Single-user, non-gateway hosts
do not send this message.

```
  Common header (4 bytes)

  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |       origin_node_index       |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |        manifest_version       |                  (4 bytes,
  |               ...             |                   wraps over)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |  chunk_index  |  chunk_count  |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |     flags     |                                  (1 byte)
  +-+-+-+-+-+-+-+-+
  |                                                                |
  |               manifest_signature (64 bytes)                    |
  |                                                                |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |           n_entries           |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  For each entry (80 bytes):
      [ 8] user_id              (8-byte hash, Section 3.3)
      [ 8] timeout
      [64] entry_signature
```

* `origin_node_index` is the index in the node index space of the
  host the manifest belongs to.
* `manifest_version` is a 32-bit counter incremented on any change to
  the entry list. Comparison uses circular arithmetic (Section 6.2)
  scaled to 32 bits.
* `chunk_index` and `chunk_count` permit a manifest to be split across
  multiple messages when its body would exceed approximately 60 KiB.
  `chunk_index` is in the range 0 through `chunk_count - 1`, and
  `chunk_count` is in the range 1 through 256.
* `flags` is a one-byte field carrying manifest-level flags. Bit 0
  is `is_gateway` which is set when the host has at least one active
  INTERNET-sphere transport connection at manifest emission time
  (Section 10.1). Bits 1-7 are reserved, MUST be set to zero by
  senders in this version of the protocol, and SHALL be ignored by
  receivers. The `flags` field carries the same value across all
  chunks of a single `manifest_version`; a receiver MAY read it
  from any chunk.
* `manifest_signature` is the host's ed25519 signature over the
  concatenation of `(origin_node_id || manifest_version ||
  chunk_index || chunk_count || flags ||
  canonical_entries_in_this_chunk)`.
  The `origin_node_id` here is the host's full multikey-encoded
  public key resolved through the node profile (Section 3.4), not
  the 8-byte ID.
* For each entry: `user_id` is the delegating user's 8-byte ID;
  `timeout` is the absolute expiry of the delegation in milliseconds
  since the Unix epoch; `entry_signature` is the user's ed25519
  signature over `(origin_node_id || timeout)`. The
  `origin_node_id` in the signed content is the host's full
  multikey-encoded public key (resolved through the node profile,
  Section 3.4), not the 8-byte ID, this binds the signature to
  the host's full cryptographic identity, defeating
  impersonation attempts based on 8-byte ID collisions. The
  signature is verified against the user's full public key
  resolved through the user profile (Section 3.4).

A manifest carrying N entries requires approximately `79 + 80·N`
bytes per chunk (4 header + 2 + 4 + 2 + 1 `flags` + 64 signature +
2 `n_entries` = 79 bytes of overhead).

A NODE_MANIFEST message is sent on:

1. Neighbour (re)connect, as part of bootstrap. The new neighbour
   receives the host's current full state immediately.
2. A transition between single-user (no manifest) and multi-user
   (manifest required) state. In this case, the new manifest SHALL
   be sent in the next 1-second relay batch and SHALL NOT wait for
   the next 10-second origin cycle.
3. Periodic full re-sync, as required by Section 10.8: at least every
   ten manifest emissions or every hour, whichever first.

Incremental changes between full emissions are propagated as
MANIFEST_DELTA messages (Section 8.6).

A receiver SHALL relay a NODE_MANIFEST whose `manifest_version` is
strictly fresher than its stored version for the same
`origin_node_index`. A node SHALL NOT relay a manifest onto a
Local-sphere transport if it received that manifest over an
Internet-sphere transport. Combined with the routing-entry
propagation rule (Section 2.3), this confines a foreign manifest
to the Internet sphere; it never enters a foreign Local sphere.

### 8.6. MANIFEST_DELTA

The `MANIFEST_DELTA` message carries an incremental update to a
manifest: a list of entries to add and a list of user IDs to remove.
A receiver applies the delta to its stored manifest at `from_version`
to produce the manifest at `to_version`, then verifies the host's
signature against the resulting state.

```
  Common header (4 bytes)

  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |       origin_node_index       |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |         from_version          |                  (4 bytes)
  |               ...             |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |          to_version           |                  (4 bytes)
  |               ...             |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |     flags     |                                  (1 byte)
  +-+-+-+-+-+-+-+-+
  |                                                                |
  |               manifest_signature (64 bytes)                    |
  |                                                                |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |            n_adds             |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  For each add (80 bytes):
      [ 8] user_id              (8-byte hash, Section 3.3)
      [ 8] timeout
      [64] entry_signature
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |           n_removes           |                  (2 bytes)
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  For each remove (8 bytes):
      [ 8] user_id
```

* `origin_node_index` is the index in the node index space of the
  host the manifest belongs to.
* `from_version` is the `manifest_version` the delta builds upon. The
  receiver MUST hold the manifest at exactly this version; otherwise
  the delta is not applicable.
* `to_version` is the resulting `manifest_version` after applying the
  delta. `to_version` SHALL be strictly fresher than `from_version`
  by circular comparison.
* `flags` is the one-byte manifest-flags field (Section 8.5), carrying
  the `is_gateway` bit and reserved bits. It holds the manifest's
  flag state at `to_version`. A delta MAY carry a changed `flags`
  value relative to `from_version` — this is how a change to
  `is_gateway` propagates incrementally.
* `manifest_signature` is the host's ed25519 signature over the
  canonical encoding of the *resulting* full entry set at `to_version`,
  using the same construction as in Section 8.5 (`origin_node_id ||
  manifest_version || chunk_index || chunk_count || flags ||
  canonical_entries`). The receiver verifies against the state
  produced by applying the delta, not against the delta itself.
* Adds carry full entries (80 bytes each), including the per-entry
  user signature, identical in shape to entries in NODE_MANIFEST.
* Removes carry only the `user_id` (8 bytes each); the receiver
  removes the matching entry from its stored state.

Per-message overhead before adds and removes is **83 bytes**
(4 header + 2 + 4 + 4 + 1 `flags` + 64 signature + 2 + 2). A delta
carrying one add totals 163 bytes; one remove totals 91 bytes; one
add and one remove together totals 171 bytes.

A delta SHALL fit in one message. Chunking is not defined for deltas.
If a delta would exceed the 60 KiB body bound (approximately 740 adds,
or 7,500 removes), the gateway SHALL emit a full NODE_MANIFEST
instead.

A receiver whose stored version for this host is not equal to
`from_version` SHALL drop the delta. Re-convergence occurs when the
host next emits a full NODE_MANIFEST (Section 10.8 bounds the wait)
or when the receiver triggers a fresh bootstrap on neighbour
reconnect.

A receiver SHALL relay a MANIFEST_DELTA whose `to_version` is
strictly fresher than its stored version, in the next 1-second relay
batch. Split horizon (Section 7.3) applies. As with NODE_MANIFEST
(Section 8.5), a node SHALL NOT relay a delta onto a Local-sphere
transport if it received that delta over an Internet-sphere
transport.

### 8.7. Receiver Processing

On receipt of any routing protocol message, a receiver SHALL:

1. Read the common header.
2. If `version` is unrecognised, skip `payload_len` bytes and discard.
3. Dispatch on `type`.

For ROUTING_UPDATE (0x01):

1. Process each of the four sections in order: user mappings, node
   mappings, user entries, node entries (Section 8.3). Each section
   maintains its own delta cursor starting at zero.
2. For each mapping, decode the delta (or escape), update the cursor
   to the resulting absolute index, and store the binding
   `(target_index -> target_id)` in the corresponding kind's
   dictionary along with the introduced `profile_version` or
   `manifest_version`, **replacing any existing binding for the
   same index**. If the previous binding was for a different
   `target_id`, the receiver SHALL also clear the routing-table
   slot at that index (release the entry, drop its back-reference
   on the previously-bound `User` or `Node` record); this handles
   the case where the sender has rebound the index to a new target
   after cooldown (Section 3.8). If the cached version for the new
   ID is older, schedule a profile or manifest fetch.
3. For each entry, decode the delta (or escape), update the cursor,
   and:
    a. Look up the resolved absolute `target_index` in the
       corresponding kind's dictionary. If absent, drop the entry
       and log.
    b. Extract the `local_only` flag from bit 7 of `hop_count`
       and retain it for storage. Mask off bit 6 (reserved,
       Section 2.2). Add the receiver's per-hop cost to `metric`.
       Increment the 6-bit hop count by one.
    c. If the resulting hop count exceeds 63, drop the entry.
    d. Apply the relay inclusion rule (Section 7.2). If accepted,
       store the entry (including the extracted `local_only`
       flag) and queue it for the next 1-second relay batch.

For INDEX_DUMP (0x02):

1. Read both sections (`n_user_mappings`, `n_node_mappings`). Each
   mapping has an absolute `target_index`. Store each binding in
   the corresponding kind's dictionary, replacing any existing
   binding for the same index. Compare the introduced version
   against any cached profile or manifest version and schedule a
   fetch if newer.
2. If further INDEX_DUMP chunks are expected, accumulate; ordering
   between mappings and across chunks is not significant.

For NODE_MANIFEST (0x03):

1. Look up `origin_node_index` to obtain the host's public key.
2. Verify `manifest_signature` against the host's public key. If
   verification fails, drop the message.
3. Compare `manifest_version` to the stored version for this host. If
   not strictly fresher, drop the message.
4. For each entry:
    a. Verify `entry_signature` against `user_id`. If verification
       fails, drop the entry but continue processing other entries.
    b. If `timeout` is in the past, drop the entry.
    c. A user MAY appear in multiple hosts' manifests
       simultaneously (Section 10.6). Each manifest is a
       self-contained statement by its host; manifests from
       different hosts are not compared for freshness across
       hosts. Add the entry to this host's delegation set
       without altering the user's other hosts' bindings.
5. Read the `flags` field. Bit 0 (`is_gateway`) is covered by the
   verified `manifest_signature` (step 2); store it on the host's
   `Node` record. Ignore reserved bits 1-7.
6. Store the chunk. When all chunks for a given `manifest_version`
   have arrived, update the host's `Node` record and the affected
   `User` records.
7. Queue the message for relay in the next 1-second batch, subject
   to the sphere-scoping relay rule (Sections 8.5 / 8.6).

For MANIFEST_DELTA (0x04):

1. Look up `origin_node_index` to obtain the host's public key.
2. Compare `from_version` to the receiver's stored version for this
   host. If not equal, drop the message; the receiver will recover on
   the next full NODE_MANIFEST.
3. In a scratch copy of the stored entry set, apply removes (delete
   matching `user_id`s) and apply adds (insert new entries).
4. For each added entry:
    a. Verify `entry_signature` against `user_id`. If verification
       fails, drop the entry from the scratch set and log.
    b. If `timeout` is in the past, drop the entry from the scratch
       set.
    c. A user MAY appear in multiple hosts' manifests
       simultaneously (Section 10.6). Add the entry to this host's
       scratch set without altering the user's other hosts'
       bindings.
5. Compute the canonical encoding of the scratch set and verify
   `manifest_signature` against it (the `flags` field is part of
   the signed content, Section 8.6). If verification fails, discard
   the scratch set without modifying stored state.
6. Commit: replace the stored entry set with the scratch set; set
   stored version to `to_version`; store the `flags` field's
   `is_gateway` bit on the host's `Node` record.
7. Queue the message for relay in the next 1-second batch, subject
   to the sphere-scoping relay rule (Sections 8.5 / 8.6).

---

## 9. Routing Table

### 9.1. Structure

A node maintains two routing tables: one indexed by user-space index,
one by node-space index (Section 3.5). Each routing entry contains:

* `target_index` (2 bytes; absolute, kind determined by which table)
* `target_id` (8 bytes; resolved from the dictionary, Section 3.6)
* `seq_num` (2 bytes)
* `metric` (2 bytes)
* `next_hop` (2 bytes; the neighbour's node index)
* `transport` (1 byte; the transport module to use for `next_hop`)
* `last_update` (8 bytes; receipt timestamp for expiry)
* `hop_count` (1 byte; 6-bit count + 1 reserved bit + the
  `local_only` flag bit, informational)
* `local_only` (1 bit; extracted from `hop_count` bit 7 on
  receipt, stored separately for easy access by route-selection
  logic; semantics in Section 7.4)

A node's own self-entry has `local_only = 1` in its stored form;
the value is then applied per outgoing transport per the origin
rule in Section 7.4.

The originator's `profile_version` (for users) or `manifest_version`
(for nodes) is stored on the corresponding `User` or `Node` record
in `UsersMap` / `NodesMap`, not on the routing entry itself. Each
`Node` record additionally stores an `is_gateway` boolean, taken
from the most recent manifest's `flags` field (Sections 8.5,
10.1); route-selection and cross-host delegation target selection
(Section 10.3) consult it.

Auxiliary structures bind identities to routing state. A node maintains
a `NodesMap` keyed by node public key, and a `UsersMap` keyed by user
public key. Each `User` record carries an optional reference to its
direct routing entry (set when the user is reachable directly) and a
list of references to gateway nodes through which the user is also
reachable (populated from received manifests).

A user is reachable by a recipient if either:

* the user has a direct routing entry, or
* the user appears as a delegated entry in some gateway's manifest,
  and that gateway has a routing entry.

If neither condition holds, the user is reported unreachable to the
calling layer; the delay-tolerant networking layer above this protocol
is responsible for handling such cases.

### 9.2. Route Selection

The packet header carries the recipient's 8-byte ID
(Section 3.3). Each forwarding node, the sender and every
intermediate hop alike, performs its own next-hop lookup
against the recipient ID.

To resolve the next hop for a recipient user, a node:

1. Looks up the recipient in `UsersMap`.
2. If the recipient has a direct routing entry, takes the next hop
   from that entry.
3. Otherwise, examines the recipient's list of delegation
   gateways. Among gateways with current routing entries, selects
   the one with the lowest metric, and takes the next hop from the
   gateway's routing entry.

If neither a direct entry nor any reachable delegation gateway is
available for the recipient, behaviour depends on the node's tier
(Section 2.3):

* A **leaf node** forwards the packet toward its nearest gateway: 
  the local gateway with the lowest routing metric, selected
  exactly as any other routing destination. The gateway holds the
  global directory and resolves the recipient. Any node along the
  way that likewise cannot resolve the recipient forwards toward
  its own nearest gateway; because every node routes along its
  shortest path to a gateway, the packet reaches one in a bounded
  number of hops without looping, and no coordination between
  nodes is required (nearest-gateway anycast).
* A **gateway** already holds the global directory. If the
  recipient is not found there, it is genuinely unreachable; the
  packet is handed to the DTN layer.
* A leaf node with no gateway in its Local sphere hands the packet
  to the DTN layer; the recipient is unreachable from this Local
  sphere.

The full lookup flow, including the handling of weak
back-references between `User`, `RoutingEntry`, and `Node`
records, is shown in Appendix A.4.

Route selection among multiple `(neighbour, transport)` candidates
for the same target happens at receive time, not at lookup time,
via the relay-inclusion rule (Section 7.2). By the time a lookup
occurs, the routing table holds at most one entry per target
(the best one accepted so far); the lookup is a slot read.

---

## 10. Delegation

Delegation is the primary mechanism by which a user becomes
reachable across the Internet sphere. Without a delegation, a
user is reachable only within their Local sphere. With one, a
*delegation target* node carries the user in its manifest, and
that manifest propagates across both spheres, making the user
findable from anywhere in the mesh.

Two delegation modes are defined, distinguished by the
relationship between the delegating user and the target node:

* **Self-delegation** (Section 10.2): the delegating user's host
  node is itself a gateway, and the user delegates to its own
  host. The simplest case; no wire communication required.
* **Cross-host delegation** (Section 10.3): the delegating
  user's host is not a gateway, and the user delegates to a
  different node (a local-reachable gateway). The default for any
  user whose host lacks an active INTERNET transport. Uses the
  network management sub-protocol (Section 11) to convey the
  signed delegation entry from the user to the target node.

A user MAY also decline delegation entirely (Section 2.4), in
which case they are reachable only within their Local sphere.
Opt-out has no delegation mechanics and is therefore specified
only in Section 2.4, alongside the rest of the delegation
decision.

The remainder of this chapter specifies the wire format of
delegation entries and their manifests (§10.1), the two modes in
detail (§10.2, §10.3), lifetime and revocation (§10.4, §10.5),
multi-gateway delegation (§10.6), reachability guarantees a
gateway must maintain (§10.7), and emission rate rules (§10.8).

### 10.1. Manifest Composition

A node's manifest is the signed, versioned list of users currently
delegated to it. Each entry in the manifest comprises:

* `user_id`: the delegating user's 8-byte ID (Section 3.3).
* `timeout`: the absolute expiry of this delegation, in milliseconds
  since the Unix epoch.
* `entry_signature`: an ed25519 signature by the delegating user
  over the concatenation `host_node_id || timeout`, where
  `host_node_id` is the **full multikey-encoded public key** of
  the node holding the delegation (resolved through that node's
  profile, Section 3.4), not the 8-byte ID. Using the full key
  in the signed content binds the signature to the node's
  cryptographic identity, defeating impersonation attempts based
  on 8-byte ID collisions. The signing user's identity is
  determined by the verifying public key (resolved from the
  wire-level `user_id` field through the user profile,
  Section 3.4).

The field name `host_node_id` refers to the node
holding the delegation, regardless of whether that node is the
user's own host node (self-delegation, Section 10.2) or a
different node (cross-host delegation, Section 10.3).

The manifest as a whole carries an additional `manifest_signature` by
the host node over the chunk contents, as detailed in Section 8.5.

The per-entry signature binds a specific user to a specific host and
timeout; replaying it on a different host or with a different timeout
fails verification. The whole-manifest signature pins the version,
the chunk ordering, and the entry set, defeating wire-attacker attempts
to drop, reorder, or downgrade.

#### Manifest flags

The manifest header carries a one-byte `flags` field (Section 8.5).
In this version of the protocol, one flag is defined:

* **`is_gateway`** (bit 0). The host SHALL set this bit when it has
  at least one active INTERNET-sphere transport connection (the
  condition that makes the host a gateway, Section 2.3); the host
  SHALL clear it otherwise. The flag reflects the host's gateway
  status at the moment the manifest version was emitted.

Bits 1-7 are reserved, MUST be zero, and are ignored by receivers.

The `flags` field is part of the manifest's signed content
(Section 8.5), so a receiver that verifies `manifest_signature`
can trust that the host itself asserted the flag value. Note that
the signature proves the host *claimed* the value; it does not
independently prove the host's INTERNET connectivity. The
`is_gateway` flag is a self-assertion.

A user evaluating a candidate node as a cross-host delegation
target uses this flag to confirm the candidate is a gateway
(Section 10.3). A change to the host's gateway status that flips
the flag triggers a manifest re-emission per Section 10.8.

### 10.2. Self-Delegation

Self-delegation is the delegation mode used when the user's host
node is a gateway (the host has an active INTERNET transport).
The user delegates to its own host; the host carries the user in
its manifest; the user is reachable globally via the host's
gateway role.

Mechanics:

1. The user generates a delegation entry (Section 10.1) with
   `host_node_id` set to its own host's full public key.
2. The entry is included in the host's next manifest version.
3. The manifest propagates per §8.5 / §8.6.

Because the user and the host are co-located, no wire
communication is needed to convey the delegation entry, the
entry is constructed locally and slotted into the host's manifest
directly. The host node's full public key, the `host_node_id`
the entry is signed over, is local state: the user's qaul
instance runs on the host node and holds both the node keypair
and the user keypair. Self-delegation therefore needs no profile
lookup to resolve `host_node_id`, unlike cross-host delegation
(Section 10.3), where the target node's full key must be resolved
from its node profile (Section 3.4).

When the host's gateway status changes (INTERNET transport is
gained or lost) the user's delegation choice may need to change:

* **Host gains INTERNET (becomes a gateway):** the user MAY
  self-delegate from this point onward. If a cross-host
  delegation was previously the only available option, the user
  MAY add a self-delegation alongside it for redundancy (see
  Section 10.6 on multi-gateway delegation), or replace it.
* **Host loses INTERNET (ceases to be a gateway):** the existing
  self-delegation no longer provides global reachability (the
  host has no gateway role to propagate across the Internet
  sphere). The user SHALL transition to cross-host delegation
  (Section 10.3) by selecting a local-reachable gateway and
  delegating to it.

### 10.3. Cross-Host Delegation

#### Terminology

For clarity in the rest of this section:

* **Host node** refers to the node where the delegating user
  lives. A user has exactly one host node at any given time.
* **Node** refers to any other node, including delegation targets
  that are not the user's host node.
* **Gateway** remains a property (any node with an active
  INTERNET transport); a node may simultaneously be a gateway,
  a delegation target, and someone's host node.

The wire-format field `host_node_id` (Section 10.1) names the
node holding the delegation, which may be the user's host node
(self-delegation, Section 10.2) or another node (cross-host
delegation, this section). The field name predates this
terminology pass and is retained for stability.

#### Mechanics

Cross-host delegation is the delegation mode used when the user's
host node is not a gateway. The user selects a local-reachable
gateway and delegates to it; the chosen gateway carries the user
in its manifest; the user is reachable globally via the gateway's
manifest propagation.

Steps:

1. The user identifies a delegation target meeting the selection
   criteria below.
2. The user generates a delegation entry (Section 10.1) with
   `host_node_id` set to the target node's full public key.
3. The user transmits the signed entry to the target via the
   network management sub-protocol (Section 11), using a
   `DelegationSubscribe` message.
4. The target validates the entry signature and accepts it, or
   rejects it per implementation policy. On acceptance, the
   target includes the entry in its next manifest version.
5. The manifest propagates per §8.5 / §8.6; the user becomes
   reachable globally via the target gateway.

If the target rejects the subscription (for example, at capacity
or by policy), the user SHOULD select a different acceptable
gateway and retry. If no acceptable gateway is available in the
user's local sphere, the user remains local-only-reachable until
one appears (Section 2.4).

#### Selection

A user selecting a cross-host delegation target SHALL apply the
following criteria. A candidate node is eligible only if both
hold:

1. **Local-sphere reachable.** The candidate has `local_only = 1`
   in the user's routing-table view (Section 7.4), meaning it is
   reachable from the user's host node via Local-sphere
   transports only.
2. **Gateway.** The candidate's most recently received manifest
   has `flags.is_gateway = 1` (Sections 8.5, 10.1). A candidate
   that is not a gateway would carry the user in its manifest,
   but that manifest would not propagate across the Internet
   sphere, defeating the purpose of delegation.

Among eligible candidates, the user SHOULD select the one with
the lowest routing metric (best path quality). Implementations
MAY apply additional selection policy like trust signals, preferred
operators, capacity hints and so on, but no such policy is defined in this
version of the protocol.

If no candidate is eligible, the user remains
local-only-reachable until an eligible candidate appears in the
routing table. The user re-evaluates eligibility whenever the
routing table changes.

#### Local-sphere reachability requirement

A cross-host delegation entry SHALL only be signed by a user when
the target node has `local_only = 1` in the user's routing-table
view (Section 7.4), that is, when the target is reachable from
the user's host node via Local-sphere transports only. This
ensures the user can reliably convey the delegation entry to the
target node and can monitor whether the target stays reachable.

If a user has already signed a cross-host delegation and the
target node's `local_only` flag transitions from 1 to 0, the
user SHALL treat the delegation as broken and SHALL sign a fresh
delegation to another node that satisfies the local-only
condition. The previously signed entry will be dropped from the
old target node's manifest naturally (via TTL expiry or via the
user ceasing to maintain the delegation through the
re-introduction triggers in Section 3.8).

### 10.4. Delegation Lifetime

A delegation has a hard time limit and is additionally bounded by
the continued reachability of the delegating user. Two mechanisms,
both required.

**Hard limit (TTL).** The `timeout` field on each delegation entry
is an absolute expiry. A receiver MUST drop entries whose timeout
has passed, regardless of manifest version. The default delegation
TTL is **6 hours**. A delegating user is expected to refresh the
delegation before the TTL elapses whuch means issuing a fresh signed entry
with a later timeout, conveyed to the target the same way as the
original (Section 10.3 for cross-host delegation; locally for
self-delegation). The recommended refresh cadence is every 3 hours
(TTL/2), leaving a full window of margin against a missed refresh.

**Liveness limit (routing-protocol reachability).** The delegation
target monitors whether the delegating user is still reachable in
the target's routing state. No separate heartbeat messages are
defined: the routing protocol's normal origin updates (every 10
seconds) and route-expiry timeout (35 seconds, Section 7.5)
already provide a liveness signal. When the delegated user becomes
unreachable in the target's routing state (the user's routing
entry, or the routing entry for the user's host node, has expired)
the target SHALL drop the delegation entry from its next manifest
version. This is the mandatory reachability requirement of
Section 10.7.

The TTL bounds a stale or stolen-key delegation; the liveness
limit keeps a manifest from advertising users the target can no
longer deliver to. Both values are tunable post-deployment.

### 10.5. Revocation

A user revokes a delegation by one of two paths.

**Explicit revocation.** The user sends a signed revocation to
the delegation target via the network management sub-protocol
(Section 11), using a `DelegationRevoke` message. The
revocation is an ed25519 signature by the user over
`(host_node_id || timeout)` the same content as the delegation
entry being revoked (Section 10.1), identifying exactly which
delegation is cancelled. On receiving a valid revocation, the
target SHALL remove the matching entry (the entry whose
`(user_id, host_node_id, timeout)` triple matches) from its
next manifest version.

Binding the revocation to the delegation's `timeout` provides
replay protection: a captured revocation can only ever cancel
the one delegation it names, and the absolute-millisecond
`timeout` makes each delegation entry unique in practice. A
replayed revocation targeting an already-removed or
never-existing entry is a harmless no-op. An implementation MAY
additionally include a nonce in the revocation's signed content
for a hard uniqueness guarantee; this is not required.

**Implicit revocation (TTL lapse).** The user simply stops
refreshing the delegation. The entry expires at its `timeout`,
no later than the TTL window after issuance (Section 10.4).
This path requires no communication with the target and always
works, but is slow and can take up to the full TTL.

Explicit revocation is the fast path, preferred when the user is
online and the target is reachable. The TTL lapse is the
fallback for when the management sub-protocol channel to the
target is unavailable.

Separately from user-initiated revocation, a delegation target
drops an entry when the delegated user becomes unreachable in
its routing state (the liveness limit of Section 10.4). This
is target-initiated cleanup, not revocation by the user.

### 10.6. Multiple Gateways

A user MAY delegate to more than one gateway simultaneously by
signing separate entries, each with a different `host_node_id`.
Each chosen gateway holds its own copy of the user's signed
entry and includes the user in its own manifest. A user
reachable through several gateways appears in several manifests
at once.

**Redundancy.** A single delegation is a single point of
failure: if the one gateway loses local reachability or goes
offline, the user is globally unreachable until a re-delegation
completes (detect the loss, select a new gateway, subscribe,
wait for the new manifest to propagate). Delegating to two or
more gateways removes this gap, while any one of them remains
reachable, the user stays globally reachable. Users SHOULD
therefore consider maintaining delegations to more than one
gateway where suitable candidates exist.

**Receiver-side selection.** When a remote node finds a user
delegated through more than one gateway, it selects among them
by the metric to each gateway in its local routing table
(Section 9.2). The delegation entry itself contains no metric:
metrics are observed dynamically by each receiver.

### 10.7. Mandatory Reachability

A gateway MUST NOT indefinitely advertise a delegated user it can
no longer deliver to. When a delegated user becomes unreachable
in the gateway's routing state (the liveness limit of
Section 10.4), the gateway MUST drop that user's delegation entry
from its manifest.

The bound on how long a stale entry may persist is the sum of
two intervals:

* **Detection** — up to the route-expiry timeout (35 seconds,
  Section 7.5) before the gateway's routing state reflects the
  user's unreachability.
* **Emission** — up to one manifest rate-limit window (60
  seconds, Section 10.8) before the gateway's next manifest
  version, which omits the dropped entry, is emitted.

A gateway MUST therefore drop an unreachable delegated user from
its manifest within approximately **95 seconds** of the loss of
reachability. This requirement prevents gateways from
indefinitely advertising users they cannot deliver to, which
would black-hole traffic addressed to those users.

### 10.8. Emission Rate and Delta Updates

A gateway SHALL emit at most one manifest message (NODE_MANIFEST or
MANIFEST_DELTA) per minute under normal operation. Manifest changes
that occur between emission windows accumulate locally and are
batched into a single MANIFEST_DELTA at the next emission window.

The rate limit applies to gateway-originated emissions. The following
events bypass the rate limit and trigger emission in the next
1-second relay batch:

1. Single-user ↔ multi-user transition. The host's routing-form
   changes; the new state SHALL propagate immediately rather than
   wait up to a minute.
2. Neighbour-connect bootstrap. A new neighbour receives the
   gateway's current full NODE_MANIFEST immediately. This is a
   direct response to a peer event, not a periodic emission, and is
   not rate-limited.

A change to the host's `is_gateway` flag (Section 10.1) when the host
gains or loses an active INTERNET-sphere transport, triggers a
manifest re-emission so that the new flag value propagates. This
re-emission is subject to the normal 1-per-minute rate limit; it
does not bypass the limit. A host with a flapping INTERNET
connection therefore emits at most one manifest per minute
regardless of how often the flag oscillates.

Removals driven by loss of routing-protocol reachability to a
delegated user (Section 10.4) MAY wait for the next emission
window rather than bypassing the rate limit. Section 10.7
specifies the mandatory upper bound on how long a manifest may
continue to advertise an unreachable user.

#### Periodic full re-sync

A gateway SHALL emit a full NODE_MANIFEST in place of a
MANIFEST_DELTA at least every ten manifest emissions, OR every
hour, whichever occurs first. This bounds the catch-up time for
receivers that have missed an intermediate delta and ensures
periodic re-anchoring of stored state.

#### Receivers out of sync

A receiver that receives a MANIFEST_DELTA whose `from_version` does
not match its stored version for that host drops the delta. It
re-converges when the gateway next emits a full NODE_MANIFEST under
the periodic re-sync rule, or when a fresh bootstrap is triggered
(for example, on neighbour reconnect).

#### Choice of full vs delta

After the initial NODE_MANIFEST has been published, MANIFEST_DELTA is
the default emission kind. A gateway selects the message kind subject
to the following rules:

* The first emission to any receiver (bootstrap) MUST be a
  NODE_MANIFEST.
* Periodic re-sync MUST be a NODE_MANIFEST.
* A delta whose body would exceed the 60 KiB bound MUST be promoted
  to a NODE_MANIFEST.
* All other emissions SHOULD be MANIFEST_DELTAs.

A receiver processes both kinds into the same downstream state; only
the wire format differs.

---

## 11. Network Management Sub-Protocol

### 11.1. Purpose

The routing protocol of the preceding sections maintains
reachability by allowing any node forward a packet towards any
reachable user, hop by hop. Several mechanisms this specification
depends on are not reachability; they are addressed, end-to-end
control messages:

* **Profile fetch**: resolving an 8-byte identifier (Section 3.3)
  to the full multikey public key and profile of its subject
  (Section 3.4).
* **Delegation subscribe**: conveying a signed cross-host
  delegation entry from a user to a chosen gateway (Section 10.3).
* **Delegation revoke**: conveying a signed revocation from a
  user to a delegation target (Section 10.5).

The network management sub-protocol is a thin end-to-end
addressed-message layer that carries these. It is a sub-protocol
of, not part of, the routing protocol: it depends on the routing
protocol for delivery but concerns control messages rather than
reachability.

### 11.2. Architecture

* **Distinct behaviour.** The sub-protocol is implemented as a
  network behaviour separate from the routing-info and messaging
  behaviours.
* **Delivery rides on the routing table.** A management message
  is addressed to an 8-byte ID; each hop forwards it using an
  ordinary routing-table next-hop lookup (Section 9.2). The
  sub-protocol performs no routing of its own.
* **Best-effort.** There is no acknowledgement-of-delivery, no
  retransmission within the sub-protocol, and no delay-tolerant
  storage. An undeliverable management message is dropped. Each
  use case detects a missing outcome, from a profile that did not
  arrive, to a manifest that still does not list the user and
  re-issues the request at its own layer.
* **Request/response.** Every operation is a request and a
  response, correlated by a `request_id`.
* **Protobuf encoding.** Management messages are infrequent and
  not on any per-second hot path, so the byte-level compactness
  the routing protocol's hot-path messages require does not
  apply.

### 11.3. Message Envelope

Every management message is a protobuf `ManagementMessage`:

    message ManagementMessage {
      uint32 version             = 1;  // sub-protocol version
      bytes  destination         = 2;  // 8-byte ID (Section 3.3)
      bool   destination_is_node = 3;  // index space, Section 3.5
      bytes  source              = 4;  // 8-byte ID of the requester
      bool   source_is_node      = 5;
      uint32 request_id          = 6;  // correlates response to request
      oneof  body {
        ProfileRequest          profile_request          = 7;
        ProfileResponse         profile_response         = 8;
        DelegationSubscribe     delegation_subscribe     = 9;
        DelegationSubscribeAck  delegation_subscribe_ack = 10;
        DelegationRevoke        delegation_revoke        = 11;
        DelegationRevokeAck     delegation_revoke_ack    = 12;
      }
    }

### 11.4. Forwarding

On receiving a `ManagementMessage`, a node:

1. Compares `destination` to its own identities (the node itself,
   and each user it hosts). If it matches, the node processes the
   message per its `body` kind (11.5–11.7).
2. Otherwise forwards the message toward `destination` using a
   routing-table next-hop lookup (Section 9.2), exactly as a user
   packet is forwarded; `destination_is_node` selects the index
   space (Section 3.5).
3. If `destination` cannot be resolved, drops the message
   (best-effort, 11.2).

A response body is addressed to the original `source`, carries
the same `request_id`, and is forwarded the same way.

### 11.5. Profile Fetch

    message ProfileRequest  { uint32 cached_version = 1; }

    message ProfileResponse { bool found = 1; Profile profile = 2; }

    message Profile {
      bytes  multikey        = 1;  // full multikey public key
      uint32 profile_version = 2;
      string name            = 3;  // may be empty
      bytes  self_signature  = 4;  // subject's ed25519 signature over
                                   // (multikey || profile_version || name)
    }

A node fetches a profile when it holds an 8-byte ID but needs the
subject's full key (to verify a signature) or a fresher profile
than it has cached. The `ProfileRequest` is addressed to the
subject (the envelope `destination`); `cached_version` is the
requester's cached `profile_version`, or 0 if none.

The subject's host answers with a `ProfileResponse`. A node that
holds the requested profile cached MAY answer on the subject's
behalf rather than forwarding the request further.

A receiver of a `ProfileResponse` SHALL verify that
`hash(multikey)` equals the requested ID and that `self_signature`
verifies against `multikey` before using the profile.

### 11.6. Delegation Subscribe

    message DelegationSubscribe {
      bytes  user_id         = 1;  // 8-byte ID of the delegating user
      uint64 timeout         = 2;
      bytes  entry_signature = 3;  // per Section 10.1
    }

    enum RejectReason { NONE = 0; AT_CAPACITY = 1; POLICY = 2; }

    message DelegationSubscribeAck {
      bool         accepted = 1;
      RejectReason reason   = 2;
    }

A user delegating to a node (cross-host delegation, Section 10.3)
sends a `DelegationSubscribe` addressed to that node. The payload
is the delegation entry of Section 10.1; the `host_node_id` it is
signed over is the envelope `destination`.

The target validates `entry_signature` against the delegating
user's full key by fetching the user's profile first (11.5) if it
is not held and either accepts the entry (including it in the
next manifest version, Section 10.1) or rejects it. It replies
with a `DelegationSubscribeAck`.

### 11.7. Delegation Revoke

    message DelegationRevoke {
      bytes  user_id          = 1;
      uint64 timeout          = 2;  // identifies the delegation entry
      bytes  revoke_signature = 3;  // per Section 10.5
    }

    message DelegationRevokeAck { bool done = 1; }

A user revoking a delegation sends a `DelegationRevoke` addressed
to the delegation target. The target validates `revoke_signature`
(Section 10.5), removes the matching entry — the entry whose
`(user_id, host_node_id, timeout)` triple matches — from its next
manifest version, and replies with a `DelegationRevokeAck`.
Removal of an already-absent entry is a successful no-op.

---

## 12. Partition Merge

When two previously separated partitions reconnect, no merge-specific
protocol mechanics are invoked. The protocol's standard rules suffice.

* Independent sequence number states across the partitions are
  reconciled by ordinary fresher-sequence-wins comparison
  (Section 6.2). Sequence numbers that have advanced far during the
  separation are accepted via reboot detection (Section 6.3) where
  applicable.
* Apparent conflicts in indexes are not actual conflicts: indexes are
  node-global. Two different nodes in the two partitions MAY have
  assigned the same index value to different targets in their own
  dictionaries; this is normal operation and presents no ambiguity to
  any receiver.
* Stale routes from before the merge expire via the normal 35-second
  expiry timeout (Section 7.5).
* The node bridging the partitions sends `INDEX_DUMP` to its new
  neighbours on connect, just as on any other neighbour connection.

If unforeseen edge cases emerge during real deployment, this section
will be revisited.

---

## 13. Security Considerations

### 13.1. Threat Model Scope

This version of the protocol assumes an open routing layer: any node
is permitted to participate in routing. Routing-update authentication
is not required for v1.

This is a deliberate scoping choice. A trusted-routing scheme
introduces a key-management problem comparable in complexity to the
routing problem itself; deferring it allows the rest of the protocol
to ship and be measured. Future versions are expected to add a route
authentication layer.

### 13.2. Manifest Authentication

Delegation entries and manifests are authenticated regardless of the
status of route authentication. Every manifest entry carries a
user-side ed25519 signature over `(host_node_id || timeout)`, where
`host_node_id` is the host's full multikey-encoded public key
(Section 10.1). The manifest as a whole carries a host-side
ed25519 signature over the chunk contents.

Both signatures are full 64-byte ed25519. Truncation is not used.
The signature length is determined by the cryptographic security
parameter of the curve and cannot be reduced without lowering the
forgery cost below acceptable bounds.

A receiver SHALL verify both signatures before acting on a manifest.
A failed manifest signature aborts processing of the entire chunk;
a failed entry signature aborts processing of the affected entry only.

### 13.3. Future Work

The following are deferred to a future version of this protocol:

* Route authentication: signed routing updates with verifiable origin
  and integrity along the path.
* Trusted-node routing: the ability for senders to restrict their
  packets to routes traversing only a specified set of trusted nodes.
* Spam and abuse mitigation: mechanisms for participants to identify
  and exclude misbehaving nodes.
* Multipath routing: storing more than one route per target and using
  them in parallel for resilience or load balancing.
* Per-link sub-metrics beyond bucketed RSSI on BLE: refinement of the
  metric formula based on measured deployment data.
* Pull- or DHT-based manifest distribution among gateways, required
  only when the gateway count itself reaches the tens of thousands —
  beyond the current design target. v1 uses scoped push (Section 2.3
  hierarchical scoping): manifests flood within their home Local
  sphere and across the Internet sphere, but never into a foreign
  Local sphere, so a leaf node's state stays bounded by its village
  and the global directory is replicated only across gateways.

---

## 14. Timing Parameters

| Parameter                       | Value           |
|---------------------------------|-----------------|
| Origin update interval          | 10 seconds      |
| Relay interval                  | 1 second        |
| Average per-hop propagation     | 0.5 seconds     |
| Route expiry timeout            | 35 seconds      |
| Index reuse cooldown            | 60 seconds      |
| Sequence-number gap for reboot  | > 100 (≈ 16 m)  |
| Delegation TTL                  | 6 hours         |
| Delegation refresh cadence      | 3 hours (recommended, TTL/2) |
| Mandatory reachability drop     | ≈ 95 s (35 + 60) |
| Manifest emission rate limit    | 1 per minute    |
| Periodic full manifest re-sync  | 10 emissions or 1 hour, whichever first |

Neighbour ping interval is determined by the transport layer and is
not specified here.

---

## Appendix A. Implementation Notes (Non-Normative)

This appendix records guidance for implementers. It is not part of the
normative wire protocol and conforming implementations MAY differ.

### A.1. Threading Model

A reference implementation maintains all routing state behind
`Arc<RwLock<...>>`. The protocol does not require multi-threaded access
in the strict sense: in practice, all routing-state operations occur
within a single async task. The atomic-reference-counted, read-write
locked wrapping accommodates the multi-threaded async runtime under
which the rest of the application typically executes, and keeps
contention zero in normal operation.

A single-task actor pattern, in which one task owns all routing state
and others communicate with it via channels, is a viable alternative.
It allows internal use of non-atomic single-threaded primitives at the
cost of converting all call sites to channel-based asynchronous
operations. This pattern is recommended only after measurement
indicates that lock contention is significant, which is unlikely in
typical deployments.

### A.2. Data Structures

A reference data structure layout follows. Field types are illustrative.

```
RouterState {
    routing_table: Arc<RwLock<RoutingTable>>,
    nodes:         Arc<RwLock<NodesMap>>,
    users:         Arc<RwLock<UsersMap>>,
    // ... additional state
}

RoutingTable {
    user_entries: Vec<Option<Arc<RwLock<RoutingEntry>>>>,    // capacity 65_536
    node_entries: Vec<Option<Arc<RwLock<RoutingEntry>>>>,    // capacity 65_536
    user_cooldown: VecDeque<(u16, Instant)>,
    node_cooldown: VecDeque<(u16, Instant)>,
}

// NodeId and UserId are the 8-byte IDs from Section 3.3.
NodesMap { inner: HashMap<NodeId, Arc<RwLock<Node>>> }
UsersMap { inner: HashMap<UserId, Arc<RwLock<User>>> }

RoutingEntry {
    target_index: u16,
    target:       TargetRef,         // strong reference, see A.3
    seq_num:      u16,
    metric:       u16,
    next_hop:     u16,
    transport:    TransportId,
    last_update:  u64,
    hop_count:    u8,
    local_only:   bool,              // extracted from hop_count bit 7
                                     // on receipt; see Section 7.4
    // profile_version / manifest_version live on User / Node,
    // not on the routing entry (Section 9.1).
}

enum TargetRef {
    User(Arc<RwLock<User>>),
    Node(Arc<RwLock<Node>>),
}

Node {
    id:               NodeId,        // 8 bytes
    public_key:       Multikey,      // resolved from node profile
    manifest_version: u32,
    is_gateway:       bool,          // from manifest flags (§8.5, §10.1)
    delegated_users:  Vec<DelegatedUser>,
}

DelegatedUser {
    user_id:            UserId,      // 8 bytes
    user:               Arc<RwLock<User>>,
    delegation_timeout: u64,
}

User {
    id:                  UserId,     // 8 bytes
    public_key:          Multikey,   // resolved from user profile
    profile_version:     u32,
    routing_entry:       Option<Weak<RwLock<RoutingEntry>>>,
    delegation_gateways: Vec<Weak<RwLock<Node>>>,
}
```

The two routing-table arrays are pre-allocated with a fixed capacity
of 65,536 each (one for user entries and one for node entries),
matching the protocol's index space. Slots are accessed by index, not
by `Vec::push`.

### A.3. Reference Cycle Avoidance

The reference graph contains cycles between `User` and `RoutingEntry`,
and between `User` and `Node` via `DelegatedUser`. Without breaking
these cycles, reference counts will not reach zero on entry expiry,
and memory will leak.

The recommended assignment is:

| Edge                                     | Strength |
|------------------------------------------|----------|
| `RoutingEntry.target` (User or Node)     | strong   |
| `User.routing_entry`                     | weak     |
| `Node.delegated_users[].user`            | strong   |
| `User.delegation_gateways[]`             | weak     |

Forward edges (entry → target, manifest → user) are strong: the
routing table and manifest each own the lifetimes of their
participants. Back edges (user → entry, user → gateways) are weak;
when the strong-side disappears, the weak reference resolves to None.

### A.4. Lookup Flow

A reference implementation of next-hop lookup for a given recipient
user is shown below. The function takes only short-lived read locks
and resolves weak back-references via `upgrade()`.

```
fn next_hop(users: &UsersMap, recipient: UserId)
    -> Option<(NodeId, TransportId)>
{
    let user_arc = users.get(&recipient)?.clone();
    let user = user_arc.read().ok()?;

    if let Some(weak) = &user.routing_entry {
        if let Some(entry_arc) = weak.upgrade() {
            let entry = entry_arc.read().ok()?;
            return Some((next_hop_id(entry.next_hop), entry.transport));
        }
    }

    user.delegation_gateways.iter()
        .filter_map(|w| w.upgrade())
        .filter_map(|gw| {
            let node = gw.read().ok()?;
            gateway_routing_entry(&node).map(|e| (
                e.metric,
                next_hop_id(e.next_hop),
                e.transport,
            ))
        })
        .min_by_key(|(metric, _, _)| *metric)
        .map(|(_, hop, transport)| (hop, transport))
}
```

Intermediate hops follow the destination index from the packet header
through a single routing-table lookup, without walking `UsersMap`.

---

## Appendix B. Wire Format Summary

Quick-reference tables for implementers.

### B.1. Common Header Fields

| Offset | Size | Field        |
|--------|------|--------------|
| 0      | 1    | version      |
| 1      | 1    | type         |
| 2      | 2    | payload_len  |

### B.2. Sizes by Element

| Element                                      | Size            |
|----------------------------------------------|-----------------|
| Common header                                | 4 bytes         |
| ROUTING_UPDATE entry (delta, no escape)      | 6 bytes         |
| ROUTING_UPDATE entry (escape, absolute idx)  | 8 bytes         |
| Inline mapping introduction (user, no esc.)  | 13 bytes        |
| Inline mapping introduction (escape)         | 15 bytes        |
| INDEX_DUMP per mapping (delta, no escape)    | 13 bytes        |
| INDEX_DUMP per mapping (escape, absolute)    | 15 bytes        |
| Identifier (target_id) on the wire           | 8 bytes         |
| Profile / manifest version                   | 4 bytes         |
| NODE_MANIFEST overhead                       | 79 bytes        |
| NODE_MANIFEST per entry                      | 80 bytes        |
| MANIFEST_DELTA overhead                      | 83 bytes        |
| MANIFEST_DELTA per add                       | 80 bytes        |
| MANIFEST_DELTA per remove                    | 8 bytes         |

ROUTING_UPDATE entries are: 1-byte delta + 2 seq + 2 metric + 1
hop_count. The escape variant prepends a 2-byte absolute index
after the `0x00` delta marker. Version travels with inline
mappings, not with routing entries (Section 8.3).

Inline mappings are: 1-byte delta + 8 target_id + 4 version.

### B.3. Index Spaces

| Space          | Width    | Range            |
|----------------|----------|------------------|
| User indexes   | 16 bits  | 0 to 65,535      |
| Node indexes   | 16 bits  | 0 to 65,535      |

The two spaces are independent. The kind of an index is determined
by the section of the message in which it appears (Section 8.3),
not by any flag bit within the index value.
