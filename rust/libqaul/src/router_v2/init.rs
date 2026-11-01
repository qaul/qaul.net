// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Init entrypoint. it builds a live RouterV2State from a
//! host identity + storage path, starts the router tick, and
//! returns the state plus a receiver for outbound bytes.

use std::sync::Arc;

use libp2p::identity::Keypair;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{
    router_v2::{
        identity::Multikey,
        propagation::{origin_due, tick},
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

    // origin and relay updates: spec §7.1, and the §10.8 manifest bump/persist path.
    spawn_router_tick(Arc::clone(&state), storage_path.to_string());

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

/// One task for both updates, so their order within a pass is fixed.
fn spawn_router_tick(state: Arc<RouterV2State>, storage_path: String) {
    tokio::spawn(async move {
        let relay_secs = state.options.relay_interval.max(1);
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(relay_secs));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        let mut pass: u64 = 0;
        loop {
            ticker.tick().await;
            let now = now_ms();
            tick(&state, now, origin_due(pass, &state.options));
            pass += 1;

            // §10.8: batched manifest pulls, one message per neighbour.
            state.drop_manifest_req_timeout(now);

            state.clear_management_msgs(now);
            // §11.2: re-ask for profiles whose first fetch never landed.
            state.sweep_delegation_trust(now);
            // §11.5: same, for a routable user whose key we still lack
            state.sweep_user_profiles(now);
            state.clear_pending_subscribes(now);
            state.clear_delegation_state(now);
            for (peer, request) in state.drain_manifest_reqs(now) {
                state.send_manifest_request(peer, request);
            }

            // §10.7: drop users we can no longer deliver to.
            if state.sweep_unreachable_delegations(now) {
                state.host_manifest_snapshot().save_to_path(&storage_path);
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
