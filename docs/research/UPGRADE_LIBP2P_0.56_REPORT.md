# libqaul Upgrade Report: libp2p 0.55 → 0.56 with async-std → tokio Migration

**Date:** 2026-01-09
**Branch:** chore/upgrade-libp2p-56
**Author:** Analysis by Claude

---

## Executive Summary

This report analyzes the upgrade path for libqaul from libp2p 0.55 to 0.56. The most significant change is the **complete removal of async-std support** in libp2p 0.56, requiring a migration to the tokio async runtime.

**Overall complexity: Medium** - The changes are mostly mechanical replacements with well-defined patterns.

---

## 1. Current State

### Dependencies

| Component | Current Version | Runtime |
|-----------|----------------|---------|
| libp2p | 0.55 | async-std |
| async-std | 1.13 | - |
| mDNS | `mdns::async_io::Behaviour` | async-std |
| futures-ticker | 0.0.3 | runtime-agnostic |

### Affected Crates

- `libqaul` (main library)
- `qaul_info` (custom libp2p protocol)
- `qaul_messaging` (custom libp2p protocol)
- `qauld` (daemon binary)
- `qaul-cli` (CLI binary)

---

## 2. Breaking Changes in libp2p 0.56

### 2.1 async-std Support Removal

libp2p 0.56 **completely removes async-std support**. The following are removed from SwarmBuilder:

- TCP transport for async-std
- QUIC transport for async-std
- DNS transport for async-std
- mDNS `async_io` variant

### 2.2 Other Changes

- **MSRV requirement:** Rust 1.83.0 or later
- **Deprecated APIs removed:** `Transport::with_bandwidth_logging`, `SwarmBuilder::with_bandwidth_logging`
- **Default idle-connection-timeout:** Changed to 10s (libqaul overrides this to `u64::MAX`, so no impact)

---

## 3. Required Changes

### 3.1 Cargo.toml Dependencies

#### File: `rust/libqaul/Cargo.toml`

**Line 16-17 - Change:**
```toml
# FROM:
libp2p = { version = "0.55", features = ["async-std", "macros", "floodsub", "identify", "mdns", "noise", "ping", "tcp", "yamux", "quic", "macros"] }
async-std = { version = "1.13", features = ["attributes"] }

# TO:
libp2p = { version = "0.56", features = ["tokio", "macros", "floodsub", "identify", "mdns", "noise", "ping", "tcp", "yamux", "quic"] }
tokio = { version = "1", features = ["full"] }
```

#### File: `rust/libp2p_modules/qaul_info/Cargo.toml`

**Line 10 - Change:**
```toml
# FROM:
libp2p = { version = "0.55", features = ["async-std", ...] }

# TO:
libp2p = { version = "0.56", features = ["tokio", ...] }
```

#### File: `rust/libp2p_modules/qaul_messaging/Cargo.toml`

**Line 10 - Change:**
```toml
# FROM:
libp2p = { version = "0.55", features = ["async-std", ...] }

# TO:
libp2p = { version = "0.56", features = ["tokio", ...] }
```

#### File: `rust/clients/qauld/Cargo.toml`

**Line 16 - Change:**
```toml
# FROM:
async-std = { version = "1.13", features = ["attributes"] }

# TO:
tokio = { version = "1", features = ["full"] }
```

#### File: `rust/clients/cli/Cargo.toml`

**Line 11 - Change:**
```toml
# FROM:
async-std = { version = "1.13", features = ["attributes"] }

# TO:
tokio = { version = "1", features = ["full", "io-std"] }
```

---

### 3.2 SwarmBuilder Migration

#### File: `rust/libqaul/src/connections/lan.rs`

**Lines 190-191 - Change:**
```rust
// FROM:
let mut swarm = SwarmBuilder::with_existing_identity(node_keys.to_owned())
    .with_async_std()

// TO:
let mut swarm = SwarmBuilder::with_existing_identity(node_keys.to_owned())
    .with_tokio()
```

