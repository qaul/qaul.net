// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Neighbour registration and per-transport bookkeeping (spec §4.2, §4.3).

use std::collections::hash_map::Entry;

use libp2p::PeerId;

use crate::{
    connections::ConnectionModule,
    router_v2::{
        identity::Multikey, index::Space, manifest::ManifestLog, metric, table::Node,
        NeighbourInfo, RouterV2State,
    },
};

impl RouterV2State {
    /// Resolves a routable node id back to the neighbour `PeerId` we reach it
    /// through.
    pub fn peer_of_node(&self, node_id: &[u8; 8]) -> Option<PeerId> {
        self.mirrors
            .read()
            .unwrap()
            .iter()
            .find(|(_, info)| info.node_id == *node_id)
            .map(|(peer, _)| *peer)
    }

    /// checks if we can reach the neighbour over internet
    pub fn has_active_internet_transport(&self) -> bool {
        self.mirrors
            .read()
            .unwrap()
            .values()
            .any(|info| info.transports.contains(&ConnectionModule::Internet))
    }

    pub fn neighbour_transport(&self, node_id: &[u8; 8]) -> Option<ConnectionModule> {
        let mirrors = self.mirrors.read().unwrap();
        mirrors
            .values()
            .find(|info| info.node_id == *node_id)?
            .transports
            .iter()
            .copied()
            .min_by_key(|t| metric::hop_cost(*t, None))
    }

    /// Registers a neighbour as a routable node: ensures a [`Node`]
    pub fn register_neighbour_node(
        &self,
        node_id: [u8; 8],
        public_key: Option<Multikey>,
        now_ms: u64,
    ) {
        {
            let mut nodes = self.nodes.write().unwrap();
            match nodes.get(&node_id) {
                Some(existing) => {
                    if public_key.is_some() {
                        existing.write().unwrap().public_key = public_key.clone();
                    }
                }
                None => nodes.insert(
                    node_id,
                    Node {
                        id: node_id,
                        public_key: public_key.clone(),
                        manifest_version: 0,
                        advertised_version: 0,
                        is_gateway: false,
                        delegated_users: Vec::new(),
                        manifest_signature: None,
                        retained_chunks: None,
                        learn_sphere: None,
                        manifest_log: ManifestLog::default(),
                    },
                ),
            }
        }

        let mut dict = self.node_dict.write().unwrap();
        if dict.idx_of(&node_id).is_some() {
            return;
        }

        let mut allocator = self.node_allocator.write().unwrap();
        let Some(idx) = allocator.allocate(now_ms) else {
            tracing::error!("node allocator exhausted registering neighbour {node_id:?}");
            return;
        };
        dict.bind(idx, node_id);

        self.reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::Node, idx);

        tracing::info!("router_v2: neighbour node {node_id:?} allocated node index {idx}");
    }

    /// saves `transport` as a way to reach `peer`.
    /// `true` when this transport was not already registered for peer
    pub fn add_neighbour_transport(
        &self,
        peer: PeerId,
        node_id: [u8; 8],
        transport: ConnectionModule,
    ) -> bool {
        let mut mirrors = self.mirrors.write().unwrap();
        match mirrors.entry(peer) {
            Entry::Occupied(mut existing) => existing.get_mut().transports.insert(transport),
            Entry::Vacant(slot) => {
                slot.insert(NeighbourInfo::new(node_id, transport));
                true
            }
        }
    }

    pub fn remove_neighbour_transport(&self, peer: PeerId, transport: ConnectionModule) {
        let mut mirrors = self.mirrors.write().unwrap();
        let now_empty = if let Some(info) = mirrors.get_mut(&peer) {
            info.transports.remove(&transport);
            info.transports.is_empty()
        } else {
            false
        };
        if now_empty {
            mirrors.remove(&peer);
            drop(mirrors);
            self.manifest_request_window.write().unwrap().remove(&peer);
            self.manifest_serve_window.write().unwrap().remove(&peer);
        }
    }
}
