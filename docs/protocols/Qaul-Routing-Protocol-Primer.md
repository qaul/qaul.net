# Qaul Routing Protocol v2 — Primer

**Status:** Non-normative companion to
[`Qaul-Routing-Protocol.md`](./Qaul-Routing-Protocol.md).

**Purpose.** This document explains the *why* and the intuitions
behind the routing protocol — worked examples, analogies, and
plain-language walkthroughs. It does not define the protocol; the
specification is the source of truth, and where this primer and the
spec disagree, the spec wins. Each topic cross-references the spec
section that carries the precise rules.

The primer is assembled from explanations produced while reviewing
the protocol design. It covers the concepts examined in that
review, not the entire protocol.

---

## Part I — Spheres and the shape of the network

### 1. Spheres, and why membership is static

#### What it means

Every transport (LAN, BLE 1M, BLE Coded, INTERNET) belongs to
exactly one sphere — Local or Internet. That assignment is:

- **Static** — fixed, never changes while the node runs.
- **Per transport module** — baked into the module's definition at
  compile time, not configured or measured.

So you can't, at runtime, decide that a particular connection is
"actually Internet-like." LAN connections are always Local sphere;
INTERNET connections are always Internet sphere; full stop.

#### Why static is the right call

This is sound, and worth being explicit about *why*:

- **The membrane depends on it.** The whole scoped-push model
  (§2.3 hierarchical scoping) rests on a node deterministically
  knowing which sphere an entry arrived on and which sphere it's
  sending to. If sphere membership were dynamic — measured, or
  runtime-configurable — a flapping measurement could flip a
  connection's sphere and scramble the propagation filter. Static
  membership = a stable membrane.
- **The filter is a constant lookup.** "Filter user entries when
  sending over an Internet-sphere transport" (§2.3) is trivial if
  the transport's sphere is a compile-time constant. If it were
  dynamic, the filter would shift under you.
- **It falls out of the module split for free.** qaul already
  separates the LAN module and the INTERNET module — even though
  both can use TCP. The sphere is just a property of *which
  module*, so there's nothing extra to decide per connection.

### 2. Hierarchical scoping: leaf nodes vs gateways

Leaf node = knows only its Local sphere (local users, local nodes,
its own gateway(s)). Gateway = knows its Local sphere **plus** the
Internet sphere (every other gateway and their manifests — the
global directory). Foreign gateways and foreign users never reach a
leaf node.

Two tiers, mirroring OSPF-area / BGP-core:

- **Leaf node** — holds only its **Local sphere**. Its own
  village's users, nodes, and gateway(s). Nothing foreign.
- **Gateway** — holds its **Local sphere + the Internet sphere**:
  every other gateway and every gateway's manifest (the global
  user→gateway directory).

A gateway is a **membrane**. Local-origin routing data stays
in/below the Local sphere; Internet-sphere data stays in the
Internet sphere. Exactly one thing crosses the membrane upward —
the gateway's own manifest (carrying its delegated users).
**Nothing crosses downward.**

#### What each node holds

| | Local sphere routing | Local manifests | Foreign gateways | Global directory (foreign manifests) |
|---|---|---|---|---|
| **Leaf node** | yes | yes (own village) | no | no |
| **Gateway** | yes | yes (own village) | yes (Internet sphere) | yes (Internet sphere) |

A leaf node's storage is bounded by its village — finally making
§2.3's claim ("a leaf's table is bounded by
`local_mesh + visible_gateways`") actually true.

### 3. Cross-village delivery, walked end to end

Alice (village A) messages Bob (village B). Both are
non-gateway-hosted, so both cross-host-delegated to their
gateways: Alice → G-A, Bob → G-B.

**Setup:**
- G-A's manifest contains Alice. G-B's manifest contains Bob.
- G-A and G-B exchange manifests over the Internet sphere → both
  gateways hold the global directory.
- Village-A leaf nodes hold only village A's Local sphere. They
  have no idea where Bob is. Same for village B.

**Segment 1 — Local sphere A (leaf → gateway):**

1. Alice's node has Bob as a contact, so it knows Bob's 8-byte
   user ID. It looks Bob up locally.
2. Bob isn't in village A's Local sphere and isn't in any manifest
   Alice's node holds. **Lookup fails.**
3. Default-route rule kicks in: can't resolve locally → forward
   toward the best local gateway, G-A. The packet header carries
   Bob's recipient ID.
4. The packet routes hop-by-hop through village A toward G-A. Each
   intermediate village-A node *also* fails to resolve Bob, *also*
   default-routes to G-A — so the packet flows to G-A naturally,
   no special marking needed.