#### File: `rust/libqaul/src/connections/internet.rs`

**Lines 257-258 - Change:**
```rust
// FROM:
let mut swarm = SwarmBuilder::with_existing_identity(node_keys.to_owned())
    .with_async_std()

// TO:
let mut swarm = SwarmBuilder::with_existing_identity(node_keys.to_owned())
    .with_tokio()
```

---

### 3.3 mDNS Behaviour Migration

#### File: `rust/libqaul/src/connections/lan.rs`

**Line 45 - Struct field change:**
```rust
// FROM:
pub mdns: mdns::async_io::Behaviour,

// TO:
pub mdns: mdns::tokio::Behaviour,
```

**Line 178 - Instantiation change:**
```rust
// FROM:
let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), Node::get_id()).unwrap();

// TO:
let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), Node::get_id()).unwrap();
```

---

### 3.4 Main Function Macros

#### File: `rust/clients/qauld/src/main.rs`

**Line 40 - Change:**
```rust
// FROM:
#[async_std::main]
async fn main() {

// TO:
#[tokio::main]
async fn main() {
```

#### File: `rust/clients/cli/src/main.rs`

**Line 43 - Change:**
```rust
// FROM:
#[async_std::main]
async fn main() {

// TO:
#[tokio::main]
async fn main() {
```

---

### 3.5 CLI stdin Migration

#### File: `rust/clients/cli/src/main.rs`

**Line 9 - Import change:**
```rust
// FROM:
use async_std::io;

// TO:
use tokio::io::{self, AsyncBufReadExt, BufReader};
```

**Lines 62-82 - stdin reading change:**
```rust
// FROM:
let mut stdin = io::BufReader::new(io::stdin()).lines();
// ...
let line_fut = stdin.next().fuse();
// ...
select! {
    line = line_fut => Some(EventType::Cli(line.expect("can get line").expect("can read line from stdin"))),
    // ...
}

// TO:
let stdin = io::stdin();
let reader = BufReader::new(stdin);
let mut lines = reader.lines();
// ...
// Note: tokio's lines() returns Result<Option<String>>, adjust accordingly
```

---

### 3.6 API Thread Spawning (Recommended)

#### File: `rust/libqaul/src/api/mod.rs`

**Lines 44-50 - Optional improvement:**
```rust
// CURRENT (works but not optimal):
thread::spawn(move || {
    block_on(async move {
        crate::start(storage_path, config).await;
    })
});

// RECOMMENDED with tokio:
thread::spawn(move || {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        crate::start(storage_path, config).await;
    });
});
```

---

## 4. Optional Migrations

### 4.1 Ticker to tokio::time::interval

The codebase uses `futures_ticker::Ticker` which is runtime-agnostic and will continue to work. However, for consistency with tokio, you may optionally migrate:

#### File: `rust/libqaul/src/lib.rs`

```rust
// CURRENT (works fine):
use futures_ticker::Ticker;
let mut rpc_ticker = Ticker::new(Duration::from_millis(10));
// Usage: rpc_ticker.next().fuse()

// OPTIONAL tokio native:
use tokio::time::{interval, Interval};
let mut rpc_ticker = interval(Duration::from_millis(10));
// Usage: rpc_ticker.tick()
```

### 4.2 void Crate Deprecation

The `void` crate is deprecated in libp2p 0.55+. Current usage in `Cargo.toml:35`:
```toml
void = "1.0"
```

Consider removing if not strictly needed, or replace with `std::convert::Infallible`.

---

## 5. Files Unchanged

The following components require **no changes**:

- **Main event loop** (`lib.rs:288-709`) - Uses `futures::select!` which is runtime-agnostic
- **Router modules** - No direct async runtime dependencies
- **Services modules** - No direct async runtime dependencies
- **Custom protocols** (`qaul_info`, `qaul_messaging`) - Only need Cargo.toml feature updates
- **BLE module** - Not libp2p-based

