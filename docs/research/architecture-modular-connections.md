# Modular Connection System Architecture

**Document Version:** 1.0
**Date:** 2026-01-09
**Status:** Proposal

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Current Architecture Analysis](#2-current-architecture-analysis)
3. [Design Goals](#3-design-goals)
4. [Proposed Architecture](#4-proposed-architecture)
5. [Core Components](#5-core-components)
6. [Module Implementation Guide](#6-module-implementation-guide)
7. [Routing Integration](#7-routing-integration)
8. [Configuration System](#8-configuration-system)
9. [RPC Interface](#9-rpc-interface)
10. [Migration Plan](#10-migration-plan)
11. [Example Implementations](#11-example-implementations)
12. [Appendix: Full Code Listings](#12-appendix-full-code-listings)

---

## 1. Executive Summary

This document proposes a plugin-based architecture for libqaul's connection modules that enables:

- **Dynamic module registration**: Add new transport protocols without modifying core code
- **Runtime activation/deactivation**: Enable or disable modules while the system is running
- **Unified routing**: All active modules contribute to a single routing table with automatic best-path selection
- **Capability-aware dispatching**: The system understands what each module can and cannot do

The architecture introduces a `ConnectionTransport` trait that all modules implement, a `ModuleRegistry` for lifecycle management, and a `UnifiedNeighbourTable` that aggregates peer information from all active modules.

---

## 2. Current Architecture Analysis

### 2.1 Existing Module Structure

The current implementation has three hardcoded connection modules:

```
src/connections/
├── mod.rs          # Module orchestration, ConnectionModule enum
├── events.rs       # Shared event handling
├── lan.rs          # LAN/mDNS discovery module
├── internet.rs     # Internet overlay module
└── ble/            # Bluetooth Low Energy module
    ├── mod.rs
    └── *.proto     # Protocol definitions
```

### 2.2 Current ConnectionModule Enum

```rust
pub enum ConnectionModule {
    Local,      // Local user (no routing needed)
    Lan,        // LAN discovery via mDNS
    Internet,   // Static Internet peer connections
    Ble,        // Bluetooth Low Energy
    None,       // Unknown/no connection
}
```

### 2.3 Current Patterns

| Pattern | Description | Used By |
|---------|-------------|---------|
| libp2p NetworkBehaviour | Composite behavior with sub-behaviors | LAN, Internet |
| Event Aggregation | Custom enum aggregating sub-behavior events | LAN, Internet |
| IPC Messages | Inter-process communication for mobile | BLE |
| InitCell + RwLock | Global state management | All modules |
| Per-Module Neighbours | Separate neighbour tables per module | Router |

### 2.4 Limitations of Current Architecture

1. **Static module definition**: Adding a new module requires modifying `ConnectionModule` enum and multiple match statements throughout the codebase
2. **No runtime control**: Modules cannot be activated/deactivated at runtime
3. **Duplicated patterns**: Each module implements similar boilerplate
4. **Tight coupling**: Core code directly references specific modules
5. **Inconsistent interfaces**: BLE uses a different pattern than LAN/Internet

---

## 3. Design Goals

### 3.1 Primary Goals

| Goal | Description |
|------|-------------|
| **Modularity** | New modules can be added by implementing a single trait |
| **Runtime Control** | Modules can be activated/deactivated without restart |
| **Unified Routing** | Single routing table aggregates data from all modules |
| **Capability Awareness** | System knows each module's capabilities |
| **Backward Compatibility** | Existing modules continue to work during migration |

### 3.2 Non-Goals

- Automatic module discovery (modules must be explicitly registered)
- Hot-loading of module code at runtime (requires restart to add new module types)
- Cross-node module negotiation (handled by existing protocols)

---

## 4. Proposed Architecture

### 4.1 Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                          RPC Interface                               │
│   (activate_module, deactivate_module, list_modules, get_status)    │
└────────────────────────────────┬────────────────────────────────────┘
                                 │
┌────────────────────────────────▼────────────────────────────────────┐
│                        Module Registry                               │
│                                                                      │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │   LAN    │ │ Internet │ │   BLE    │ │   Iroh   │ │  Nostr   │  │
│  │ (active) │ │ (active) │ │ (stopped)│ │ (active) │ │ (stopped)│  │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘  │
│       │            │            │            │            │         │
│       └────────────┴────────────┴────────────┴────────────┘         │
│                                 │                                    │
│                  impl ConnectionTransport                            │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  │ poll_events() → ModuleEvent
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        Event Dispatcher                              │
│                                                                      │
│   PeerConnected ──────────────► UnifiedNeighbourTable               │
│   PeerDisconnected ───────────► UnifiedNeighbourTable               │
│   RttUpdated ─────────────────► UnifiedNeighbourTable               │
│   RoutingInfoReceived ────────► RouterInfo                          │
│   MessageReceived ────────────► Messaging Service                   │
│   BroadcastReceived ──────────► Feed Service                        │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Unified Neighbour Table                           │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │ PeerId A: { lan: 50ms, internet: 120ms }                       │ │
│  │ PeerId B: { iroh: 80ms }                                       │ │
│  │ PeerId C: { lan: 30ms, ble: 200ms }                            │ │
│  │ PeerId D: { internet: 95ms, iroh: 110ms, nostr: 500ms }        │ │
│  └────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Routing Table                                │
│                                                                      │
│   • Best path selection (lowest RTT, hop count, link quality)       │
│   • Module priority for tie-breaking                                 │
│   • Routing info scheduler (deduplication)                          │
│   • Multi-path awareness for redundancy                             │
└─────────────────────────────────────────────────────────────────────┘
```

### 4.2 Data Flow

```
                    ┌─────────────────┐
                    │  External World │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        ▼                    ▼                    ▼
   ┌─────────┐         ┌─────────┐         ┌─────────┐
   │   LAN   │         │ Internet│         │  Iroh   │
   │ Module  │         │ Module  │         │ Module  │
   └────┬────┘         └────┬────┘         └────┬────┘
        │                   │                   │
        │ ModuleEvent       │ ModuleEvent       │ ModuleEvent
        │                   │                   │
        └───────────────────┼───────────────────┘
                            ▼
                   ┌─────────────────┐
                   │ Event Dispatcher│
                   └────────┬────────┘
                            │
           ┌────────────────┼────────────────┐
           ▼                ▼                ▼
   ┌───────────────┐ ┌───────────┐ ┌─────────────────┐
   │  Neighbour    │ │  Router   │ │    Services     │
   │    Table      │ │   Info    │ │ (Msg, Feed, etc)│
   └───────────────┘ └───────────┘ └─────────────────┘
```

---

## 5. Core Components

### 5.1 ConnectionTransport Trait

The foundational trait that all connection modules must implement:

```rust
// src/connections/traits.rs

use async_trait::async_trait;
use libp2p::PeerId;
use std::collections::HashMap;

/// Capability flags indicating what a module supports
#[derive(Clone, Debug, Default)]
pub struct ModuleCapabilities {
    /// Can flood messages to all connected peers
    pub supports_broadcast: bool,
    /// Can send messages to a specific peer
    pub supports_direct: bool,
    /// Automatically discovers peers (e.g., mDNS, DHT)
    pub supports_discovery: bool,
    /// Needs bootstrap peer addresses to start
    pub requires_bootstrap: bool,
    /// Only works on local network (not routable over internet)
    pub is_local_only: bool,
    /// Supports NAT traversal / hole punching
    pub supports_nat_traversal: bool,
    /// Works on mobile platforms
    pub mobile_compatible: bool,
}

/// Information about a connected neighbour
#[derive(Clone, Debug)]
pub struct NeighbourInfo {
    /// The peer's libp2p PeerId
    pub peer_id: PeerId,
    /// Round-trip time in microseconds
    pub rtt_micros: u32,
    /// Timestamp when connection was established
    pub connected_at: u64,
    /// Module-specific metadata
    pub metadata: HashMap<String, String>,
}

/// Events emitted by connection modules
#[derive(Clone, Debug)]
pub enum ModuleEvent {
    /// A new peer was discovered/connected
    PeerConnected {
        peer_id: PeerId,
        rtt: Option<u32>,
    },

    /// A peer disconnected
    PeerDisconnected {
        peer_id: PeerId,
    },

    /// RTT measurement updated
    RttUpdated {
        peer_id: PeerId,
        rtt_micros: u32,
    },

    /// Routing info received from peer
    RoutingInfoReceived {
        peer_id: PeerId,
        data: Vec<u8>,
    },

    /// Direct message received
    MessageReceived {
        peer_id: PeerId,
        data: Vec<u8>,
    },

    /// Broadcast/flood message received
    BroadcastReceived {
        topic: String,
        data: Vec<u8>,
    },

    /// Module status changed
    StatusChanged {
        status: ModuleStatus,
    },

    /// Custom event for module-specific needs
    Custom {
        event_type: String,
        data: Vec<u8>,
    },
}

/// Module lifecycle states
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModuleStatus {
    /// Module created but not initialized
    Uninitialized,
    /// Module is starting up
    Starting,
    /// Module is running and operational
    Running,
    /// Module is shutting down
    Stopping,
    /// Module is stopped
    Stopped,
    /// Module encountered an error
    Error(String),
}

/// Configuration passed to modules during initialization
#[derive(Clone)]
pub struct ModuleConfig {
    /// The node's libp2p keypair for identity
    pub node_keypair: libp2p::identity::Keypair,
    /// Module-specific settings from configuration file
    pub settings: HashMap<String, toml::Value>,
}

/// Errors that can occur in module operations
#[derive(Debug, thiserror::Error)]
pub enum ModuleError {
    #[error("Module not initialized")]
    NotInitialized,

    #[error("Module not running")]
    NotRunning,

    #[error("Operation not supported: {0}")]
    NotSupported(String),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// The core trait all connection modules must implement
#[async_trait]
pub trait ConnectionTransport: Send + Sync {
    // ============ Identity & Capabilities ============

    /// Unique identifier for this module type
    /// Examples: "lan", "internet", "ble", "iroh", "nostr"
    fn module_id(&self) -> &'static str;

    /// Human-readable display name
    /// Examples: "LAN (mDNS)", "Internet Overlay", "Bluetooth LE"
    fn display_name(&self) -> &str;

    /// Module capabilities
    fn capabilities(&self) -> ModuleCapabilities;

    /// Current module status
    fn status(&self) -> ModuleStatus;

    // ============ Lifecycle ============

    /// Initialize the module with configuration
    /// Called once when module is registered
    async fn init(&mut self, config: ModuleConfig) -> Result<(), ModuleError>;

    /// Start the module
    /// Can be called multiple times for activate/deactivate cycles
    async fn start(&mut self) -> Result<(), ModuleError>;

    /// Stop the module gracefully
    /// Should close all connections and release resources
    async fn stop(&mut self) -> Result<(), ModuleError>;

    // ============ Event Handling ============

    /// Poll for pending events
    /// Called frequently in the main event loop
    async fn poll_events(&mut self) -> Vec<ModuleEvent>;

    // ============ Peer Information ============

    /// Get list of currently connected neighbours
    fn neighbours(&self) -> Vec<NeighbourInfo>;

    /// Get listening addresses (if applicable)
    fn listening_addresses(&self) -> Vec<String>;

    // ============ Sending ============

    /// Send direct message to a specific peer
    async fn send_direct(
        &mut self,
        peer_id: &PeerId,
        data: Vec<u8>,
    ) -> Result<(), ModuleError>;

    /// Send routing information to a specific peer
    async fn send_routing_info(
        &mut self,
        peer_id: &PeerId,
        data: Vec<u8>,
    ) -> Result<(), ModuleError>;

    /// Broadcast message to all peers (if supported)
    async fn broadcast(
        &mut self,
        topic: &str,
        data: Vec<u8>,
    ) -> Result<(), ModuleError>;

    // ============ Connection Management ============

    /// Dial a specific peer address (if supported)
    async fn dial(&mut self, address: &str) -> Result<(), ModuleError>;
}
```

### 5.2 Module Registry

Manages module lifecycle and provides access to all registered modules:

```rust
// src/connections/registry.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::traits::*;

/// Handle to a registered module
pub type ModuleHandle = Arc<RwLock<dyn ConnectionTransport>>;

/// Information about a registered module
struct RegisteredModule {
    /// The module instance
    module: ModuleHandle,
    /// Whether the module is currently enabled
    enabled: bool,
    /// Priority for routing (lower = higher priority)
    priority: u8,
}

/// Central registry for all connection modules
pub struct ModuleRegistry {
    /// Registered modules by ID
    modules: HashMap<String, RegisteredModule>,
    /// Channel for broadcasting events
    event_tx: tokio::sync::mpsc::Sender<(String, ModuleEvent)>,
}

impl ModuleRegistry {
    /// Create a new registry
    /// Returns the registry and a receiver for module events
    pub fn new() -> (Self, tokio::sync::mpsc::Receiver<(String, ModuleEvent)>) {
        let (tx, rx) = tokio::sync::mpsc::channel(1024);
        (
            Self {
                modules: HashMap::new(),
                event_tx: tx,
            },
            rx,
        )
    }

    /// Register a new module
    /// The module is initialized but not started
    pub async fn register<T: ConnectionTransport + 'static>(
        &mut self,
        module: T,
        config: ModuleConfig,
        priority: u8,
    ) -> Result<(), ModuleError> {
        let id = module.module_id().to_string();

        if self.modules.contains_key(&id) {
            return Err(ModuleError::Internal(
                format!("Module already registered: {}", id)
            ));
        }

        let mut module = module;
        module.init(config).await?;

        self.modules.insert(
            id.clone(),
            RegisteredModule {
                module: Arc::new(RwLock::new(module)),
                enabled: false,
                priority,
            },
        );

        log::info!("Registered connection module: {}", id);
        Ok(())
    }

    /// Unregister a module
    /// Stops the module if it's running
    pub async fn unregister(&mut self, module_id: &str) -> Result<(), ModuleError> {
        if let Some(reg) = self.modules.remove(module_id) {
            let mut module = reg.module.write().await;
            if module.status() == ModuleStatus::Running {
                module.stop().await?;
            }
            log::info!("Unregistered connection module: {}", module_id);
        }
        Ok(())
    }

    /// Activate (start) a module
    pub async fn activate(&mut self, module_id: &str) -> Result<(), ModuleError> {
        let reg = self.modules.get_mut(module_id).ok_or_else(|| {
            ModuleError::Internal(format!("Module not found: {}", module_id))
        })?;

        let mut module = reg.module.write().await;
        module.start().await?;
        reg.enabled = true;

        log::info!("Activated connection module: {}", module_id);
        Ok(())
    }

    /// Deactivate (stop) a module
    /// The module remains registered and can be reactivated
    pub async fn deactivate(&mut self, module_id: &str) -> Result<(), ModuleError> {
        let reg = self.modules.get_mut(module_id).ok_or_else(|| {
            ModuleError::Internal(format!("Module not found: {}", module_id))
        })?;

        let mut module = reg.module.write().await;
        module.stop().await?;
        reg.enabled = false;

        log::info!("Deactivated connection module: {}", module_id);
        Ok(())
    }

    /// Check if a module is active
    pub fn is_active(&self, module_id: &str) -> bool {
        self.modules
            .get(module_id)
            .map(|r| r.enabled)
            .unwrap_or(false)
    }

    /// Get all active modules
    pub fn active_modules(&self) -> Vec<(String, ModuleHandle)> {
        self.modules
            .iter()
            .filter(|(_, reg)| reg.enabled)
            .map(|(id, reg)| (id.clone(), reg.module.clone()))
            .collect()
    }

    /// Get all registered modules with their status
    pub async fn list_modules(&self) -> Vec<ModuleInfo> {
        let mut result = Vec::new();

        for (id, reg) in &self.modules {
            let module = reg.module.read().await;
            result.push(ModuleInfo {
                id: id.clone(),
                display_name: module.display_name().to_string(),
                status: module.status(),
                enabled: reg.enabled,
                priority: reg.priority,
                capabilities: module.capabilities(),
            });
        }

        result
    }

    /// Get a specific module by ID
    pub fn get(&self, module_id: &str) -> Option<ModuleHandle> {
        self.modules.get(module_id).map(|r| r.module.clone())
    }

    /// Get module priority
    pub fn priority(&self, module_id: &str) -> Option<u8> {
        self.modules.get(module_id).map(|r| r.priority)
    }

    /// Poll all active modules for events
    pub async fn poll_all(&self) -> Vec<(String, ModuleEvent)> {
        let mut all_events = Vec::new();

        for (id, reg) in &self.modules {
            if reg.enabled {
                let mut module = reg.module.write().await;
                let events = module.poll_events().await;
                for event in events {
                    all_events.push((id.clone(), event));
                }
            }
        }

        all_events
    }

    /// Broadcast to all active modules that support it
    pub async fn broadcast_all(
        &self,
        topic: &str,
        data: Vec<u8>,
    ) -> Vec<(String, Result<(), ModuleError>)> {
        let mut results = Vec::new();

        for (id, reg) in &self.modules {
            if reg.enabled {
                let mut module = reg.module.write().await;
                if module.capabilities().supports_broadcast {
                    let result = module.broadcast(topic, data.clone()).await;
                    results.push((id.clone(), result));
                }
            }
        }

        results
    }

    /// Send direct message via best available module
    pub async fn send_direct_best(
        &self,
        peer_id: &PeerId,
        data: Vec<u8>,
        preferred_module: Option<&str>,
    ) -> Result<String, ModuleError> {
        // Try preferred module first
        if let Some(module_id) = preferred_module {
            if let Some(reg) = self.modules.get(module_id) {
                if reg.enabled {
                    let mut module = reg.module.write().await;
                    if module.neighbours().iter().any(|n| &n.peer_id == peer_id) {
                        module.send_direct(peer_id, data).await?;
                        return Ok(module_id.to_string());
                    }
                }
            }
        }

        // Find any module that can reach this peer
        for (id, reg) in &self.modules {
            if reg.enabled {
                let mut module = reg.module.write().await;
                if module.neighbours().iter().any(|n| &n.peer_id == peer_id) {
                    module.send_direct(peer_id, data).await?;
                    return Ok(id.clone());
                }
            }
        }

        Err(ModuleError::ConnectionFailed(
            format!("No module can reach peer: {:?}", peer_id)
        ))
    }
}

/// Information about a module for listing
#[derive(Clone, Debug)]
pub struct ModuleInfo {
    pub id: String,
    pub display_name: String,
    pub status: ModuleStatus,
    pub enabled: bool,
    pub priority: u8,
    pub capabilities: ModuleCapabilities,
}
```

### 5.3 Unified Neighbour Table

Aggregates peer information from all modules:

```rust
// src/router/unified_neighbours.rs

use std::collections::HashMap;
use libp2p::PeerId;

/// A neighbour as seen across all modules
#[derive(Clone, Debug)]
pub struct UnifiedNeighbour {
    /// The peer's ID
    pub peer_id: PeerId,
    /// RTT per module (module_id -> rtt_micros)
    pub rtt_by_module: HashMap<String, u32>,
    /// Last seen timestamp per module
    pub last_seen: HashMap<String, u64>,
}

impl UnifiedNeighbour {
    /// Get the best (lowest RTT) module for reaching this peer
    pub fn best_module(&self) -> Option<(&str, u32)> {
        self.rtt_by_module
            .iter()
            .min_by_key(|(_, rtt)| *rtt)
            .map(|(module, rtt)| (module.as_str(), *rtt))
    }

    /// Get all modules that can reach this peer
    pub fn available_modules(&self) -> Vec<&str> {
        self.rtt_by_module.keys().map(|s| s.as_str()).collect()
    }

    /// Get the lowest RTT across all modules
    pub fn best_rtt(&self) -> Option<u32> {
        self.rtt_by_module.values().min().copied()
    }

    /// Check if peer is reachable via a specific module
    pub fn reachable_via(&self, module_id: &str) -> bool {
        self.rtt_by_module.contains_key(module_id)
    }
}

/// Unified table of all neighbours across all modules
pub struct UnifiedNeighbourTable {
    /// All known neighbours
    neighbours: HashMap<PeerId, UnifiedNeighbour>,
    /// Timeout for stale entries in milliseconds
    timeout_ms: u64,
}

impl UnifiedNeighbourTable {
    /// Create a new table with specified timeout
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            neighbours: HashMap::new(),
            timeout_ms,
        }
    }

    /// Update neighbour info from a module
    pub fn update(&mut self, module_id: &str, peer_id: PeerId, rtt_micros: u32) {
        let now = crate::utilities::timestamp::Timestamp::get_timestamp();

        let neighbour = self.neighbours.entry(peer_id).or_insert_with(|| {
            UnifiedNeighbour {
                peer_id,
                rtt_by_module: HashMap::new(),
                last_seen: HashMap::new(),
            }
        });

        neighbour.rtt_by_module.insert(module_id.to_string(), rtt_micros);
        neighbour.last_seen.insert(module_id.to_string(), now);
    }

    /// Remove a peer from a specific module
    pub fn remove_from_module(&mut self, module_id: &str, peer_id: &PeerId) {
        if let Some(neighbour) = self.neighbours.get_mut(peer_id) {
            neighbour.rtt_by_module.remove(module_id);
            neighbour.last_seen.remove(module_id);

            // Remove entirely if no modules can reach this peer
            if neighbour.rtt_by_module.is_empty() {
                self.neighbours.remove(peer_id);
            }
        }
    }

    /// Remove all peers from a module (called when module is deactivated)
    pub fn remove_all_from_module(&mut self, module_id: &str) {
        let peers_to_check: Vec<PeerId> = self.neighbours.keys().cloned().collect();

        for peer_id in peers_to_check {
            self.remove_from_module(module_id, &peer_id);
        }
    }

    /// Clean up stale entries
    pub fn cleanup_stale(&mut self) {
        let now = crate::utilities::timestamp::Timestamp::get_timestamp();
        let timeout = self.timeout_ms;

        self.neighbours.retain(|_, neighbour| {
            // Remove stale module entries
            neighbour.last_seen.retain(|_, last| {
                now.saturating_sub(*last) < timeout
            });

            // Keep only RTT entries for modules we still have last_seen for
            neighbour.rtt_by_module.retain(|module, _| {
                neighbour.last_seen.contains_key(module)
            });

            // Keep the neighbour if any module can still reach them
            !neighbour.rtt_by_module.is_empty()
        });
    }

    /// Get best route to a peer
    pub fn best_route(&self, peer_id: &PeerId) -> Option<(String, u32)> {
        self.neighbours
            .get(peer_id)
            .and_then(|n| n.best_module())
            .map(|(m, r)| (m.to_string(), r))
    }

    /// Get all neighbours
    pub fn all(&self) -> impl Iterator<Item = &UnifiedNeighbour> {
        self.neighbours.values()
    }

    /// Get a specific neighbour
    pub fn get(&self, peer_id: &PeerId) -> Option<&UnifiedNeighbour> {
        self.neighbours.get(peer_id)
    }

    /// Check if peer is reachable via any module
    pub fn is_neighbour(&self, peer_id: &PeerId) -> bool {
        self.neighbours.contains_key(peer_id)
    }

    /// Get all peers reachable only via a specific module
    pub fn exclusive_to_module(&self, module_id: &str) -> Vec<PeerId> {
        self.neighbours
            .values()
            .filter(|n| {
                n.rtt_by_module.len() == 1 && n.rtt_by_module.contains_key(module_id)
            })
            .map(|n| n.peer_id)
            .collect()
    }

    /// Get statistics
    pub fn stats(&self) -> NeighbourStats {
        let mut by_module: HashMap<String, usize> = HashMap::new();

        for neighbour in self.neighbours.values() {
            for module_id in neighbour.rtt_by_module.keys() {
                *by_module.entry(module_id.clone()).or_insert(0) += 1;
            }
        }

        NeighbourStats {
            total_neighbours: self.neighbours.len(),
            by_module,
        }
    }
}

/// Statistics about the neighbour table
#[derive(Clone, Debug)]
pub struct NeighbourStats {
    pub total_neighbours: usize,
    pub by_module: HashMap<String, usize>,
}
```

### 5.4 Event Dispatcher

Routes events from modules to appropriate handlers:

```rust
// src/connections/dispatcher.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use super::traits::*;
use crate::router::unified_neighbours::UnifiedNeighbourTable;

/// Dispatches events from modules to appropriate handlers
pub struct EventDispatcher {
    /// Unified neighbour table
    neighbours: Arc<RwLock<UnifiedNeighbourTable>>,
}

impl EventDispatcher {
    /// Create a new dispatcher
    pub fn new(neighbours: Arc<RwLock<UnifiedNeighbourTable>>) -> Self {
        Self { neighbours }
    }

    /// Process an event from a module
    pub async fn process(&self, module_id: &str, event: ModuleEvent) {
        match event {
            ModuleEvent::PeerConnected { peer_id, rtt } => {
                log::debug!("[{}] Peer connected: {:?}", module_id, peer_id);

                if let Some(rtt) = rtt {
                    let mut neighbours = self.neighbours.write().await;
                    neighbours.update(module_id, peer_id, rtt);
                }

                // Notify router about new peer
                crate::router::info::RouterInfo::peer_connected(peer_id, module_id);
            }

            ModuleEvent::PeerDisconnected { peer_id } => {
                log::debug!("[{}] Peer disconnected: {:?}", module_id, peer_id);

                let mut neighbours = self.neighbours.write().await;
                neighbours.remove_from_module(module_id, &peer_id);

                // Notify router about peer loss
                crate::router::info::RouterInfo::peer_disconnected(peer_id, module_id);
            }

            ModuleEvent::RttUpdated { peer_id, rtt_micros } => {
                let mut neighbours = self.neighbours.write().await;
                neighbours.update(module_id, peer_id, rtt_micros);
            }

            ModuleEvent::RoutingInfoReceived { peer_id, data } => {
                // Forward to router for processing
                crate::router::info::RouterInfo::received(
                    peer_id,
                    module_id.to_string(),
                    data,
                );
            }

            ModuleEvent::MessageReceived { peer_id, data } => {
                // Forward to messaging service
                crate::services::messaging::Messaging::received(
                    peer_id,
                    module_id.to_string(),
                    data,
                );
            }

            ModuleEvent::BroadcastReceived { topic, data } => {
                // Forward to feed/flooding service
                crate::services::feed::Feed::received(topic, data);
            }

            ModuleEvent::StatusChanged { status } => {
                log::info!("[{}] Module status changed: {:?}", module_id, status);

                // If module stopped, clean up its neighbours
                if status == ModuleStatus::Stopped {
                    let mut neighbours = self.neighbours.write().await;
                    neighbours.remove_all_from_module(module_id);
                }
            }

            ModuleEvent::Custom { event_type, data } => {
                log::debug!(
                    "[{}] Custom event: {} ({} bytes)",
                    module_id,
                    event_type,
                    data.len()
                );
            }
        }
    }

    /// Process multiple events
    pub async fn process_batch(&self, events: Vec<(String, ModuleEvent)>) {
        for (module_id, event) in events {
            self.process(&module_id, event).await;
        }
    }
}
```

---

## 6. Module Implementation Guide

### 6.1 Creating a New Module

To add a new connection module:

1. Create a new file in `src/connections/modules/`
2. Implement the `ConnectionTransport` trait
3. Register the module in the startup code

### 6.2 Module Template

```rust
// src/connections/modules/my_module.rs

use async_trait::async_trait;
use libp2p::PeerId;
use std::collections::HashMap;
use crate::connections::traits::*;

pub struct MyModule {
    status: ModuleStatus,
    config: Option<ModuleConfig>,
    pending_events: Vec<ModuleEvent>,
    // Module-specific state...
}

impl MyModule {
    pub fn new() -> Self {
        Self {
            status: ModuleStatus::Uninitialized,
            config: None,
            pending_events: Vec::new(),
        }
    }

    fn emit_event(&mut self, event: ModuleEvent) {
        self.pending_events.push(event);
    }
}

#[async_trait]
impl ConnectionTransport for MyModule {
    fn module_id(&self) -> &'static str {
        "my_module"
    }

    fn display_name(&self) -> &str {
        "My Custom Module"
    }

    fn capabilities(&self) -> ModuleCapabilities {
        ModuleCapabilities {
            supports_broadcast: false,
            supports_direct: true,
            supports_discovery: false,
            requires_bootstrap: true,
            is_local_only: false,
            supports_nat_traversal: false,
            mobile_compatible: true,
        }
    }

    fn status(&self) -> ModuleStatus {
        self.status.clone()
    }

    async fn init(&mut self, config: ModuleConfig) -> Result<(), ModuleError> {
        self.config = Some(config);
        self.status = ModuleStatus::Stopped;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), ModuleError> {
        self.status = ModuleStatus::Starting;

        // Initialize connections, start listeners, etc.

        self.status = ModuleStatus::Running;
        self.emit_event(ModuleEvent::StatusChanged {
            status: ModuleStatus::Running,
        });

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), ModuleError> {
        self.status = ModuleStatus::Stopping;

        // Close connections, stop listeners, etc.

        self.status = ModuleStatus::Stopped;
        self.emit_event(ModuleEvent::StatusChanged {
            status: ModuleStatus::Stopped,
        });

        Ok(())
    }

    async fn poll_events(&mut self) -> Vec<ModuleEvent> {
        // Also poll internal event sources here
        std::mem::take(&mut self.pending_events)
    }

    fn neighbours(&self) -> Vec<NeighbourInfo> {
        // Return list of connected peers
        Vec::new()
    }

    fn listening_addresses(&self) -> Vec<String> {
        Vec::new()
    }

    async fn send_direct(
        &mut self,
        peer_id: &PeerId,
        data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        // Send data to peer
        Ok(())
    }

    async fn send_routing_info(
        &mut self,
        peer_id: &PeerId,
        data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        self.send_direct(peer_id, data).await
    }

    async fn broadcast(
        &mut self,
        _topic: &str,
        _data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        Err(ModuleError::NotSupported(
            "This module does not support broadcast".into()
        ))
    }

    async fn dial(&mut self, address: &str) -> Result<(), ModuleError> {
        // Parse address and connect
        Ok(())
    }
}
```

### 6.3 Wrapping Existing libp2p Modules

For modules based on libp2p swarms (like LAN and Internet):

```rust
// src/connections/modules/lan.rs

use async_trait::async_trait;
use libp2p::{swarm::Swarm, PeerId};
use crate::connections::traits::*;

pub struct LanModule {
    status: ModuleStatus,
    swarm: Option<Swarm<QaulLanBehaviour>>,
    pending_events: Vec<ModuleEvent>,
    config: Option<ModuleConfig>,
}

#[async_trait]
impl ConnectionTransport for LanModule {
    fn module_id(&self) -> &'static str {
        "lan"
    }

    fn display_name(&self) -> &str {
        "LAN (mDNS Discovery)"
    }

    fn capabilities(&self) -> ModuleCapabilities {
        ModuleCapabilities {
            supports_broadcast: true,   // via floodsub
            supports_direct: true,
            supports_discovery: true,   // via mDNS
            requires_bootstrap: false,
            is_local_only: true,
            supports_nat_traversal: false,
            mobile_compatible: false,
        }
    }

    async fn poll_events(&mut self) -> Vec<ModuleEvent> {
        let mut events = std::mem::take(&mut self.pending_events);

        // Poll the swarm for new events
        if let Some(swarm) = &mut self.swarm {
            while let Some(event) = swarm.select_next_some().now_or_never() {
                match event {
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        events.push(ModuleEvent::PeerConnected {
                            peer_id,
                            rtt: None,
                        });
                    }
                    SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        events.push(ModuleEvent::PeerDisconnected { peer_id });
                    }
                    SwarmEvent::Behaviour(behaviour_event) => {
                        // Convert behaviour events to ModuleEvents
                        self.process_behaviour_event(behaviour_event, &mut events);
                    }
                    _ => {}
                }
            }
        }

        events
    }

    // ... other trait methods ...
}

impl LanModule {
    fn process_behaviour_event(
        &mut self,
        event: QaulLanEvent,
        events: &mut Vec<ModuleEvent>,
    ) {
        match event {
            QaulLanEvent::Ping(ping::Event { peer, result, .. }) => {
                if let Ok(ping::Success::Ping { rtt }) = result {
                    events.push(ModuleEvent::RttUpdated {
                        peer_id: peer,
                        rtt_micros: rtt.as_micros() as u32,
                    });
                }
            }
            QaulLanEvent::QaulInfo(info_event) => {
                if let QaulInfoEvent::Received { peer_id, data } = info_event {
                    events.push(ModuleEvent::RoutingInfoReceived {
                        peer_id,
                        data,
                    });
                }
            }
            QaulLanEvent::QaulMessaging(msg_event) => {
                if let QaulMessagingEvent::Received { peer_id, data } = msg_event {
                    events.push(ModuleEvent::MessageReceived {
                        peer_id,
                        data,
                    });
                }
            }
            QaulLanEvent::Floodsub(FloodsubEvent::Message(msg)) => {
                events.push(ModuleEvent::BroadcastReceived {
                    topic: msg.topics.first()
                        .map(|t| t.to_string())
                        .unwrap_or_default(),
                    data: msg.data,
                });
            }
            _ => {}
        }
    }
}
```

---

## 7. Routing Integration

### 7.1 Modified RouterInfo

The router needs to be updated to work with module IDs instead of the enum:

```rust
// src/router/info.rs (modified)

impl RouterInfo {
    /// Receive routing info from a peer via a module
    pub fn received(peer_id: PeerId, module_id: String, data: Vec<u8>) {
        // Process routing table update
        // Update scheduler to avoid sending via this module to same peer
    }

    /// Check if routing info should be sent
    pub fn check_scheduler() -> Option<(PeerId, String, Vec<u8>)> {
        // Return (peer_id, module_id, routing_data)
        // The module_id tells which module to use for sending
    }

    /// Notify about new peer connection
    pub fn peer_connected(peer_id: PeerId, module_id: &str) {
        // Schedule sending routing table to new peer
    }

    /// Notify about peer disconnection
    pub fn peer_disconnected(peer_id: PeerId, module_id: &str) {
        // Remove peer from scheduler
        // Update routing table entries
    }
}
```

### 7.2 Routing Table Updates

The routing table stores module IDs as strings:

```rust
// src/router/table.rs (modified)

pub struct RoutingConnectionEntry {
    /// Module ID (e.g., "lan", "internet", "iroh")
    pub module_id: String,
    /// Next-hop peer
    pub node: PeerId,
    /// Round-trip time in microseconds
    pub rtt: u32,
    /// Hop count
    pub hc: u8,
    /// Link quality (0-100)
    pub lq: u32,
    /// Last update timestamp
    pub last_update: u64,
}

impl RoutingTable {
    /// Get best connection for a user
    pub fn best_connection(&self, user_id: &[u8]) -> Option<&RoutingConnectionEntry> {
        self.table
            .get(user_id)
            .and_then(|entry| {
                entry.connections
                    .iter()
                    .min_by_key(|c| c.rtt + (c.hc as u32 * 10000))
            })
    }

    /// Get all connections for a user, sorted by quality
    pub fn connections_for_user(&self, user_id: &[u8]) -> Vec<&RoutingConnectionEntry> {
        self.table
            .get(user_id)
            .map(|entry| {
                let mut conns: Vec<_> = entry.connections.iter().collect();
                conns.sort_by_key(|c| c.rtt + (c.hc as u32 * 10000));
                conns
            })
            .unwrap_or_default()
    }
}
```

---

## 8. Configuration System

### 8.1 Configuration File Format

```yaml
# config.yaml

# Global node settings
node:
  name: "My qaul Node"

# Connection modules configuration
modules:
  lan:
    enabled: true
    priority: 1
    settings:
      listen:
        - "/ip4/0.0.0.0/udp/0/quic-v1"
        - "/ip4/0.0.0.0/tcp/0"
        - "/ip6/::/udp/0/quic-v1"
        - "/ip6/::/tcp/0"

  internet:
    enabled: true
    priority: 2
    settings:
      peers:
        - address: "/ip4/144.91.74.192/udp/9229/quic-v1"
          name: "qaul Community Node [IPv4]"
          enabled: true
        - address: "/ip6/2a02:c207:2080:6427::1/udp/9229/quic-v1"
          name: "qaul Community Node [IPv6]"
          enabled: true
      do_listen: false
      listen:
        - "/ip4/0.0.0.0/udp/9229/quic-v1"
        - "/ip4/0.0.0.0/tcp/9229"

  ble:
    enabled: false  # Only enable on mobile
    priority: 3
    settings:
      scan_interval_ms: 5000
      advertise: true

  iroh:
    enabled: false
    priority: 4
    settings:
      relay_nodes:
        - "relay.iroh.network"
      discovery: true

  nostr:
    enabled: false
    priority: 5
    settings:
      relays:
        - "wss://relay.qaul.net"
        - "wss://relay.damus.io"
      publish_interval_ms: 30000

  https:
    enabled: false
    priority: 6
    settings:
      endpoints:
        - "https://bridge.qaul.net/api"
      poll_interval_ms: 10000
```

### 8.2 Configuration Loading

```rust
// src/configuration/modules.rs

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModulesConfig {
    #[serde(flatten)]
    pub modules: HashMap<String, ModuleConfigEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModuleConfigEntry {
    pub enabled: bool,
    pub priority: u8,
    #[serde(default)]
    pub settings: HashMap<String, toml::Value>,
}

impl ModulesConfig {
    /// Get enabled modules sorted by priority
    pub fn enabled_modules(&self) -> Vec<(&str, &ModuleConfigEntry)> {
        let mut modules: Vec<_> = self.modules
            .iter()
            .filter(|(_, config)| config.enabled)
            .map(|(id, config)| (id.as_str(), config))
            .collect();

        modules.sort_by_key(|(_, config)| config.priority);
        modules
    }
}
```

---

## 9. RPC Interface

### 9.1 Module Management RPC

```protobuf
// modules.proto

syntax = "proto3";
package qaul.rpc.modules;

// Request to list all modules
message ListModulesRequest {}

// Response with module list
message ListModulesResponse {
    repeated ModuleInfo modules = 1;
}

// Information about a module
message ModuleInfo {
    string id = 1;
    string display_name = 2;
    ModuleStatus status = 3;
    bool enabled = 4;
    uint32 priority = 5;
    ModuleCapabilities capabilities = 6;
    repeated string listening_addresses = 7;
    uint32 neighbour_count = 8;
}

// Module status
enum ModuleStatus {
    UNINITIALIZED = 0;
    STARTING = 1;
    RUNNING = 2;
    STOPPING = 3;
    STOPPED = 4;
    ERROR = 5;
}

// Module capabilities
message ModuleCapabilities {
    bool supports_broadcast = 1;
    bool supports_direct = 2;
    bool supports_discovery = 3;
    bool requires_bootstrap = 4;
    bool is_local_only = 5;
    bool supports_nat_traversal = 6;
    bool mobile_compatible = 7;
}

// Request to activate a module
message ActivateModuleRequest {
    string module_id = 1;
}

// Request to deactivate a module
message DeactivateModuleRequest {
    string module_id = 1;
}

// Generic response
message ModuleOperationResponse {
    bool success = 1;
    string error_message = 2;
}

// Get neighbours for a module
message GetModuleNeighboursRequest {
    string module_id = 1;
}

message GetModuleNeighboursResponse {
    repeated NeighbourInfo neighbours = 1;
}

message NeighbourInfo {
    bytes peer_id = 1;
    uint32 rtt_micros = 2;
    uint64 connected_at = 3;
}
```

### 9.2 RPC Handler

```rust
// src/rpc/modules.rs

use crate::connections::registry::ModuleRegistry;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ModulesRpc {
    registry: Arc<RwLock<ModuleRegistry>>,
}

impl ModulesRpc {
    pub fn new(registry: Arc<RwLock<ModuleRegistry>>) -> Self {
        Self { registry }
    }

    /// List all registered modules
    pub async fn list_modules(&self) -> ListModulesResponse {
        let registry = self.registry.read().await;
        let modules = registry.list_modules().await;

        ListModulesResponse {
            modules: modules.into_iter().map(|m| m.into()).collect(),
        }
    }

    /// Activate a module
    pub async fn activate_module(
        &self,
        module_id: &str,
    ) -> ModuleOperationResponse {
        let mut registry = self.registry.write().await;

        match registry.activate(module_id).await {
            Ok(()) => ModuleOperationResponse {
                success: true,
                error_message: String::new(),
            },
            Err(e) => ModuleOperationResponse {
                success: false,
                error_message: e.to_string(),
            },
        }
    }

    /// Deactivate a module
    pub async fn deactivate_module(
        &self,
        module_id: &str,
    ) -> ModuleOperationResponse {
        let mut registry = self.registry.write().await;

        match registry.deactivate(module_id).await {
            Ok(()) => ModuleOperationResponse {
                success: true,
                error_message: String::new(),
            },
            Err(e) => ModuleOperationResponse {
                success: false,
                error_message: e.to_string(),
            },
        }
    }

    /// Get neighbours for a specific module
    pub async fn get_module_neighbours(
        &self,
        module_id: &str,
    ) -> GetModuleNeighboursResponse {
        let registry = self.registry.read().await;

        if let Some(module) = registry.get(module_id) {
            let module = module.read().await;
            let neighbours = module.neighbours();

            GetModuleNeighboursResponse {
                neighbours: neighbours.into_iter().map(|n| n.into()).collect(),
            }
        } else {
            GetModuleNeighboursResponse {
                neighbours: Vec::new(),
            }
        }
    }
}
```

---

## 10. Migration Plan

### Phase 1: Foundation (Non-Breaking)

**Goal**: Create the new infrastructure without modifying existing code.

**Tasks**:
1. Create `src/connections/traits.rs` with `ConnectionTransport` trait
2. Create `src/connections/registry.rs` with `ModuleRegistry`
3. Create `src/router/unified_neighbours.rs` with `UnifiedNeighbourTable`
4. Create `src/connections/dispatcher.rs` with `EventDispatcher`
5. Add new dependencies to `Cargo.toml` (`async-trait`, `thiserror`)

**Files Created**:
- `src/connections/traits.rs`
- `src/connections/registry.rs`
- `src/connections/dispatcher.rs`
- `src/router/unified_neighbours.rs`

### Phase 2: Wrapper Modules

**Goal**: Wrap existing modules to implement the new trait.

**Tasks**:
1. Create `src/connections/modules/` directory
2. Create `LanModule` wrapper implementing `ConnectionTransport`
3. Create `InternetModule` wrapper implementing `ConnectionTransport`
4. Create `BleModule` wrapper implementing `ConnectionTransport`
5. Ensure all existing functionality is preserved

**Files Created**:
- `src/connections/modules/mod.rs`
- `src/connections/modules/lan.rs`
- `src/connections/modules/internet.rs`
- `src/connections/modules/ble.rs`

### Phase 3: Parallel Infrastructure

**Goal**: Run new and old systems in parallel for validation.

**Tasks**:
1. Create alternate startup path using new registry
2. Add feature flag to switch between old and new systems
3. Validate that both systems produce identical behaviour
4. Add comprehensive logging for debugging

**Changes**:
- `src/lib.rs`: Add feature-flagged alternate startup
- `Cargo.toml`: Add feature flag

### Phase 4: Router Integration

**Goal**: Update router to use unified neighbour table.

**Tasks**:
1. Update `RouterInfo` to accept module IDs as strings
2. Update `RoutingTable` connection entries
3. Migrate routing scheduler to new system
4. Update message sending to use registry

**Files Modified**:
- `src/router/info.rs`
- `src/router/table.rs`
- `src/router/connections.rs`

### Phase 5: Full Migration

**Goal**: Remove old code and make new system default.

**Tasks**:
1. Update main event loop to use new architecture
2. Remove old `ConnectionModule` enum usages
3. Remove old per-module neighbour tables
4. Update RPC handlers for module management
5. Update configuration system

**Files Modified**:
- `src/lib.rs`
- `src/connections/mod.rs`
- `src/rpc/*.rs`
- `src/configuration/*.rs`

### Phase 6: New Modules

**Goal**: Demonstrate extensibility with new modules.

**Tasks**:
1. Implement `IrohModule`
2. Implement `NostrModule`
3. Implement `HttpsModule`
4. Add configuration support for new modules
5. Write documentation and examples

**Files Created**:
- `src/connections/modules/iroh.rs`
- `src/connections/modules/nostr.rs`
- `src/connections/modules/https.rs`

---

## 11. Example Implementations

### 11.1 Iroh Module

```rust
// src/connections/modules/iroh.rs

use async_trait::async_trait;
use iroh::{Endpoint, NodeId, SecretKey};
use libp2p::PeerId;
use std::collections::HashMap;
use crate::connections::traits::*;

pub struct IrohModule {
    status: ModuleStatus,
    endpoint: Option<Endpoint>,
    connections: HashMap<NodeId, iroh::endpoint::Connection>,
    pending_events: Vec<ModuleEvent>,
    config: Option<ModuleConfig>,
}

impl IrohModule {
    pub fn new() -> Self {
        Self {
            status: ModuleStatus::Uninitialized,
            endpoint: None,
            connections: HashMap::new(),
            pending_events: Vec::new(),
            config: None,
        }
    }

    /// Convert libp2p PeerId to Iroh NodeId
    fn peer_to_node_id(peer_id: &PeerId) -> Option<NodeId> {
        // Implementation depends on key format compatibility
        None
    }

    /// Convert Iroh NodeId to libp2p PeerId
    fn node_to_peer_id(node_id: &NodeId) -> PeerId {
        // Implementation depends on key format compatibility
        PeerId::random()
    }
}

#[async_trait]
impl ConnectionTransport for IrohModule {
    fn module_id(&self) -> &'static str {
        "iroh"
    }

    fn display_name(&self) -> &str {
        "Iroh (QUIC Relay)"
    }

    fn capabilities(&self) -> ModuleCapabilities {
        ModuleCapabilities {
            supports_broadcast: false,
            supports_direct: true,
            supports_discovery: true,  // via relay nodes
            requires_bootstrap: true,
            is_local_only: false,
            supports_nat_traversal: true,  // Iroh handles this
            mobile_compatible: true,
        }
    }

    fn status(&self) -> ModuleStatus {
        self.status.clone()
    }

    async fn init(&mut self, config: ModuleConfig) -> Result<(), ModuleError> {
        self.config = Some(config);
        self.status = ModuleStatus::Stopped;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), ModuleError> {
        self.status = ModuleStatus::Starting;

        let config = self.config.as_ref()
            .ok_or(ModuleError::NotInitialized)?;

        // Create Iroh endpoint
        let secret_key = SecretKey::generate();
        let endpoint = Endpoint::builder()
            .secret_key(secret_key)
            .bind()
            .await
            .map_err(|e| ModuleError::Internal(e.to_string()))?;

        self.endpoint = Some(endpoint);
        self.status = ModuleStatus::Running;

        self.pending_events.push(ModuleEvent::StatusChanged {
            status: ModuleStatus::Running,
        });

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), ModuleError> {
        self.status = ModuleStatus::Stopping;

        // Close all connections
        for (node_id, conn) in self.connections.drain() {
            conn.close(0u8.into(), b"shutdown");

            self.pending_events.push(ModuleEvent::PeerDisconnected {
                peer_id: Self::node_to_peer_id(&node_id),
            });
        }

        // Close endpoint
        if let Some(endpoint) = self.endpoint.take() {
            endpoint.close().await
                .map_err(|e| ModuleError::Internal(e.to_string()))?;
        }

        self.status = ModuleStatus::Stopped;
        self.pending_events.push(ModuleEvent::StatusChanged {
            status: ModuleStatus::Stopped,
        });

        Ok(())
    }

    async fn poll_events(&mut self) -> Vec<ModuleEvent> {
        // Poll endpoint for incoming connections
        if let Some(endpoint) = &self.endpoint {
            // Non-blocking check for new connections
            // Add connection events to pending_events
        }

        std::mem::take(&mut self.pending_events)
    }

    fn neighbours(&self) -> Vec<NeighbourInfo> {
        self.connections.keys()
            .map(|node_id| NeighbourInfo {
                peer_id: Self::node_to_peer_id(node_id),
                rtt_micros: 0,
                connected_at: 0,
                metadata: HashMap::new(),
            })
            .collect()
    }

    fn listening_addresses(&self) -> Vec<String> {
        self.endpoint.as_ref()
            .map(|e| vec![e.node_id().to_string()])
            .unwrap_or_default()
    }

    async fn send_direct(
        &mut self,
        peer_id: &PeerId,
        data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        let node_id = Self::peer_to_node_id(peer_id)
            .ok_or(ModuleError::Internal("Invalid peer ID".into()))?;

        let conn = self.connections.get(&node_id)
            .ok_or(ModuleError::ConnectionFailed("Not connected".into()))?;

        let mut send = conn.open_uni().await
            .map_err(|e| ModuleError::ConnectionFailed(e.to_string()))?;

        send.write_all(&data).await
            .map_err(|e| ModuleError::ConnectionFailed(e.to_string()))?;

        send.finish()
            .map_err(|e| ModuleError::ConnectionFailed(e.to_string()))?;

        Ok(())
    }

    async fn send_routing_info(
        &mut self,
        peer_id: &PeerId,
        data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        self.send_direct(peer_id, data).await
    }

    async fn broadcast(
        &mut self,
        _topic: &str,
        _data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        Err(ModuleError::NotSupported(
            "Iroh does not support broadcast".into()
        ))
    }

    async fn dial(&mut self, address: &str) -> Result<(), ModuleError> {
        let endpoint = self.endpoint.as_ref()
            .ok_or(ModuleError::NotRunning)?;

        let node_id: NodeId = address.parse()
            .map_err(|_| ModuleError::ConfigError("Invalid node ID".into()))?;

        let conn = endpoint.connect(node_id, b"qaul")
            .await
            .map_err(|e| ModuleError::ConnectionFailed(e.to_string()))?;

        let peer_id = Self::node_to_peer_id(&node_id);
        self.connections.insert(node_id, conn);

        self.pending_events.push(ModuleEvent::PeerConnected {
            peer_id,
            rtt: None,
        });

        Ok(())
    }
}
```

### 11.2 Nostr Module

```rust
// src/connections/modules/nostr.rs

use async_trait::async_trait;
use libp2p::PeerId;
use nostr_sdk::{Client, Event, EventBuilder, Keys, Kind, Tag};
use std::collections::HashMap;
use crate::connections::traits::*;

const QAUL_KIND: Kind = Kind::Custom(30078);  // Application-specific data

pub struct NostrModule {
    status: ModuleStatus,
    client: Option<Client>,
    keys: Option<Keys>,
    known_peers: HashMap<PeerId, nostr_sdk::PublicKey>,
    pending_events: Vec<ModuleEvent>,
    config: Option<ModuleConfig>,
}

impl NostrModule {
    pub fn new() -> Self {
        Self {
            status: ModuleStatus::Uninitialized,
            client: None,
            keys: None,
            known_peers: HashMap::new(),
            pending_events: Vec::new(),
            config: None,
        }
    }
}

#[async_trait]
impl ConnectionTransport for NostrModule {
    fn module_id(&self) -> &'static str {
        "nostr"
    }

    fn display_name(&self) -> &str {
        "Nostr (Relay Network)"
    }

    fn capabilities(&self) -> ModuleCapabilities {
        ModuleCapabilities {
            supports_broadcast: true,   // via relay subscription
            supports_direct: true,      // via encrypted DMs
            supports_discovery: true,   // via relay queries
            requires_bootstrap: true,   // needs relay URLs
            is_local_only: false,
            supports_nat_traversal: true,  // relays handle this
            mobile_compatible: true,
        }
    }

    fn status(&self) -> ModuleStatus {
        self.status.clone()
    }

    async fn init(&mut self, config: ModuleConfig) -> Result<(), ModuleError> {
        // Generate nostr keys from libp2p keypair
        // This ensures consistent identity
        self.config = Some(config);
        self.status = ModuleStatus::Stopped;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), ModuleError> {
        self.status = ModuleStatus::Starting;

        let config = self.config.as_ref()
            .ok_or(ModuleError::NotInitialized)?;

        // Get relay URLs from config
        let relays: Vec<String> = config.settings
            .get("relays")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        if relays.is_empty() {
            return Err(ModuleError::ConfigError(
                "No relay URLs configured".into()
            ));
        }

        // Create nostr client
        let keys = Keys::generate();
        let client = Client::new(keys.clone());

        // Connect to relays
        for relay in &relays {
            client.add_relay(relay).await
                .map_err(|e| ModuleError::ConnectionFailed(e.to_string()))?;
        }

        client.connect().await;

        self.keys = Some(keys);
        self.client = Some(client);
        self.status = ModuleStatus::Running;

        self.pending_events.push(ModuleEvent::StatusChanged {
            status: ModuleStatus::Running,
        });

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), ModuleError> {
        self.status = ModuleStatus::Stopping;

        if let Some(client) = self.client.take() {
            client.disconnect().await
                .map_err(|e| ModuleError::Internal(e.to_string()))?;
        }

        self.known_peers.clear();
        self.status = ModuleStatus::Stopped;

        self.pending_events.push(ModuleEvent::StatusChanged {
            status: ModuleStatus::Stopped,
        });

        Ok(())
    }

    async fn poll_events(&mut self) -> Vec<ModuleEvent> {
        // Poll nostr client for new events
        // Convert to ModuleEvents
        std::mem::take(&mut self.pending_events)
    }

    fn neighbours(&self) -> Vec<NeighbourInfo> {
        self.known_peers.keys()
            .map(|peer_id| NeighbourInfo {
                peer_id: *peer_id,
                rtt_micros: 0,
                connected_at: 0,
                metadata: HashMap::new(),
            })
            .collect()
    }

    fn listening_addresses(&self) -> Vec<String> {
        self.keys.as_ref()
            .map(|k| vec![k.public_key().to_string()])
            .unwrap_or_default()
    }

    async fn send_direct(
        &mut self,
        peer_id: &PeerId,
        data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        let client = self.client.as_ref()
            .ok_or(ModuleError::NotRunning)?;

        let recipient = self.known_peers.get(peer_id)
            .ok_or(ModuleError::ConnectionFailed("Unknown peer".into()))?;

        // Send as encrypted DM
        let event = EventBuilder::encrypted_direct_msg(
            self.keys.as_ref().unwrap(),
            *recipient,
            String::from_utf8_lossy(&data).to_string(),
            None,
        )
        .map_err(|e| ModuleError::Internal(e.to_string()))?;

        client.send_event(event).await
            .map_err(|e| ModuleError::ConnectionFailed(e.to_string()))?;

        Ok(())
    }

    async fn send_routing_info(
        &mut self,
        peer_id: &PeerId,
        data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        self.send_direct(peer_id, data).await
    }

    async fn broadcast(
        &mut self,
        topic: &str,
        data: Vec<u8>,
    ) -> Result<(), ModuleError> {
        let client = self.client.as_ref()
            .ok_or(ModuleError::NotRunning)?;

        // Publish as a custom kind event
        let event = EventBuilder::new(
            QAUL_KIND,
            String::from_utf8_lossy(&data).to_string(),
        )
        .tag(Tag::custom(
            nostr_sdk::TagKind::Custom("topic".into()),
            vec![topic.to_string()],
        ))
        .sign_with_keys(self.keys.as_ref().unwrap())
        .map_err(|e| ModuleError::Internal(e.to_string()))?;

        client.send_event(event).await
            .map_err(|e| ModuleError::ConnectionFailed(e.to_string()))?;

        Ok(())
    }

    async fn dial(&mut self, _address: &str) -> Result<(), ModuleError> {
        // Nostr doesn't have direct dialing
        // Peers are discovered through relay subscriptions
        Err(ModuleError::NotSupported(
            "Nostr uses relay-based discovery".into()
        ))
    }
}
```

---

## 12. Appendix: Full Code Listings

### 12.1 Updated Main Event Loop

```rust
// src/lib.rs (with new architecture)

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use futures::{select, StreamExt};
use tokio::sync::RwLock;

use crate::connections::dispatcher::EventDispatcher;
use crate::connections::registry::ModuleRegistry;
use crate::connections::modules::{LanModule, InternetModule, BleModule};
use crate::connections::traits::ModuleConfig;
use crate::router::unified_neighbours::UnifiedNeighbourTable;

pub async fn start(
    storage_path: String,
    def_config: Option<BTreeMap<String, String>>,
) {
    // ============ Initialize Core Components ============

    Storage::init(storage_path.clone());
    let node_keys = Node::init();
    Router::init();
    Services::init();

    // ============ Create Module Infrastructure ============

    let (mut registry, _event_rx) = ModuleRegistry::new();
    let neighbours = Arc::new(RwLock::new(UnifiedNeighbourTable::new(30_000)));
    let dispatcher = EventDispatcher::new(neighbours.clone());

    // ============ Load Configuration ============

    let config = Configuration::get();
    let module_config = ModuleConfig {
        node_keypair: node_keys.clone(),
        settings: std::collections::HashMap::new(),
    };

    // ============ Register and Activate Modules ============

    // LAN Module
    if config.lan.active {
        let lan = LanModule::new();
        if let Err(e) = registry.register(lan, module_config.clone(), 1).await {
            log::error!("Failed to register LAN module: {}", e);
        } else if let Err(e) = registry.activate("lan").await {
            log::error!("Failed to activate LAN module: {}", e);
        }
    }

    // Internet Module
    if config.internet.active {
        let internet = InternetModule::new();
        if let Err(e) = registry.register(internet, module_config.clone(), 2).await {
            log::error!("Failed to register Internet module: {}", e);
        } else if let Err(e) = registry.activate("internet").await {
            log::error!("Failed to activate Internet module: {}", e);
        }
    }

    // BLE Module (mobile only)
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let ble = BleModule::new();
        if let Err(e) = registry.register(ble, module_config.clone(), 3).await {
            log::error!("Failed to register BLE module: {}", e);
        } else if let Err(e) = registry.activate("ble").await {
            log::error!("Failed to activate BLE module: {}", e);
        }
    }

    // ============ Wrap Registry for Shared Access ============

    let registry = Arc::new(RwLock::new(registry));

    // ============ Initialize RPC/Sys Channels ============

    let rpc_rx = Rpc::init();
    let sys_rx = Sys::init();

    // ============ Create Periodic Timers ============

    let mut poll_ticker = Ticker::new(Duration::from_millis(10));
    let mut routing_ticker = Ticker::new(Duration::from_millis(100));
    let mut cleanup_ticker = Ticker::new(Duration::from_secs(10));
    let mut reconnect_ticker = Ticker::new(Duration::from_secs(10));

    // ============ Main Event Loop ============

    log::info!("Starting main event loop");

    loop {
        select! {
            // Poll all modules for events
            _ = poll_ticker.next().fuse() => {
                let reg = registry.read().await;
                let events = reg.poll_all().await;
                dispatcher.process_batch(events).await;
            }

            // Send routing information
            _ = routing_ticker.next().fuse() => {
                if let Some((peer_id, module_id, data)) = RouterInfo::check_scheduler() {
                    let reg = registry.read().await;
                    if let Some(module) = reg.get(&module_id) {
                        let mut m = module.write().await;
                        if let Err(e) = m.send_routing_info(&peer_id, data).await {
                            log::warn!(
                                "Failed to send routing info via {}: {}",
                                module_id,
                                e
                            );
                        }
                    }
                }
            }

            // Cleanup stale neighbours
            _ = cleanup_ticker.next().fuse() => {
                let mut n = neighbours.write().await;
                n.cleanup_stale();
            }

            // Handle reconnections (for Internet module)
            _ = reconnect_ticker.next().fuse() => {
                let reg = registry.read().await;
                if let Some(module) = reg.get("internet") {
                    let mut m = module.write().await;
                    // Trigger reconnection logic
                    // Module internally tracks disconnected peers
                }
            }

            // Process RPC messages
            rpc_message = rpc_rx.recv().fuse() => {
                if let Ok(msg) = rpc_message {
                    process_rpc_message(msg, &registry, &neighbours).await;
                }
            }

            // Process System messages
            sys_message = sys_rx.recv().fuse() => {
                if let Ok(msg) = sys_message {
                    process_sys_message(msg, &registry).await;
                }
            }
        }
    }
}

async fn process_rpc_message(
    msg: RpcMessage,
    registry: &Arc<RwLock<ModuleRegistry>>,
    neighbours: &Arc<RwLock<UnifiedNeighbourTable>>,
) {
    // Handle module management RPC commands
    match &msg.message {
        RpcMessageType::ModuleList => {
            let reg = registry.read().await;
            let modules = reg.list_modules().await;
            // Send response
        }
        RpcMessageType::ModuleActivate { module_id } => {
            let mut reg = registry.write().await;
            let result = reg.activate(module_id).await;
            // Send response
        }
        RpcMessageType::ModuleDeactivate { module_id } => {
            let mut reg = registry.write().await;
            let result = reg.deactivate(module_id).await;
            // Send response
        }
        // ... handle other RPC types ...
        _ => {
            // Forward to existing RPC handlers
        }
    }
}

async fn process_sys_message(
    msg: SysMessage,
    registry: &Arc<RwLock<ModuleRegistry>>,
) {
    // Handle system messages (e.g., from BLE native code)
    // Forward to appropriate module
}
```

---

## Summary

This architecture provides a clean, extensible system for managing multiple connection transports in libqaul. Key benefits include:

1. **Trait-Based Modularity**: New transports only need to implement `ConnectionTransport`
2. **Runtime Flexibility**: Modules can be activated/deactivated without restart
3. **Unified Routing**: Single neighbour table aggregates all transport data
4. **Capability Awareness**: System knows what each module can do
5. **Clean Separation**: Core logic is independent of specific transports
6. **Backward Compatible Migration**: Existing modules can be wrapped incrementally

The migration can be done incrementally over multiple releases, with the new and old systems running in parallel during the transition period.

