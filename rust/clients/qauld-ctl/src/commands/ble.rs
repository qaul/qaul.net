// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! BLE module controls (status, start/stop, discovered peers).
//!
//! Ported from `rust/clients/cli/src/ble.rs` with the transport swapped
//! to qauld-ctl's `RpcCommand` trait and Phase 0 hardening applied
//! (errors return `Err`, error output to stderr, JSON support).

use prost::Message;
use serde_json::json;

use crate::{cli::BleSubcmd, commands::RpcCommand, proto::Modules};

use qaul_proto::qaul_rpc_ble as proto;
use qaul_proto::qaul_sys_ble as proto_sys;

impl RpcCommand for BleSubcmd {
    fn expects_response(&self) -> bool {
        // Info / Discovered round-trip; Start / Stop are fire-and-forget
        // on the wire (libqaul does not emit a response for them).
        matches!(self, BleSubcmd::Info | BleSubcmd::Discovered)
    }

    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let envelope = match self {
            BleSubcmd::Info => proto::Ble {
                message: Some(proto::ble::Message::InfoRequest(proto::InfoRequest {})),
            },
            BleSubcmd::Start => proto::Ble {
                message: Some(proto::ble::Message::StartRequest(proto::StartRequest {})),
            },
            BleSubcmd::Stop => proto::Ble {
                message: Some(proto::ble::Message::StopRequest(proto::StopRequest {})),
            },
            BleSubcmd::Discovered => proto::Ble {
                message: Some(proto::ble::Message::DiscoveredRequest(
                    proto::DiscoveredRequest {},
                )),
            },
        };
        Ok((envelope.encode_to_vec(), Modules::Ble))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        match proto::Ble::decode(data) {
            Ok(envelope) => match envelope.message {
                Some(proto::ble::Message::InfoResponse(info)) => print_info(info, json)?,
                Some(proto::ble::Message::DiscoveredResponse(d)) => print_discovered(d, json)?,
                Some(proto::ble::Message::RightsRequest(_)) => {
                    return Err("BLE rights requested by daemon (handle via system prompt)".into());
                }
                _ => return Err("unprocessable RPC BLE message".into()),
            },
            Err(e) => return Err(format!("ble: failed to decode response: {e}").into()),
        }
        Ok(())
    }
}

fn print_info(info: proto::InfoResponse, json_out: bool) -> Result<(), Box<dyn std::error::Error>> {
    let device = proto_sys::BleDeviceInfo::decode(&info.device_info[..]).ok();
    let q8id = bs58::encode(&info.q8id).into_string();

    if json_out {
        let device_json = match &device {
            Some(d) if d.ble_support => Some(json!({
                "ble_support": true,
                "id": d.id,
                "name": d.name,
                "bluetooth_on": d.bluetooth_on,
                "adv_extended": d.adv_extended,
                "adv_extended_bytes": d.adv_extended_bytes,
                "le_2m": d.le_2m,
                "le_audio": d.le_audio,
                "le_periodic_adv_support": d.le_periodic_adv_support,
                "le_multiple_adv_support": d.le_multiple_adv_support,
                "offload_filter_support": d.offload_filter_support,
                "offload_scan_batching_support": d.offload_scan_batching_support,
            })),
            Some(_) => Some(json!({ "ble_support": false })),
            None => None,
        };
        let obj = json!({
            "q8id": q8id,
            "status": info.status,
            "device": device_json,
        });
        println!("{}", serde_json::to_string_pretty(&obj)?);
    } else {
        println!("Node small BLE ID: {}", q8id);
        println!("BLE module status: {}", info.status);
        match device {
            Some(d) if d.ble_support => {
                println!("Device ID: {}", d.id);
                println!("Device Name: {}", d.name);
                println!("Bluetooth On: {}", d.bluetooth_on);
                println!("Extended Advertisement Supported: {}", d.adv_extended);
                if d.adv_extended {
                    println!("Extended Advertisement Bytes: {}", d.adv_extended_bytes);
                }
                println!("2M Supported: {}", d.le_2m);
                println!("Audio Supported: {}", d.le_audio);
                println!("Periodic Advertisement Supported: {}", d.le_periodic_adv_support);
                println!("Multiple Advertisement Supported: {}", d.le_multiple_adv_support);
                println!("Offload Filter Supported: {}", d.offload_filter_support);
                println!("Offload Scan Batching Supported: {}", d.offload_scan_batching_support);
            }
            Some(_) => println!("BLE not supported by this device"),
            None => eprintln!("warning: could not decode device_info"),
        }
    }
    Ok(())
}

fn print_discovered(
    d: proto::DiscoveredResponse,
    json_out: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if json_out {
        let obj = json!({
            "nodes_count": d.nodes_count,
            "to_confirm_count": d.to_confirm_count,
        });
        println!("{}", serde_json::to_string_pretty(&obj)?);
    } else {
        println!("Nodes Count: {}", d.nodes_count);
        println!("To Confirm Count: {}", d.to_confirm_count);
    }
    Ok(())
}
