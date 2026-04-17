use libp2p::{floodsub, Multiaddr, PeerId};
use serde::{Deserialize, Serialize};

use super::ConnectionModule;

#[derive(Debug)]
pub enum TransportError {
    AlreadyRunning,
    AlreadyStopped,
    InitFailed(String),
    ShutdownFailed(String),
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::AlreadyRunning => write!(f, "transport is already running"),
            TransportError::AlreadyStopped => write!(f, "transport is already stopped"),
            TransportError::InitFailed(e) => write!(f, "transport init failed: {}", e),
            TransportError::ShutdownFailed(e) => write!(f, "transport shutdown failed: {}", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportStatus {
    Disabled,
    Starting,
    Running,
    Stopping,
    Error(String),
}

#[derive(Debug)]
pub enum TransportEvent {
    ConnectionEstablished {
        peer_id: PeerId,
        module: ConnectionModule,
    },
    ConnectionClosed {
        peer_id: PeerId,
        module: ConnectionModule,
    },
    Behaviour {
        module: ConnectionModule,
    },
}

pub struct TransportCapabilities {
    pub supports_runtime_toggle: bool,
    pub supports_peer_list: bool,
    pub is_local_only: bool,
}

pub trait Transport {
    fn id(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn module(&self) -> ConnectionModule;
    fn capabilities(&self) -> TransportCapabilities;
    fn status(&self) -> &TransportStatus;

    fn send_qaul_info_message(&mut self, peer_id: PeerId, data: Vec<u8>);
    fn send_qaul_messaging_message(&mut self, peer_id: PeerId, data: Vec<u8>);
    fn publish_floodsub(&mut self, topic: floodsub::Topic, data: Vec<u8>);

    fn listeners(&self) -> Vec<Multiaddr>;
    fn external_addresses(&self) -> Vec<Multiaddr>;
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TransportConfig {
    pub enabled: bool,
}
