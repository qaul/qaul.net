// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Serve-side decision making and inbound chunk assembly (spec §8.5, §8.8).

use std::collections::HashMap;

use tracing::debug;

use crate::router_v2::{
    codec::messages::{ManifestEntry, ManifestRequestItem, NodeManifest},
    seq::is_fresher_u32,
    Sphere,
};

pub struct OriginServeState {
    pub committed: u32,
    pub log_base: u32,
    /// Sphere the manifest was learned over.
    pub learn_sphere: Option<Sphere>,
}

/// per 8.8 how a MANIFEST_REQUEST should be answered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServeDecision {
    /// Full `NODE_MANIFEST`
    /// which means the requester needs to bootstrap, that is, its base is far behind the current log
    Full,
    Delta {
        from_version: u32,
    },
    /// versions are equal
    Nothing,
    /// per 2.3: if we learnt about it over internet
    Sealed,
}

/// per 8.8: steps 2 to 5: we decide how to answer one request item
pub fn decide_serve(
    item: &ManifestRequestItem,
    origin: &OriginServeState,
    requester_sphere: Sphere,
) -> ServeDecision {
    if origin.learn_sphere == Some(Sphere::Internet) && requester_sphere == Sphere::Local {
        return ServeDecision::Sealed;
    }

    if item.have_none() {
        return ServeDecision::Full;
    }

    if !is_fresher_u32(item.have_version, origin.log_base) {
        return ServeDecision::Full;
    }

    if item.have_version == origin.committed {
        return ServeDecision::Nothing;
    }

    if is_fresher_u32(origin.committed, item.have_version) {
        return ServeDecision::Delta {
            from_version: item.have_version,
        };
    }

    // anything else we cannot decipher, sending back full manifest
    // is the best decision, i think
    ServeDecision::Full
}

/// a manifest that was assembled successfully from bytes over the wire
pub struct CompletedManifest {
    pub manifest_version: u32,
    pub flags: u8,
    pub entries: Vec<ManifestEntry>,
    pub chunks: Vec<NodeManifest>,
}

pub struct ChunkAssembler {
    partials: HashMap<[u8; 8], PartialManifest>,
}

struct PartialManifest {
    manifest_version: u32,
    chunk_count: u8,
    flags: u8,
    /// chunk_index → its entries.
    received: HashMap<u8, NodeManifest>,
}

impl ChunkAssembler {
    pub fn new() -> Self {
        Self {
            partials: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        origin_node_id: [u8; 8],
        chunk: NodeManifest,
    ) -> Option<CompletedManifest> {
        if chunk.chunk_index >= chunk.chunk_count {
            debug!("chunk index exceeds chunk_count");
            return None;
        }

        enum Action {
            Insert,
            Replace,
            Reset,
        }

        let action = match self.partials.get(&origin_node_id) {
            Some(partial) if partial.manifest_version == chunk.manifest_version => {
                if chunk.chunk_count != partial.chunk_count || chunk.flags != partial.flags {
                    Action::Reset
                } else {
                    Action::Insert
                }
            }
            _ => Action::Replace,
        };

        match action {
            Action::Insert => {
                self.partials
                    .get_mut(&origin_node_id)
                    .unwrap()
                    .received
                    .insert(chunk.chunk_index, chunk);
            }
            Action::Replace => {
                let manifest_version = chunk.manifest_version;
                let chunk_count = chunk.chunk_count;
                let flags = chunk.flags;
                let chunk_index = chunk.chunk_index;

                let mut received = HashMap::new();
                received.insert(chunk_index, chunk);

                self.partials.insert(
                    origin_node_id,
                    PartialManifest {
                        manifest_version,
                        chunk_count,
                        flags,
                        received,
                    },
                );
            }
            Action::Reset => {
                debug!("chunk set inconsistent; dropping partial for {origin_node_id:?}");
                self.partials.remove(&origin_node_id);
                return None;
            }
        }

        let partial = self.partials.get_mut(&origin_node_id)?;
        if (partial.received.len() as u8) < partial.chunk_count {
            return None;
        }

        let manifest_version = partial.manifest_version;
        let flags = partial.flags;
        let chunk_count = partial.chunk_count;

        let mut entries: Vec<ManifestEntry> = Vec::new();
        let mut chunks: Vec<NodeManifest> = Vec::new();

        for i in 0..chunk_count {
            if let Some(chunk) = partial.received.remove(&i) {
                entries.extend(chunk.entries.iter().cloned());
                chunks.push(chunk);
            }
        }

        self.partials.remove(&origin_node_id);

        Some(CompletedManifest {
            manifest_version,
            flags,
            entries,
            chunks,
        })
    }
}
