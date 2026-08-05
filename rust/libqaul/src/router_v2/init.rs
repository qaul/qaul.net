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
        BumpTrigger, OutboundMsg, RouterV2State,
    },
    storage::{configuration::RoutingV2Options, manifest_state::HostManifestState},
};

pub struct RouterV2Handle {
    pub state: Arc<RouterV2State>,
    pub rx: UnboundedReceiver<OutboundMsg>,
}

/// Builds a live [`RouterV2State`] from the host's keypair.
pub fn init_router_v2(
    host_keypair: Keypair,
    storage_path: &str,
    options: RoutingV2Options,
) -> RouterV2Handle {
    let host_multikey = Multikey::from(host_keypair.public());

    let (state, rx) = RouterV2State::new(host_keypair, host_multikey, options);
    let state = Arc::new(state);

    // Restore persisted host manifest (spec §10.8).
    let persisted = HostManifestState::load_or_default(storage_path);
    state.restore_host_manifest(&persisted);

    // origin tick: spec §7.1
    spawn_origin_tick(Arc::clone(&state));

    // relay tick: spec §7.1, and the §10.8 manifest bump/persist path.
    spawn_relay_tick(Arc::clone(&state), storage_path.to_string());

    RouterV2Handle { state, rx }
}

/// Milliseconds since the UNIX epoch, matching the unit used throughout
/// `router_v2` for timeouts, expiry and rate-limit windows.
fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn spawn_origin_tick(state: Arc<RouterV2State>) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(10));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        ticker.tick().await;
        loop {
            ticker.tick().await;
            tick_origin(&state, now_ms());
        }
    });
}

fn spawn_relay_tick(state: Arc<RouterV2State>, storage_path: String) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(1));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            let now = now_ms();
            tick_relay(&state, now);

            // §10.8: batched manifest pulls, one message per neighbour.
            state.drop_manifest_req_timeout(now);
            for (peer, request) in state.drain_manifest_reqs(now) {
                state.send_manifest_request(peer, request);
            }

            // §10.8
            if state
                .try_bump_manifest_version(now, BumpTrigger::Accumulated)
                .is_some()
            {
                state.host_manifest_snapshot().save_to_path(&storage_path);
            }
        }
    });
}
