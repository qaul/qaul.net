// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use ble_protocol::engine::{EngineAction, EngineEvent, Peer, Role, Snapshot};
use bytes::Bytes;
use std::{error::Error, time::{Duration, Instant}};
use tokio::time::{interval, MissedTickBehavior};

use crate::{
    ble::radio::{get_device_info, PeerKey, RadioEvent, Running},
    rpc::{proto_sys::{ble::Message::*, *}, utils::*, SysRpcReceiver},
};

use super::BleRpc;

/// How often the status line is logged. Android logs its snapshot every 3 s.
const STATUS_INTERVAL: Duration = Duration::from_secs(3);

/// The BLE module's one loop: handles libqaul's requests, the GATT server's
/// events and the tick for as long as the module runs.
pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    mut internal_sender: BleResultSender,
) -> Result<(), Box<dyn Error>> {
    let mut running: Option<Running> = None;
    let mut tick = interval(Duration::from_secs(1));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
    // A one line status summary, like Androids ConnectionPool snapshot.
    let mut status = interval(STATUS_INTERVAL);
    status.set_missed_tick_behavior(MissedTickBehavior::Skip);
    // Adverts heard since the last status line: 0 means the scanner hears nothing.
    let mut adverts_heard: usize = 0;
    loop {
        tokio::select! {
            evt = rpc_receiver.recv() => { 
                match evt {
                    None => {
                        log::info!("Qaul 'sys' message channel closed. Shutting down gracefully.");
                        break;
                    }
                    Some(msg) => {
                        if msg.message.is_none() {
                            log::warn!("BLE RPC message with no content, ignored");
                            continue;
                        }
                        match msg.message.unwrap() {
                            StartRequest(req) => {
                                if running.is_some(){
                                    log::warn!(
                                        "Received Start Request, but bluetooth service is already running!"
                                    );
                                    internal_sender.send_result_already_running();
                                    continue;
                                }
                                let device_id = req.device_id;
                                let qaul_id = Bytes::from(req.qaul_id);
                                running = match Running::start(&device_id, qaul_id).await {
                                    Ok(r) => {
                                        log::info!(
                                            "BLE service started successfully on device '{}'",
                                            device_id
                                        );
                                        internal_sender.send_start_successful();
                                        Some(r)
                                    },
                                    Err(err) => {
                                        log::error!("BLE device '{}' unavailable: {}", device_id, err);
                                        internal_sender.send_start_error(
                                            BleError::DeviceUnavailable,
                                            format!("BLE device '{}' unavailable: {}", device_id, err),
                                        );
                                        continue;
                                    }
                                };
                                
                            }
                            StopRequest(_) => {
                                if stop(&mut running, &mut internal_sender) {
                                    internal_sender.send_stop_successful();
                                }
                                else {
                                    internal_sender.send_result_not_running();
                                }
                            }
                            DirectSend(req) => {
                                let Some(r) = running.as_mut() else {
                                    internal_sender.send_direct_send_error(req.message_id, "BLE not running".into());
                                    continue;
                                };
                                let Ok(to) = req.receiver_id.as_slice().try_into() else {
                                    internal_sender.send_direct_send_error(req.message_id, "bad receiver id".into());
                                    continue;
                                };
                                // hand it to the engine
                                let event = EngineEvent::Send { to, message: req.data, message_id: req.message_id };
                                let engine_actions = r.engine.handle(event, Instant::now());
                                execute(r, engine_actions, &mut internal_sender);
                            }
                            InfoRequest(_) => {
                                let mut sender_handle_clone = internal_sender.clone();
                                tokio::task::spawn_local(async move {
                                    match get_device_info().await {
                                        Ok(info) => {
                                            sender_handle_clone.send_ble_sys_msg(InfoResponse(info))
                                        }
                                        Err(err) => {
                                            log::error!("Error getting device info: {:#?}", &err)
                                        }
                                    }
                                });
                            }
                            _ => (),
                        }
                    }
                }
            }
            evt = next_event(&mut running) => match evt {
                // Radio events
                Some(RadioEvent::Subscribed { peer, mtu, outbox }) => {
                    log::info!("{} ({:?}) subscribed, MTU {}", peer.address, peer.role, mtu);
                    if let Some(r) = running.as_mut() {
                        // Their connect endeds the advert, put it back so others can still find us.
                        if peer.role == Role::Peripheral {
                            if let Err(err) = r.readvertise().await {
                                log::warn!("re-advertising after {} connected failed: {}", peer.address, err);
                            }
                        }
                        r.outboxes.insert(peer, outbox);
                        let engine_actions = r.engine.handle(EngineEvent::PeerReady { peer, mtu }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }      
                Some(RadioEvent::FrameReceived { peer, bytes }) => {
                    // The first byte is the frame type: 01 SEND_ID, 06 ping, 07 neighbours, 08 chunk...
                    log::debug!("{} ({:?}) frame {:02x?}, {} bytes", peer.address, peer.role, bytes.first(), bytes.len());
                    if let Some(r) = running.as_mut() {
                        let engine_actions = r.engine.handle(EngineEvent::Frame { peer, bytes }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }
                Some(RadioEvent::Advert{address, id_prefix, rssi}) => {
                    // Debug:one per advert, every ~100 ms per phone.
                    log::debug!("{} advert {:02x?} rssi {}", address, id_prefix, rssi);
                    adverts_heard += 1;
                    if let Some(r) = running.as_mut() {
                        // The key wed connect under, so the engine can match it to a link.
                        let peer = PeerKey::central(address);
                        let engine_actions = r.engine.handle(EngineEvent::Discovered { peer, id_prefix, rssi }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }
                Some(RadioEvent::PeerIdRead { peer, id }) => {
                    log::info!("{} ({:?}) read id {:02x?}", peer.address, peer.role, id);
                    if let Some(r) = running.as_mut() {
                        let engine_actions = r.engine.handle(EngineEvent::PeerIdRead { peer, id }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }
                Some(RadioEvent::ConnectFailed { peer }) => {
                    log::info!("{} ({:?}) connect failed", peer.address, peer.role);
                    if let Some(r) = running.as_mut() {
                        let engine_actions = r.engine.handle(EngineEvent::ConnectFailed { peer }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }
                Some(RadioEvent::Gone { peer }) => {
                    log::info!("{} ({:?}) gone", peer.address, peer.role);
                    if let Some(r) = running.as_mut() {
                        r.disconnect(peer);
                        let engine_actions = r.engine.handle(EngineEvent::PeerGone { peer }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }
                None => {
                    // Can't happen while Running holds a sender; kept so the loop can't spin on None.
                    log::error!("radio event channel closed, stopping BLE");
                    stop(&mut running, &mut internal_sender);
                }
            },
            _ = tick.tick() => {
                if let Some(r) = running.as_mut() {
                    let engine_actions = r.engine.handle(EngineEvent::Tick, Instant::now());
                    execute(r, engine_actions, &mut internal_sender);
                }
            }
            _ = status.tick() => {
                if let Some(r) = running.as_ref() {
                    log::info!("{}", status_line(&r.engine.snapshot(), adverts_heard));
                }
                adverts_heard = 0;
            }
        }
    }
    Ok(())
}

/// One line summing up BLE
/// IDs are shown as their advertised prefix; "up" counts links libqaul has been told about.
fn status_line(snapshot: &Snapshot<PeerKey>, adverts_heard: usize) -> String {
    let up = snapshot.links.iter().filter(|link| link.id.is_some()).count();
    // Sorted, so the same links print in the same order every time.
    let mut links: Vec<String> = snapshot
        .links
        .iter()
        .map(|link| {
            let id = link.id.map_or("unresolved".to_string(), |id| id_prefix(&id));
            let ready = if link.ready { "" } else { " (not ready)" };
            format!("{}/{:?}/{}{}", link.peer.address, link.peer.role(), id, ready)
        })
        .collect();
    links.sort();
    format!(
        "BLE me={} adverts={} connecting={} links={} up={} | {}",
        id_prefix(&snapshot.our_id),
        adverts_heard,
        snapshot.connecting,
        snapshot.links.len(),
        up,
        if links.is_empty() { "none".to_string() } else { links.join(" ") }
    )
}

/// The part of a qaul ID that's advertised, as hex.
fn id_prefix(id: &[u8]) -> String {
    id.iter().take(ble_protocol::QAUL_ID_ADVERT_BYTES).map(|b| format!("{:02x}", b)).collect()
}

/// The next event from the GATT server, or never while BLE is stopped, so
/// the loop's `select!` always has a future for this branch.
async fn next_event(running: &mut Option<Running>) -> Option<RadioEvent> {
    match running {
        Some(r) => r.events.recv().await,
        None => std::future::pending().await,
    }
}

/// Take BLE off air: report every unfinished message and identified peer
/// to libqaul, then drop Running. Returns false if it wasn't running.
fn stop(running: &mut Option<Running>, sender: &mut BleResultSender) -> bool {
    let Some(mut r) = running.take() else {
        return false;
    };
    // Before the drop, or the engine's promises go with it.
    let actions = r.engine.shutdown();
    execute(&mut r, actions, sender);
    true
}

/// Carry out what the engine decided (EngineActions), in order. Cant await, so one slow
/// peer doesnt hold up the loop.
fn execute(r: &mut Running, actions: Vec<EngineAction<PeerKey>>, sender: &mut BleResultSender) {
    for action in actions {
        match action {
            EngineAction::Transmit { peer, bytes } => {
                // A dropped chunk is recovered by MissingChunk, liveness catches a stuck peer.
                match r.outboxes.get(&peer) {
                    Some(outbox) => {
                        if let Err(err) = outbox.try_send(bytes) {
                            log::warn!("dropped frame to {:?}: {}", peer, err);
                        }
                    }
                    None => log::warn!("dropped frame to {:?}: no outbox", peer),
                }
            }
            EngineAction::Disconnect { peer } => r.disconnect(peer),
            EngineAction::Connect { peer } => {
                log::info!("{} connecting as central", peer.address);
                r.connect(peer);
            }
            // No RSSI for a phone connected to our GATT server yet?
            EngineAction::PeerUp { id } => sender.send_device_found(id.to_vec(), 0),
            EngineAction::PeerDown { id } => sender.send_peer_unavailable(id.to_vec()),
            EngineAction::Deliver { from, message } => {
                sender.send_direct_received(from.to_vec(), message)
            }
            EngineAction::SendResult { message_id, success: true } => {
                sender.send_direct_send_success(message_id)
            }
            EngineAction::SendResult { message_id, success: false } => {
                sender.send_direct_send_error(message_id, "BLE link failed".into())
            }
        }
    }
    // Every engine call ends here, so this sees each connect start and finish.
    r.update_scan_pause();
}