// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! ROUTING_UPDATE entry processing (spec §7.2, §8.3, §8.8 step 3).
//!
//! Applies the per-hop metric and hop-count rules, runs the relay-inclusion
//! decision, and commits accepted entries into the routing table.

use std::sync::{Arc, RwLock};

use tracing::{info, warn};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::messages::{NodeEntry, RoutingUpdate, UserEntry},
        index::Space,
        metric::hop_cost,
        receive::ReceiveCtx,
        seq::{is_fresher_u32, Acceptance, SeqNum},
        table::{RoutingEntry, TargetRef},
        Result, RouterV2State, RoutingV2Error,
    },
};
use libp2p::PeerId;

struct EntryArg {
    pub abs_idx: u16,
    pub seq: u16,
    pub metric: u16,
    pub hop_count: u8,
    pub local_only: bool,
}

impl From<&UserEntry> for EntryArg {
    fn from(e: &UserEntry) -> Self {
        Self {
            abs_idx: e.abs_idx,
            seq: e.seq,
            metric: e.metric,
            hop_count: e.hop_count,
            local_only: e.local_only,
        }
    }
}

impl From<&NodeEntry> for EntryArg {
    fn from(e: &NodeEntry) -> Self {
        Self {
            abs_idx: e.abs_idx,
            seq: e.seq,
            metric: e.metric,
            hop_count: e.hop_count,
            local_only: e.local_only,
        }
    }
}

struct AcceptedEntry {
    own_idx: u16,
    target: TargetRef,
    seq: u16,
    metric: u16,
    next_hop_idx: u16,
    hop_count: u8,
    local_only: bool,
}

enum EvaluateOutcome {
    Accept(AcceptedEntry),
    /// we successfly translated the target, but relay-inclusion rejected it.
    RejectedButTargetKnown {
        target_ref: TargetRef,
    },
    /// dropped before target resolution (unknown mapping, TTL, missing target).
    Dropped,
}

impl RouterV2State {
    fn evaluate_entry(
        &self,
        ctx: &ReceiveCtx,
        space: Space,
        entry: EntryArg,
    ) -> Result<EvaluateOutcome> {
        if entry.hop_count >= 63 {
            info!(
                "receive-loop drop: hop_count {} >= 63 (space={space:?}, peer={})",
                entry.hop_count, ctx.neighbour
            );
            return Ok(EvaluateOutcome::Dropped);
        }

        let metric = entry
            .metric
            .saturating_add(hop_cost(ctx.transport, ctx.rssi_dbm));

        let own_idx = match self.translate_incoming(ctx.neighbour, space, entry.abs_idx) {
            Ok(idx) => idx,
            Err(RoutingV2Error::UnknownMapping(idx)) => {
                info!(
                    "receive-loop drop: no mapping for incoming idx={idx} (space={space:?}, peer={})",
                    ctx.neighbour
                );
                return Ok(EvaluateOutcome::Dropped);
            }
            Err(e) => return Err(e),
        };

        let Some(target) = self.lookup_target(space, own_idx) else {
            info!("receive-loop drop: target lookup failed (space={space:?}, own_idx={own_idx})");
            return Ok(EvaluateOutcome::Dropped);
        };

        // §7.2 relay-inclusion + §7.4 local_only monotonicity.
        let (accept, local_only) = {
            let rt = self.routing_table.read().unwrap();
            match rt.get(space, own_idx) {
                None => (true, entry.local_only),
                Some(existing) => {
                    let stored = existing.read().unwrap();
                    let accept = match stored.seq_num.acceptance(SeqNum::from(entry.seq)) {
                        Acceptance::Fresher | Acceptance::Reboot => true,
                        Acceptance::NoChange => metric < stored.metric,
                    };
                    (accept, stored.local_only && entry.local_only)
                }
            }
        };
        if !accept {
            info!(
                    "receive-loop drop: not better than stored (own_idx={own_idx}, seq={}, metric={metric})",
                    entry.seq
                );
            return Ok(EvaluateOutcome::RejectedButTargetKnown { target_ref: target });
        }

        let neighbour_node_id = {
            let mirrors = self.mirrors.read().unwrap();
            let Some(neighbour_info) = mirrors.get(&ctx.neighbour) else {
                info!("neighbour vanished mid-receive: {:?}", ctx.neighbour);
                return Ok(EvaluateOutcome::RejectedButTargetKnown { target_ref: target });
            };
            neighbour_info.node_id
        };
        let next_hop_idx = {
            let dict = self.node_dict.read().unwrap();
            match dict.idx_of(&neighbour_node_id) {
                Some(idx) => idx,
                None => {
                    info!("neighbour node_id has no node_dict entry: {neighbour_node_id:?}");
                    return Ok(EvaluateOutcome::RejectedButTargetKnown { target_ref: target });
                }
            }
        };

        Ok(EvaluateOutcome::Accept(AcceptedEntry {
            own_idx,
            target,
            seq: entry.seq,
            metric,
            next_hop_idx,
            hop_count: entry.hop_count,
            local_only,
        }))
    }

