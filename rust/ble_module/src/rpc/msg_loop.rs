// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use ble_protocol::engine::{EngineAction, EngineEvent};
use bytes::Bytes;
use std::{error::Error, time::{Duration, Instant}};
use tokio::time::{interval, MissedTickBehavior};

use crate::{
    ble::radio::{get_device_info, PeerKey, RadioEvent, Running},
    rpc::{proto_sys::{ble::Message::*, *}, utils::*, SysRpcReceiver},
};

use super::BleRpc;

/// The BLE module's one loop: handles libqaul's requests, the GATT server's
/// events and the tick for as long as the module runs.
pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    mut internal_sender: BleResultSender,
) -> Result<(), Box<dyn Error>> {
    let mut running: Option<Running> = None;
    let mut tick = interval(Duration::from_secs(1));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
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
                Some(RadioEvent::Subscribed { address, mtu, outbox }) => {
                    log::info!("{} subscribed, MTU {}", address, mtu);
                    if let Some(r) = running.as_mut() {
                        let peer = PeerKey::peripheral(address);
                        r.outboxes.insert(peer, outbox);
                        let engine_actions = r.engine.handle(EngineEvent::PeerReady { peer, mtu }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }      
                Some(RadioEvent::FrameReceived { address, bytes }) => {
                    log::info!("{} sent {} bytes: {:02x?}", address, bytes.len(), bytes);
                    if let Some(r) = running.as_mut() {
                        let peer = PeerKey::peripheral(address);
                        let engine_actions = r.engine.handle(EngineEvent::Frame { peer, bytes }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }
                Some(RadioEvent::Advert{address, id_prefix, rssi}) => {
                    log::info!("{} advert seen: {} rssi {:02x?}", address, rssi, id_prefix);
                }
                Some(RadioEvent::Gone { address }) => {
                    log::info!("{} gone", address);
                    if let Some(r) = running.as_mut() {
                        let peer = PeerKey::peripheral(address);
                        r.outboxes.remove(&peer);
                        let engine_actions = r.engine.handle(EngineEvent::PeerGone { peer }, Instant::now());
                        execute(r, engine_actions, &mut internal_sender);
                    }
                }
                None => {
                    // Every radio task is gone, so recv() would return None forever.
                    log::error!("GATT server event channel closed, stopping BLE");
                    stop(&mut running, &mut internal_sender);
                }
            },
            _ = tick.tick() => {
                if let Some(r) = running.as_mut() {
                    let engine_actions = r.engine.handle(EngineEvent::Tick, Instant::now());
                    execute(r, engine_actions, &mut internal_sender);
                }
            }
        }
    }
    Ok(())
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
            EngineAction::Connect { peer } => log::warn!("connect to {:?}: central not implemented", peer),
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
}