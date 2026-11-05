//! Finds nearby qaul nodes by their adverts and reports each one to the loop.

use std::{collections::HashSet, pin::Pin};

use bluer::{Adapter, AdapterEvent, Address};
use futures::{Stream, StreamExt};
use tokio::sync::{mpsc, watch};

use crate::ble::{
    ble_uuids::{main_service_uuid, QAUL_ID_ADVERT_BYTES, QAUL_MANUFACTURER_ID},
    radio::RadioEvent,
};

/// Scan for qaul adverts and report each one as RadioEvent::Advert.
/// Scanning stops when the loop drops its events receiver.
///
/// While `paused` is true the scanner is switched off. 
pub async fn scan(
    adapter: &Adapter,
    events: mpsc::Sender<RadioEvent>,
    mut paused: watch::Receiver<bool>,
) -> bluer::Result<()> {
    //adapter.set_discovery_filter(get_filter()).await?;    Currently disabled see below

    let mut stream = Some(Box::pin(adapter.discover_devices_with_changes().await?));

    let adapter = adapter.clone();
    tokio::task::spawn_local(async move {
        loop {
            tokio::select! {
                evt = next_event(&mut stream) => match evt {
                    // Also sent for each property change, so roughly once per advert.
                    Some(AdapterEvent::DeviceAdded(address)) => {
                        let Some((id_prefix, rssi)) = read_advert(&adapter, address).await else {
                            continue;
                        };
                        let advert = RadioEvent::Advert { address, id_prefix, rssi };
                        if events.send(advert).await.is_err() {
                            break;
                        }
                    }
                    // Out of range is judged by silence, not by DeviceRemoved.
                    Some(_) => {}
                    None => break,
                },
                changed = paused.changed() => {
                    // Running dropped its sender so BLE stopped.
                    if changed.is_err() {
                        break;
                    }
                    if *paused.borrow_and_update() {
                        log::info!("scan paused while connecting");
                        stream = None;
                    } else if stream.is_none() {
                        match adapter.discover_devices_with_changes().await {
                            Ok(new) => {
                                log::info!("scan resumed");
                                stream = Some(Box::pin(new));
                            }
                            // the one issue: Stays off until the next pause and resume.
                            Err(err) => log::warn!("resuming scan failed: {}", err),
                        }
                    }
                }
                // The loop dropped its receiver, BLE stopped.
                _ = events.closed() => break,
            }
        }
    });

    Ok(())
}

/// The next scan event, or never while paused.
async fn next_event<S: Stream<Item = AdapterEvent>>(stream: &mut Option<Pin<Box<S>>>) -> Option<AdapterEvent> {
    match stream {
        Some(s) => s.next().await,
        None => std::future::pending().await,
    }
}

/// The qaul ID prefix and signal strength from `address`'s latest advert, or
/// None if it isn't a qaul advert or wasn't heard in this scan.
async fn read_advert(adapter: &Adapter, address: Address) -> Option<([u8; QAUL_ID_ADVERT_BYTES], i16)> {
    let device = adapter.device(address).ok()?;
    // None: BlueZ knows the device from before but hasn't heard it in this scan, as rssi only exists for
    // entries found in the most recent scan.
    let rssi = device.rssi().await.ok()??;
    let data = device.manufacturer_data().await.ok()??;
    let id_prefix = data.get(&QAUL_MANUFACTURER_ID)?.get(..QAUL_ID_ADVERT_BYTES)?.try_into().ok()?;
    Some((id_prefix, rssi))
}


/// CURRENTLY FILTERING SEEMS TO CAUSE A SEGFAULT ON BLUEZ. This needs investigated further.
/// Scan filter for qaul adverts.
fn get_filter() -> bluer::DiscoveryFilter {
    let mut qaul_uuids = HashSet::new();
    qaul_uuids.insert(main_service_uuid());
    bluer::DiscoveryFilter {
        uuids: qaul_uuids,
        transport: bluer::DiscoveryTransport::Le,
        duplicate_data: false,
        ..Default::default()
    }
}