    pub(crate) fn apply_user_entry(&self, ctx: &ReceiveCtx, entry: UserEntry) -> Result<()> {
        match self.evaluate_entry(&ctx, Space::User, (&entry).into())? {
            EvaluateOutcome::Accept(a) => {
                self.commit_routing_entry(ctx, Space::User, a);
            }
            _ => {}
        };
        Ok(())
    }

    pub(crate) fn apply_node_entry(&self, ctx: &ReceiveCtx, entry: NodeEntry) -> Result<()> {
        let outcome = self.evaluate_entry(ctx, Space::Node, (&entry).into())?;
        let target = match &outcome {
            EvaluateOutcome::Accept(a) => Some(&a.target),
            EvaluateOutcome::RejectedButTargetKnown { target_ref } => Some(target_ref),
            EvaluateOutcome::Dropped => None,
        };

        if let Some(TargetRef::Node(n)) = target {
            let target_id = {
                let mut node = n.write().unwrap();
                if is_fresher_u32(entry.manifest_version, node.advertised_version) {
                    node.advertised_version = entry.manifest_version;
                }
                node.id
            };
            // per 8.8: fire another pull trigger since the advertisement is recorded
            self.maybe_request_manifest(ctx.neighbour, target_id, entry.manifest_version);
        }

        if let EvaluateOutcome::Accept(accepted) = outcome {
            self.commit_routing_entry(ctx, Space::Node, accepted);
        }

        Ok(())
    }

    fn commit_routing_entry(&self, ctx: &ReceiveCtx, space: Space, accepted: AcceptedEntry) {
        let new_entry = Arc::new(RwLock::new(RoutingEntry {
            target: accepted.target,
            target_index: accepted.own_idx,
            seq_num: SeqNum::from(accepted.seq),
            metric: accepted.metric,
            next_hop: accepted.next_hop_idx,
            transport: ctx.transport,
            last_update: ctx.now,
            hop_count: accepted.hop_count.saturating_add(1),
            local_only: accepted.local_only,
        }));

        if let TargetRef::User(user) = &new_entry.read().unwrap().target {
            user.write().unwrap().routing_entry = Some(Arc::downgrade(&new_entry));
        }

        self.routing_table
            .write()
            .unwrap()
            .set(space, accepted.own_idx, new_entry);

        self.relay_queue
            .write()
            .unwrap()
            .insert((space, accepted.own_idx));

        // TEMP(smoke test): positive confirmation that an entry landed. Without
        // this, acceptance is only inferable from the *absence* of a drop log.
        info!(
            "router_v2 ACCEPT ✓ space={space:?} own_idx={} metric={} hop_count={} next_hop={} local_only={} peer={}",
            accepted.own_idx,
            accepted.metric,
            accepted.hop_count.saturating_add(1),
            accepted.next_hop_idx,
            accepted.local_only,
            ctx.neighbour,
        );
    }

    pub fn handle_routing_update(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        rssi_dbm: Option<i8>,
        msg: RoutingUpdate,
        now: u64,
    ) -> Result<()> {
        for mapping in msg.user_mappings {
            match self.apply_mapping(neighbour, Space::User, mapping) {
                Ok(_) => {}
                Err(e) => warn!("apply_mapping user failed: {e}"),
            };
        }

        for mapping in msg.node_mappings {
            match self.apply_mapping(neighbour, Space::Node, mapping) {
                Ok(_) => {}
                Err(e) => warn!("apply_mapping node failed: {e}"),
            };
        }

        let ctx = ReceiveCtx {
            neighbour,
            transport,
            rssi_dbm,
            now,
        };
        for entry in msg.user_entries {
            if let Err(e) = self.apply_user_entry(&ctx, entry) {
                warn!("apply_user_entry failed: {e}");
            }
        }

        for entry in msg.node_entries {
            if let Err(e) = self.apply_node_entry(&ctx, entry) {
                warn!("apply_node_entry failed: {e}");
            }
        }

        Ok(())
    }
}
