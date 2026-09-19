// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Answers MANIFEST_REQUEST (spec §8.7, §8.8 steps 1-5, §10.8).

use libp2p::PeerId;
use tracing::{debug, error, info};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{
            messages::{ManifestEntry, ManifestRequest, NodeManifest},
            RoutingMessage,
        },
        manifest::{
            decide_serve, reconstruct_single_chunk_full, DeltaHeader, OriginServeState,
            ServeDecision,
        },
        Result, RouterV2State, Sphere,
    },
};

impl RouterV2State {
    pub fn handle_manifest_request(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        msg: ManifestRequest,
        now: u64,
    ) -> Result<()> {
        let host_node_id = self.host_mk.to_id();
        let requester_sphere = Sphere::of(transport);

        for item in &msg.items {
            let origin_node_id = item.origin_node_id;
            let is_own = origin_node_id == host_node_id;

            // Step 1: no state for this origin means we never advertised it
            let Some(origin) = self.origin_serve_state(&origin_node_id, is_own) else {
                debug!("manifest_request: no state for origin {origin_node_id:?}, ignoring item");
                continue;
            };

            let decision = decide_serve(item, &origin, requester_sphere);
            info!(
                "router_v2 MANIFEST_REQUEST ← peer={neighbour} origin={origin_node_id:?} have={} have_none={} committed={} log_base={} → {decision:?}",
                item.have_version,
                item.have_none(),
                origin.committed,
                origin.log_base,
            );

            match decision {
                ServeDecision::Sealed | ServeDecision::Nothing => {}
                ServeDecision::Full | ServeDecision::Delta { .. }
                    if !self.allow_manifest_serve(neighbour, now) =>
                {
                    debug!(
                        "manifest_request: serve rate limit reached for peer={neighbour}, ignoring item for origin {origin_node_id:?}"
                    );
                }
                ServeDecision::Full => {
                    self.serve_full_manifest(neighbour, transport, origin_node_id, is_own)
                }
                ServeDecision::Delta { from_version } => {
                    self.serve_delta(neighbour, transport, origin_node_id, is_own, from_version)
                }
            }
        }

        Ok(())
    }

    /// looks for the serving view of an origin, or `None` when we hold nothing
    fn origin_serve_state(
        &self,
        origin_node_id: &[u8; 8],
        is_own: bool,
    ) -> Option<OriginServeState> {
        if is_own {
            return Some(OriginServeState {
                committed: self.manifest.read().unwrap().manifest_version,
                log_base: self.own_manifest_log.read().unwrap().log_base,
                learn_sphere: None,
            });
        }

        let node_arc = self.nodes.read().unwrap().get(origin_node_id)?;
        let node = node_arc.read().unwrap();
        // a stub created from a mapping has version 0 and no entries
        if node.manifest_version == 0 && node.delegated_users.is_empty() {
            return None;
        }
        Some(OriginServeState {
            committed: node.manifest_version,
            log_base: node.manifest_log.log_base,
            learn_sphere: node.learn_sphere,
        })
    }

    /// serve a full `NODE_MANIFEST`
    fn serve_full_manifest(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        origin_node_id: [u8; 8],
        is_own: bool,
    ) {
        let chunks: Vec<NodeManifest> = if is_own {
            let manifest = self.manifest.read().unwrap();

            if let Some(retained) = &manifest.retained_chunks {
                retained.clone()
            } else if let Some(signature) = manifest.manifest_signature {
                // TODO: fix this function, it has too many args, at least for now
                vec![reconstruct_single_chunk_full(
                    origin_node_id,
                    manifest.manifest_version,
                    manifest.is_gateway,
                    manifest.entries().to_vec(),
                    signature,
                )]
            } else {
                debug!("serve_full: own manifest not signed yet, cannot serve");
                return;
            }
        } else {
            let Some(node_arc) = self.nodes.read().unwrap().get(&origin_node_id) else {
                return;
            };
            let node = node_arc.read().unwrap();

            if let Some(retained) = &node.retained_chunks {
                retained.clone()
            } else if let Some(signature) = node.manifest_signature {
                vec![reconstruct_single_chunk_full(
                    origin_node_id,
                    node.manifest_version,
                    node.is_gateway,
                    node.delegated_users
                        .iter()
                        .map(|d| ManifestEntry {
                            user_id: d.user_id,
                            timeout: d.delegation_timeout,
                            entry_signature: d.entry_signature,
                            profile_version: d.profile_version,
                        })
                        .collect(),
                    signature,
                )]
            } else {
                debug!("serve_full: no signed bytes retained for {origin_node_id:?}, cannot serve");
                return;
            }
        };

        for chunk in chunks {
            let mut body = Vec::new();
            if let Err(e) = chunk.encode(&mut body) {
                error!("serve_full: encode failed for {origin_node_id:?}: {e}");
                return;
            }
            self.send_framed(neighbour, transport, RoutingMessage::NodeManifest, body);
        }
    }

    /// serve a MANIFEST_DELTA. respond with full manifest is body is above 60kb
    fn serve_delta(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        origin_node_id: [u8; 8],
        is_own: bool,
        from_version: u32,
    ) {
        let assembled = if is_own {
            let manifest = self.manifest.read().unwrap();
            // Cached at bump time, never computed here — see serve_full.
            let Some(signature) = manifest.manifest_signature else {
                debug!("serve_delta: own manifest not signed yet, serving full");
                drop(manifest);
                self.serve_full_manifest(neighbour, transport, origin_node_id, true);
                return;
            };
            let header = DeltaHeader {
                origin_node_id,
                from_version,
                to_version: manifest.manifest_version,
                is_gateway: manifest.is_gateway,
                manifest_signature: signature,
            };
            let records = self
                .own_manifest_log
                .read()
                .unwrap()
                .records_after(from_version);
            header.assemble(records)
        } else {
            let Some(node_arc) = self.nodes.read().unwrap().get(&origin_node_id) else {
                return;
            };
            let node = node_arc.read().unwrap();
            let Some(signature) = node.manifest_signature else {
                debug!(
                    "serve_delta: no whole-state signature for {origin_node_id:?}, serving full"
                );
                drop(node);
                self.serve_full_manifest(neighbour, transport, origin_node_id, false);
                return;
            };
            let header = DeltaHeader {
                origin_node_id,
                from_version,
                to_version: node.manifest_version,
                is_gateway: node.is_gateway,
                manifest_signature: signature,
            };
            let records = node.manifest_log.records_after(from_version);
            header.assemble(records)
        };

        // §8.6: a delta is never chunked, so an oversize range becomes a
        // full manifest
        let assembled = match assembled {
            Ok(msg) => msg,
            Err(e) => {
                debug!("serve_delta: {e} for {origin_node_id:?}");
                self.serve_full_manifest(neighbour, transport, origin_node_id, is_own);
                return;
            }
        };

        let mut body = Vec::new();
        if let Err(e) = assembled.encode(&mut body) {
            error!("serve_delta: encode failed for {origin_node_id:?}: {e}");
            return;
        }

        self.send_framed(neighbour, transport, RoutingMessage::ManifestDelta, body);
    }
}