**Segment 2 — Internet sphere (gateway → gateway):**

5. G-A receives the packet. G-A holds the global directory. It
   looks up Bob → finds Bob in G-B's manifest. So: Bob is
   reachable via G-B.
6. G-A looks up G-B in its Internet-sphere routing table → next
   hop toward G-B.
7. G-A forwards across the Internet sphere (possibly through
   intermediate gateways/relays) toward G-B. Packet still carries
   Bob's recipient ID — nothing is rewritten.

**Segment 3 — Local sphere B (gateway → leaf):**

8. G-B receives the packet, looks up Bob. Bob is in G-B's own
   manifest *and* in G-B's Local sphere — G-B can route to Bob
   directly.
9. G-B forwards the packet into village B's Local sphere toward
   Bob's node.
10. Hop-by-hop through village B → delivered to Bob's node.

**Path:** Alice's node → (Local sphere A) → G-A → (Internet
sphere) → G-B → (Local sphere B) → Bob's node. Three segments,
each gateway a stitch point.

#### Key properties

- **Leaf nodes never hold foreign manifests.** Alice's node never
  knew "Bob → G-B." It just default-routed to G-A and trusted G-A.
- **Only gateways resolve cross-sphere.** The global directory
  lives on gateways (thousands), never on leaf nodes (100k).
  That's the scaling win.
- **The packet carries Bob's ID end-to-end.** Every forwarding
  node does its own lookup (§9.2); intermediate nodes that can't
  resolve simply default-route onward.
- **The reply works symmetrically:** Bob → G-B → Internet → G-A →
  Alice. This is why *both* parties must be delegated — G-B needs
  to resolve Alice → G-A for the reply, which it can because it
  holds the global directory and Alice is delegated.

#### Subtleties

1. **"Unreachable" determination moves to the gateway.** Alice's
   node can't tell "Bob is far away" from "Bob doesn't exist" — it
   just default-routes optimistically. If G-A *also* can't find Bob
   in the global directory, *G-A* declares unreachable and hands to
   DTN. (This is exactly how a default route works on the
   Internet — your laptop doesn't know if an IP is alive; it trusts
   the gateway.)
2. **No local gateway in village A** → Alice's node has nowhere to
   default-route → Bob is unreachable from village A → DTN.
   Consistent with §2.4's no-gateway case.
3. **Multiple gateways in one village** — a leaf node picks its
   nearest gateway; this is loop-free without any coordination.
   See §4 below.

### 4. How a leaf node picks a gateway: nearest-gateway anycast

When a leaf node can't resolve a recipient locally, it forwards
toward a gateway. If a village has several gateways, it picks its
**nearest** one — and lets each node decide independently.

#### What this is — anycast

"Forward toward the nearest gateway, each node decides for itself"
is exactly **anycast** — routing to the nearest member of a set of
destinations. Anycast is a well-established, loop-free routing
technique (it's how DNS root servers, CDNs, and 1.1.1.1 work).
It's loop-free for the same reason ordinary shortest-path routing
is.

#### Why it can't loop

Define, for any node, its "potential" = the metric distance from
that node to its **nearest** gateway.

When a node forwards a packet toward its nearest gateway, the next
hop is one step along the *shortest path* to that gateway — so the
next hop is **strictly closer** to that gateway. And the next
hop's nearest gateway is at most that close (possibly closer, if a
different gateway is now nearer).

So the potential **strictly decreases at every hop.** It's a
non-negative quantity that goes down every step — it must hit zero
in finite hops. Potential zero = the packet is *at* a gateway. The
packet always arrives. No loop, ever.

This holds even if different nodes pick different gateways, and
even with inconsistent tie-breaking — the potential still drops
every hop regardless of which gateway each node chose.

#### The rule, and why it is also better

The rule is just: **a leaf node that can't resolve a recipient
forwards toward its nearest (best-metric) gateway.** Each node
decides independently. No hash, no marking, no deterministic-ID
coordination.

And nearest-gateway anycast beats the alternatives on every axis:

- **Optimal path** — each packet takes the genuine shortest route
  to *a* gateway.
- **Natural load-balancing** — nodes near G1 use G1, nodes near G2
  use G2; traffic splits by geography automatically.
- **Resilience** — a gateway dies, its metric goes stale/expires,
  nodes near it just re-resolve to the next-nearest. Free failover.

#### The one real caveat

During routing **convergence** (the table still settling after a
topology change), transient inconsistency could briefly loop a
packet. But that's true of *all* routing during convergence, not
anything specific to gateway selection — and the existing
hop-count TTL (§7.4, drop at 63) catches transient loops exactly
as it does everywhere else.
