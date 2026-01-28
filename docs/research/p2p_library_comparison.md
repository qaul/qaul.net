# P2P Library Feature Comparison

## rust-libp2p vs litep2p vs iroh

| Feature Category | rust-libp2p | litep2p | iroh |
|------------------|-------------|---------|------|
| **Repository** | [github.com/libp2p/rust-libp2p](https://github.com/libp2p/rust-libp2p) | [github.com/paritytech/litep2p](https://github.com/paritytech/litep2p) | [github.com/n0-computer/iroh](https://github.com/n0-computer/iroh) |
| **License** | MIT | MIT | MIT / Apache 2.0 |
| **GitHub Stars** | ~5.3k | ~133 | ~7.5k |
| **Primary Focus** | Full libp2p spec implementation | Lightweight libp2p-compatible library | Direct P2P connections (QUIC-focused) |
| **libp2p Compatible** | ✅ Full | ✅ Partial | ❌ Different approach |

---

## Node Identity

| Feature | rust-libp2p | litep2p | iroh |
|---------|-------------|---------|------|
| **Identity Creation** | ✅ `Keypair::generate_ed25519()`, secp256k1, RSA, ECDSA | ✅ Ed25519-based PeerId | ✅ Ed25519-based EndpointId (PublicKey) |
| **PeerId Format** | ✅ Multihash of public key | ✅ libp2p-compatible PeerId | ✅ z-base-32 encoded public key |
| **Key Types** | Ed25519, secp256k1, RSA, ECDSA | Ed25519 | Ed25519 (X25519 for ECDH) |
| **Persistent Identity** | ✅ Yes | ✅ Yes | ✅ Yes |

---

## Transport Layers

| Transport | rust-libp2p | litep2p | iroh |
|-----------|-------------|---------|------|
| **TCP** | ✅ Full support | ✅ Full support | ❌ Not directly (QUIC over UDP) |
| **QUIC** | ✅ Full support (quic-v1) | ✅ Full support | ✅ Primary transport (via Quinn) |
| **WebSocket** | ✅ WS + WSS | ✅ WS + WSS | ✅ WebSocket (for relay connections) |
| **WebRTC** | ✅ Browser-to-server (alpha) | ✅ Full support | ❌ No (uses relays for browsers) |
| **WebTransport** | ✅ Browser support | ❌ Not listed | ❌ No |
| **HTTPS** | ✅ Via WebTransport | ❌ No | ✅ Relay over HTTPS |
| **BLE (Bluetooth Low Energy)** | ❌ No | ❌ No | ❌ No |
| **Unix Domain Sockets** | ✅ Yes (uds) | ❌ No | ❌ No |

### Transport Notes

- **rust-libp2p**: Most comprehensive transport support. Includes DNS resolution, plaintext, and pnet (private network) transports.
- **litep2p**: Focused subset of transports optimized for blockchain use cases (Substrate/Polkadot).
- **iroh**: QUIC-only for data, with HTTPS/WebSocket for relay coordination. Prioritizes reliable connections over transport variety.

---

## LAN Discovery Mechanisms

| Feature | rust-libp2p | litep2p | iroh |
|---------|-------------|---------|------|
| **mDNS** | ✅ Full support (`libp2p-mdns`) | ✅ Yes (Multicast DNS) | ✅ Yes (`discovery-local-network` feature) |
| **Local Swarm Discovery** | ✅ Via mDNS | ✅ Via mDNS | ✅ Custom swarm-discovery implementation |
| **Passive Discovery Mode** | ✅ Yes | ❌ Unknown | ✅ Yes (listen without publishing) |
| **Expiry Events** | ✅ Yes | ❌ Unknown | ✅ Yes (DiscoveryEvent::Expired) |

### LAN Discovery Notes

- **rust-libp2p**: Standard libp2p mDNS implementation with discovered/expired events.
- **litep2p**: Basic mDNS support for local peer discovery.
- **iroh**: Custom `swarm-discovery` implementation with active/passive modes and application-specific service names for isolation.

---

## NAT Traversal & Holepunching

| Feature | rust-libp2p | litep2p | iroh |
|---------|-------------|---------|------|
| **Hole Punching** | ✅ DCUtR (Direct Connection Upgrade) | ⚠️ Limited | ✅ Primary feature (Disco protocol) |
| **STUN** | ✅ Via external | ⚠️ Unknown | ✅ Built-in |
| **QUIC Address Discovery (QAD)** | ❌ No | ❌ No | ✅ Yes (RFC implementation) |
| **Circuit Relay** | ✅ Full (v2) | ❌ No relay | ✅ DERP-style relay servers |
| **Relay Fallback** | ✅ Yes | ❌ No | ✅ Always available |
| **AutoNAT** | ✅ Yes | ❌ No | ✅ Built into net-report |
| **UPnP** | ✅ Yes (`libp2p-upnp`) | ❌ No | ❌ No |

### NAT Traversal Notes

- **rust-libp2p**: Comprehensive NAT traversal with AutoNAT detection, DCUtR for hole-punching, and circuit relay v2 for fallback.
- **litep2p**: Limited NAT traversal capabilities; designed for scenarios where nodes may have public IPs.
- **iroh**: Excellent hole-punching with "MagicSock" (inspired by Tailscale). Uses Disco ping/pong for coordination and DERP-style relays as fallback. Claims high hole-punch success rates.

---

## Transport Encryption

| Feature | rust-libp2p | litep2p | iroh |
|---------|-------------|---------|------|
| **Noise Protocol** | ✅ XX, IK, IX patterns | ✅ Yes | ❌ Uses QUIC/TLS 1.3 |
| **TLS 1.3** | ✅ Full support | ✅ Yes | ✅ Via QUIC (always on) |
| **Plaintext (testing)** | ✅ Available | ❌ No | ❌ No |
| **Private Network (PSK)** | ✅ pnet crate | ❌ No | ❌ No |
| **Forward Secrecy** | ✅ Yes | ✅ Yes | ✅ Yes (TLS 1.3) |

### Encryption Notes

- **rust-libp2p**: Flexible encryption with Noise or TLS 1.3, selectable per connection. Supports private networks with pre-shared keys.
- **litep2p**: Standard Noise/TLS support inherited from libp2p compatibility.
- **iroh**: All connections encrypted via QUIC/TLS 1.3. Uses self-signed certificates with EndpointId verification (libp2p handshake spec). No option for unencrypted connections.

---

## End-to-End Encryption

| Feature | rust-libp2p | litep2p | iroh |
|---------|-------------|---------|------|
| **Transport E2E Encryption** | ✅ Noise/TLS provides E2E | ✅ Yes | ✅ QUIC/TLS 1.3 (always E2E) |
| **Relay-Resistant E2E** | ✅ Yes (relay cannot read content) | N/A (no relay) | ✅ Yes (relay only sees encrypted packets) |
| **Application-Layer E2E** | ⚠️ User-implemented | ⚠️ User-implemented | ✅ Built-in (all traffic encrypted to peer) |
| **Peer Authentication** | ✅ Via PeerId verification | ✅ Via PeerId | ✅ EndpointId verification during handshake |

### E2E Encryption Notes

- All three libraries provide transport-level end-to-end encryption.
- **iroh**: Specifically designed so relay servers cannot decrypt traffic—they only forward encrypted QUIC packets.

---

## Broadcast / Multicast / PubSub

| Protocol | rust-libp2p | litep2p | iroh |
|----------|-------------|---------|------|
| **Floodsub** | ✅ Yes (legacy) | ❌ No | ❌ No |
| **Gossipsub** | ✅ Full implementation | ❌ No native gossipsub | ⚠️ Via iroh-gossip (separate crate) |
| **Episub** | ❌ Experimental | ❌ No | ❌ No |
| **Custom PubSub** | ✅ Via NetworkBehaviour | ✅ Notification protocol | ✅ iroh-gossip (HyParView + PlumTree) |
| **Topic-Based Subscription** | ✅ Yes | ✅ Yes (notification) | ✅ Yes (TopicId in iroh-gossip) |

### PubSub Notes

- **rust-libp2p**: Full gossipsub implementation with floodsub backward compatibility. Powers Ethereum and Filecoin consensus.
- **litep2p**: Uses a "Notification protocol" for broadcast rather than gossipsub. Simpler model for Substrate use cases.
- **iroh**: No built-in pubsub in core library. `iroh-gossip` is a separate crate using HyParView for membership and PlumTree for broadcast. Described as "gossipsub-like functionality."

---

## Routing & DHT

| Feature | rust-libp2p | litep2p | iroh |
|---------|-------------|---------|------|
| **Kademlia DHT** | ✅ Full `/ipfs/kad/1.0.0` | ✅ `/ipfs/kad/1.0.0` | ❌ No built-in DHT |
| **DHT Content Discovery** | ✅ Yes (provider records) | ✅ Yes | ❌ No |
| **DHT Peer Routing** | ✅ Yes | ✅ Yes | ⚠️ Via DHT discovery service (optional) |
| **Rendezvous Protocol** | ✅ Yes | ❌ No | ❌ No |
| **Mesh Routing** | ⚠️ Via gossipsub mesh | ⚠️ Via notification | ⚠️ Via iroh-gossip |

### Routing Notes

- **rust-libp2p**: Full Kademlia implementation for both peer routing and content discovery. Rendezvous for lightweight discovery.
- **litep2p**: Kademlia support for peer discovery and content routing.
- **iroh**: No DHT in core library. Discovery uses DNS (Pkarr), mDNS, or optional BitTorrent Mainline DHT integration. Focus is on direct connections, not distributed routing.

---

## Additional Protocols

| Protocol | rust-libp2p | litep2p | iroh |
|----------|-------------|---------|------|
| **Ping** | ✅ `/ipfs/ping/1.0.0` | ✅ `/ipfs/ping/1.0.0` | ✅ Disco ping/pong |
| **Identify** | ✅ `/ipfs/identify/1.0.0` | ✅ `/ipfs/identify/1.0.0` | ❌ Different approach |
| **Bitswap** | ✅ Yes | ✅ `/ipfs/bitswap/1.2.0` | ❌ Uses iroh-blobs instead |
| **Request-Response** | ✅ Generic protocol | ✅ Built-in | ✅ Via QUIC streams |

---

## Stream Multiplexing

| Multiplexer | rust-libp2p | litep2p | iroh |
|-------------|-------------|---------|------|
| **Yamux** | ✅ Primary | ✅ Yes | ❌ Uses QUIC native muxing |
| **Mplex** | ✅ Legacy support | ❌ No | ❌ No |
| **QUIC Native** | ✅ Yes (with QUIC transport) | ✅ Yes | ✅ Primary (via Quinn) |

---

## Platform Support

| Platform | rust-libp2p | litep2p | iroh |
|----------|-------------|---------|------|
| **Linux** | ✅ Full | ✅ Full | ✅ Full |
| **macOS** | ✅ Full | ✅ Full | ✅ Full |
| **Windows** | ✅ Full | ✅ Full | ✅ Full |
| **iOS** | ✅ Yes | ⚠️ Unknown | ✅ Yes (via FFI) |
| **Android** | ✅ Yes | ⚠️ Unknown | ✅ Yes (via FFI) |
| **WASM/Browser** | ✅ WebRTC, WebTransport | ⚠️ Limited | ✅ Alpha (relay-only mode) |

---

## Summary Comparison

### rust-libp2p
**Best for**: Projects needing full libp2p spec compliance, maximum interoperability with other libp2p implementations (Go, JS, etc.), or specific features like Kademlia DHT.

**Strengths**:
- Most comprehensive feature set
- Battle-tested in production (Ethereum, Filecoin, IPFS, Substrate)
- Full gossipsub and Kademlia implementations
- Extensive transport options

**Weaknesses**:
- Complex API with steep learning curve
- Many features means larger binary size
- More configuration required

---

### litep2p
**Best for**: Substrate/Polkadot ecosystem projects needing a lighter libp2p-compatible library.

**Strengths**:
- Simpler API than rust-libp2p
- Designed for blockchain networking needs
- libp2p protocol compatibility
- Maintained by Parity Technologies

**Weaknesses**:
- Smaller feature set than rust-libp2p
- No circuit relay or advanced NAT traversal
- No gossipsub (uses notification protocol instead)
- Smaller community

---

### iroh
**Best for**: Applications prioritizing reliable direct connections, mobile support, and simple API over libp2p compatibility.

**Strengths**:
- Excellent hole-punching (reportedly high success rates)
- Simple "dial by public key" API
- Always-encrypted connections
- Strong mobile platform support
- Good local network discovery
- Active development with clear focus

**Weaknesses**:
- Not libp2p compatible
- No built-in DHT (relies on DNS/external discovery)
- Gossip/pubsub requires separate crate
- Fewer transport options (QUIC-focused)
- Younger project

---

## Feature Availability Matrix

| Feature | rust-libp2p | litep2p | iroh |
|---------|:-----------:|:-------:|:----:|
| Node Identity | ✅ | ✅ | ✅ |
| TCP Transport | ✅ | ✅ | ❌ |
| QUIC Transport | ✅ | ✅ | ✅ |
| WebSocket | ✅ | ✅ | ✅ |
| WebRTC | ✅ | ✅ | ❌ |
| BLE | ❌ | ❌ | ❌ |
| mDNS Discovery | ✅ | ✅ | ✅ |
| Hole Punching | ✅ | ⚠️ | ✅ |
| Circuit Relay | ✅ | ❌ | ✅ |
| Noise Encryption | ✅ | ✅ | ❌ |
| TLS 1.3 | ✅ | ✅ | ✅ |
| Floodsub | ✅ | ❌ | ❌ |
| Gossipsub | ✅ | ❌ | ⚠️¹ |
| Kademlia DHT | ✅ | ✅ | ❌ |
| Request-Response | ✅ | ✅ | ✅ |

**Legend**: ✅ = Full Support | ⚠️ = Partial/Limited | ❌ = Not Available

¹ Via separate `iroh-gossip` crate using HyParView/PlumTree (not gossipsub protocol)

---

*Document generated: January 2026*
*Sources: Official repositories, documentation, and blog posts*
