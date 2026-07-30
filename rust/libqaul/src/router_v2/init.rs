// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Init entrypoint. it builds a live RouterV2State from a
//! host identity + storage path, starts the origin/relay ticks, and
//! returns the state plus a receiver for outbound bytes.

use std::sync::Arc;

use libp2p::identity::Keypair;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{
    router_v2::{
        identity::Multikey,
        propagation::{tick_origin, tick_relay},
        OutboundMsg, RouterV2State,
    },
    storage::manifest_state::HostManifestState,
};

pub struct RouterV2Handle {
    pub state: Arc<RouterV2State>,
    pub rx: UnboundedReceiver<OutboundMsg>,
}

pub fn init_router_v2(
    host_keypair: Keypair,
    host_multikey: Multikey,
    storage_path: &str,
) -> RouterV2Handle {
    let host_node_id = host_multikey.to_id();

    let (state, rx) = RouterV2State::new(host_node_id, host_keypair, host_multikey);
    let state = Arc::new(state);

    // Restore persisted host manifest (spec §10.8).
    let persisted = HostManifestState::load_or_default(storage_path);
    state.restore_host_manifest(&persisted);

    // origin tick: spec §7.1
    spawn_origin_tick(Arc::clone(&state));

    // relay tick: spec §7.1.
    spawn_relay_tick(Arc::clone(&state));

    RouterV2Handle { state, rx }
}

fn spawn_origin_tick(state: Arc<RouterV2State>) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(10));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        ticker.tick().await;
        loop {
            ticker.tick().await;
            tick_origin(&state);
        }
    });
}

fn spawn_relay_tick(state: Arc<RouterV2State>) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(1));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            tick_relay(&state, now_ms);
        }
    });
}
