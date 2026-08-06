// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Index translation and mapping application (spec §3.6, §3.8, §8.8).

use libp2p::PeerId;
use tracing::{debug, info, warn};

use crate::router_v2::{
    codec::messages::{IndexDump, Mapping},
    index::Space,
    manifest::ManifestLog,
    seq::is_fresher_u32,
    table::{Node, TargetRef, User},
    Result, RouterV2State, RoutingV2Error,
};

impl RouterV2State {
    pub fn translate_incoming(
        &self,
        neighbour: PeerId,
        space: Space,
        incoming_idx: u16,
    ) -> Result<u16> {
        let id = {
            let mirrors = self.mirrors.read().unwrap();
            let mirrors_for_neighbour = mirrors
                .get(&neighbour)
                .ok_or(RoutingV2Error::UnknownMapping(incoming_idx))?;
            let mirror_dict = match space {
                Space::Node => &mirrors_for_neighbour.nodes,
                Space::User => &mirrors_for_neighbour.users,
            };
            mirror_dict
                .id_of(incoming_idx)
                .ok_or(RoutingV2Error::UnknownMapping(incoming_idx))?
        };

        let (dict, alloc) = match space {
            Space::Node => (&self.node_dict, &self.node_allocator),
            Space::User => (&self.user_dict, &self.users_allocator),
        };

        let mut self_dict = dict.write().unwrap();
        if let Some(i) = self_dict.idx_of(&id) {
            return Ok(i);
        }

        let mut allocator = alloc.write().unwrap();
        let mut tracker = self.reintroduction_tracker.write().unwrap();

        let Some(allocated_idx) = allocator.allocate() else {
            return Err(RoutingV2Error::AllocatorExhausted);
        };
        self_dict.bind(allocated_idx, id);
        tracker.mark_first_time(space, allocated_idx);

        Ok(allocated_idx)
    }

    pub fn apply_mapping(&self, neighbour: PeerId, space: Space, mapping: Mapping) -> Result<()> {
        let mirror_id = {
            let mirrors = self.mirrors.read().unwrap();
            let Some(neigbour_mirrors) = mirrors.get(&neighbour) else {
                debug!("neighbour vanished between mapping steps: {neighbour:?}");
                return Ok(());
            };
            let dict = match space {
                Space::Node => &neigbour_mirrors.nodes,
                Space::User => &neigbour_mirrors.users,
            };
            dict.id_of(mapping.abs_idx)
        };

        match mirror_id {
            Some(id) if id != mapping.target_id => {
                self.release_index(space, &id);
            }
            Some(_) => {}
            None => {}
        };

        // now, we can safely bind the mapping to the correspoding neighbor
        {
            let mut mirrors = self.mirrors.write().unwrap();
            let Some(neigbour_mirrors) = mirrors.get_mut(&neighbour) else {
                return Ok(());
            };
            let dict = match space {
                Space::Node => &mut neigbour_mirrors.nodes,
                Space::User => &mut neigbour_mirrors.users,
            };
            dict.bind(mapping.abs_idx, mapping.target_id);
        }

        match space {
            Space::Node => {
                let existing = self.nodes.read().unwrap().get(&mapping.target_id);
                match existing {
                    Some(node) => {
                        let advertised = {
                            let n = node.read().unwrap();
                            n.advertised_version
                        };

                        if is_fresher_u32(mapping.version, advertised) {
                            {
                                let mut n = node.write().unwrap();
                                n.advertised_version = mapping.version;
                            }
                            // per 10.8. a fresher advertisement is the pull trigger
                            self.maybe_request_manifest(
                                neighbour,
                                mapping.target_id,
                                mapping.version,
                            );
                        } else {
                            debug!("stale node advertisement from {neighbour:?}: target={:?} stored_advertised={advertised} incoming={}",
                                mapping.target_id, mapping.version);
                        }
                    }
                    None => {
                        let n = Node {
                            id: mapping.target_id,
                            public_key: None,
                            manifest_version: 0,
                            advertised_version: mapping.version,
                            is_gateway: false,
                            delegated_users: Vec::new(),
                            manifest_signature: None,
                            retained_chunks: None,
                            learn_sphere: None,
                            manifest_log: ManifestLog::default(),
                        };
                        self.nodes.write().unwrap().insert(mapping.target_id, n);
                        // new origin does not hold any state, so, we need to pull it since its version is fresher
                        self.maybe_request_manifest(neighbour, mapping.target_id, mapping.version);
                        return Ok(());
                    }
                }
            }
            Space::User => {
                let mut users = self.users.write().unwrap();
                match users.get(&mapping.target_id) {
                    Some(user) => {
                        let version = {
                            let u = user.read().unwrap();
                            u.profile_version
                        };
                        if is_fresher_u32(mapping.version, version) {
                            let mut u = user.write().unwrap();
                            u.profile_version = mapping.version;
                        } else if version == mapping.version {
                        } else {
                            // TODO
                            debug!(
                                "stale profile echo from {neighbour:?}: target={:?} stored_version={version} incoming={}",
                                mapping.target_id,
                                mapping.version
                            );
                        }
                    }
                    None => {
                        let u = User {
                            id: mapping.target_id,
                            profile_version: mapping.version,
                            routing_entry: None,
                            delegation_gateways: Vec::new(),
                            public_key: None,
                            is_hosted: false,
                        };
                        users.insert(mapping.target_id, u);
                    }
                };
            }
        }
        Ok(())
    }

    pub(crate) fn lookup_target(&self, space: Space, own_idx: u16) -> Option<TargetRef> {
        match space {
            Space::User => {
                let dict = self.user_dict.read().unwrap();
                let id = dict.id_of(own_idx)?;
                drop(dict);
                let users = self.users.read().unwrap();
                users.get(&id).map(TargetRef::User)
            }
            Space::Node => {
                let dict = self.node_dict.read().unwrap();
                let id = dict.id_of(own_idx)?;
                drop(dict);
                let nodes = self.nodes.read().unwrap();
                nodes.get(&id).map(TargetRef::Node)
            }
        }
    }

    pub fn handle_index_dump(&self, neighbour: PeerId, msg: IndexDump) -> Result<()> {
        // TEMP(smoke test)
        info!(
            "router_v2 INDEX_DUMP ← peer={neighbour} user_mappings={} node_mappings={}",
            msg.user_mappings.len(),
            msg.node_mappings.len(),
        );

        for mapping in msg.user_mappings {
            if let Err(e) = self.apply_mapping(neighbour, Space::User, mapping) {
                warn!("index_dump: apply_mapping user failed: {e}");
            }
        }

        for mapping in msg.node_mappings {
            if let Err(e) = self.apply_mapping(neighbour, Space::Node, mapping) {
                warn!("index_dump: apply_mapping node failed: {e}");
            }
        }
        Ok(())
    }
}
