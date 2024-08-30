// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # BLE Module
//! 
//! Control functions for the Bluetooth Low Energy Module.

use prost::Message;
use super::rpc::Rpc;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.ble.rs"); }
mod proto_sys { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.sys.ble.rs"); }

/// BLE Module Function Handling
pub struct Ble {}

impl Ble {
    /// CLI command interpretation
    /// 
    /// The CLI commands of BLE module are processed here
    pub fn cli(command: &str) {
        match command {
            // request BLE device info
            cmd if cmd.starts_with("info") => {
                // create rpc message
                let proto_message = proto::Ble {
                    message: Some(proto::ble::Message::InfoRequest(
                        proto::InfoRequest{}
                    )),
                };
                // send the message
                Self::rpc_send(proto_message);
            },
            // send start request for BLE module
            cmd if cmd.starts_with("start") => {
                // create rpc message
                let proto_message = proto::Ble {
                    message: Some(proto::ble::Message::StartRequest(
                        proto::StartRequest{}
                    )),
                };
                // send the message
                Self::rpc_send(proto_message);
            },
            // send stop request for BLE module
            cmd if cmd.starts_with("stop") => {
                // create rpc message
                let proto_message = proto::Ble {
                    message: Some(proto::ble::Message::StopRequest(
                        proto::StopRequest{}
                    )),
                };
                // send the message
                Self::rpc_send(proto_message);
            },
            // request discovered devices
            cmd if cmd.starts_with("discovered") => {
                // create rpc message
                let proto_message = proto::Ble {
                    message: Some(proto::ble::Message::DiscoveredRequest(
                        proto::DiscoveredRequest{}
                    )),
                };
                // send the message
                Self::rpc_send(proto_message);
            },
            // unknown command
            _ => log::error!("unknown BLE command"),
        }
    }

    /// Send rpc message to libqaul
    fn rpc_send(proto_message: proto::Ble) {
        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Ble.into(), "".to_string());
    }

    /// Print BLE module information
    fn print_info(info: proto::InfoResponse) {
        println!("Node small BLE ID: {:?}", info.small_id);
        println!("BLE module status: {}", info.status);
        
        // decode device info
        match proto_sys::BleDeviceInfo::decode(&info.device_info[..]) {
            Ok(device) => {
                // print device info
                if device.ble_support {
                    println!("Device ID: {}", device.id);
                    println!("Device Name: {}", device.name);
                    println!("Bluetooth On: {:?}", device.bluetooth_on);
                    println!("Extended Advertisement Supported: {:?}", device.adv_extended);
                    if device.adv_extended {
                        println!("Extended Advertisements Bytes: {:?}", device.adv_extended_bytes);
                    }
                    println!("2M Supported: {:?}", device.le_2m);
                    println!("Audio Supported: {:?}", device.le_audio);
                    println!("Periodic Advertisement Supported: {:?}", device.le_periodic_adv_support);
                    println!("Multiple Advertisement Supported: {:?}", device.le_multiple_adv_support);
                    println!("Offload Filer Supported: {:?}", device.offload_filter_support);
                    println!("Offload Scan Batching Supported: {:?}", device.offload_scan_batching_support);
                }
                else {
                    println!("BLE not supported");
                }
            },
            Err(e) => {
                log::error!("{:?}", e);
            },
        }
    }

    /// Print Discovered BLE Nodes
    fn print_discovered(discovered: proto::DiscoveredResponse) {
        println!("Nodes Count: {}", discovered.nodes_count);
        println!("To Confirm Count: {}", discovered.to_confirm_count);
    }

    /// Process received RPC message
    /// 
    /// Decodes received protobuf encoded binary RPC message
    /// of the BLE module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Ble::decode(&data[..]) {
            Ok(ble) => {
                match ble.message {
                    Some(proto::ble::Message::InfoResponse(info)) => {
                        Self::print_info(info);
                    }
                    Some(proto::ble::Message::DiscoveredResponse(discovered)) => {
                        Self::print_discovered(discovered);
                    }
                    Some(proto::ble::Message::RightsRequest(_)) => {
                        log::error!("BLE rights requested");
                    }
                    _ => {
                        log::error!("unprocessable RPC debug message");
                    },
                }    
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }
}