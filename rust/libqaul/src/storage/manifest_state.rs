// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Host Manifest State
//!
//! Persistence for the origin's own manifest state under the
//! pull-based sync model (spec §10.8).

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use tracing::{error, info};

/// One delegation entry as it lives on disk. Matches the wire `ManifestEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationEntry {
    pub user_id: [u8; 8],
    pub timeout: u64,
    #[serde(with = "serde_bytes")]
    pub entry_signature: Vec<u8>,
    pub profile_version: u32,
}

/// Origin-side manifest state that must survive restart per spec §10.8.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HostManifestState {
    pub manifest_version: u32,
    pub is_gateway: bool,
    #[serde(default)]
    pub entries: Vec<DelegationEntry>,
    #[serde(default)]
    pub last_bump_ms_reserved: Option<u64>,
}

impl HostManifestState {
    /// load data from `<storage_path>/manifest_state.yaml`
    pub fn load_or_default(storage_path: &str) -> Self {
        let path = Path::new(storage_path).join("manifest_state.yaml");
        let Ok(bytes) = fs::read(&path) else {
            info!("no manifest_state.yaml. first startup, using defaults");
            return Self::default();
        };

        match serde_yaml_ng::from_slice::<Self>(&bytes) {
            Ok(state) => {
                if state.entries.iter().any(|e| e.entry_signature.len() != 64) {
                    error!("manifest_state.yaml has bad entry_signature length; using defaults");
                    return Self::default();
                }
                state
            }
            Err(e) => {
                error!("failed to deserialize manifest_state.yaml: {e}; using defaults");
                Self::default()
            }
        }
    }

    /// save to `<storage_path>/manifest_state.yaml`
    pub fn save_to_path(&self, storage_path: &str) {
        let yaml =
            serde_yaml_ng::to_string(self).expect("HostManifestState should always serialise");
        let path = Path::new(storage_path).join("manifest_state.yaml");
        if let Err(e) = fs::write(&path, yaml) {
            error!("failed to persist manifest_state to {path:?}: {e}");
        }
    }
}