---

## 6. Migration Checklist

### Phase 1: Dependencies
- [ ] Update `rust/libqaul/Cargo.toml`
- [ ] Update `rust/libp2p_modules/qaul_info/Cargo.toml`
- [ ] Update `rust/libp2p_modules/qaul_messaging/Cargo.toml`
- [ ] Update `rust/clients/qauld/Cargo.toml`
- [ ] Update `rust/clients/cli/Cargo.toml`

### Phase 2: Core Library
- [ ] Update SwarmBuilder in `connections/lan.rs`
- [ ] Update SwarmBuilder in `connections/internet.rs`
- [ ] Update mDNS behaviour in `connections/lan.rs`

### Phase 3: Binaries
- [ ] Update main macro in `clients/qauld/src/main.rs`
- [ ] Update main macro in `clients/cli/src/main.rs`
- [ ] Update stdin handling in `clients/cli/src/main.rs`

### Phase 4: Optional Improvements
- [ ] Update API spawning in `api/mod.rs`
- [ ] Consider migrating `futures_ticker` to `tokio::time::interval`
- [ ] Remove deprecated `void` crate if unused

### Phase 5: Verification
- [ ] Run `cargo build`
- [ ] Run `cargo test`
- [ ] Test LAN discovery (mDNS)
- [ ] Test Internet connections (TCP/QUIC)
- [ ] Test CLI functionality

---

## 7. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| mDNS behavior differences | Low | Medium | Test LAN discovery thoroughly |
| tokio runtime conflicts | Low | High | Ensure single runtime instance |
| CLI stdin handling issues | Medium | Low | Test interactive CLI mode |
| Custom protocol breakage | Low | Medium | Verify qaul_info/qaul_messaging |

---

## 8. Complexity Summary

| Change Area | Complexity | Files | Lines Changed (est.) |
|-------------|------------|-------|---------------------|
| Cargo.toml deps | Low | 5 | ~10 |
| SwarmBuilder | Low | 2 | ~4 |
| mDNS migration | Low | 1 | ~4 |
| Main macros | Low | 2 | ~2 |
| CLI stdin | Medium | 1 | ~15 |
| API spawning | Low | 1 | ~6 |
| **Total** | **Medium** | **12** | **~41** |

---

## 9. References

- [libp2p 0.56.0 Changelog](https://github.com/libp2p/rust-libp2p/blob/master/libp2p/CHANGELOG.md)
- [tokio Documentation](https://docs.rs/tokio/latest/tokio/)
- [libp2p SwarmBuilder API](https://docs.rs/libp2p/0.56.0/libp2p/struct.SwarmBuilder.html)

---

## 10. Appendix: Full File List

### Files to Modify

1. `rust/libqaul/Cargo.toml`
2. `rust/libp2p_modules/qaul_info/Cargo.toml`
3. `rust/libp2p_modules/qaul_messaging/Cargo.toml`
4. `rust/clients/qauld/Cargo.toml`
5. `rust/clients/cli/Cargo.toml`
6. `rust/libqaul/src/connections/lan.rs`
7. `rust/libqaul/src/connections/internet.rs`
8. `rust/clients/qauld/src/main.rs`
9. `rust/clients/cli/src/main.rs`
10. `rust/libqaul/src/api/mod.rs` (optional)
11. `rust/libqaul/src/lib.rs` (optional - ticker migration)

### Files Unchanged

- `rust/libqaul/src/connections/mod.rs`
- `rust/libqaul/src/connections/events.rs`
- `rust/libqaul/src/connections/ble/*`
- `rust/libqaul/src/router/*`
- `rust/libqaul/src/services/*`
- `rust/libqaul/src/rpc/*`
- `rust/libqaul/src/node/*`
- `rust/libqaul/src/storage/*`
- `rust/libp2p_modules/qaul_info/src/*`
- `rust/libp2p_modules/qaul_messaging/src/*`
