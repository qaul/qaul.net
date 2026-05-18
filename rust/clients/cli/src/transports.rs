// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Transports management
//!
//! Drives the `Modules::Transports` RPC: list every registered
//! transport (LAN, Internet, BLE) and toggle each one. The
//! enabled/disabled choice is persisted by libqaul to `config.yaml`,
//! so it survives a daemon restart.

use prost::Message;

use super::rpc::Rpc;
use qaul_proto::qaul_rpc_transports as proto;

/// `transports …` CLI handler.
pub struct Transports {}

impl Transports {
    /// Entry point for the `transports …` CLI command.
    ///
    /// Sub-commands:
    /// - `transports list` — print every registered transport with
    ///   its current status.
    /// - `transports enable <id>` — start the transport identified
    ///   by `id` (`lan`, `internet`, `ble`).
    /// - `transports disable <id>` — stop it.
    pub fn cli(state: &super::CliState, command: &str) {
        let command = command.trim();

        if command == "list" {
            Self::list(state);
            return;
        }

        if let Some(id) = command.strip_prefix("enable ") {
            let id = id.trim();
            if id.is_empty() {
                log::error!("usage: transports enable <id>");
                return;
            }
            Self::set_enabled(state, id, true);
            return;
        }

        if let Some(id) = command.strip_prefix("disable ") {
            let id = id.trim();
            if id.is_empty() {
                log::error!("usage: transports disable <id>");
                return;
            }
            Self::set_enabled(state, id, false);
            return;
        }

        log::error!(
            "unknown transports command '{}'. Valid: list | enable <id> | disable <id>",
            command
        );
    }

    fn list(state: &super::CliState) {
        let msg = proto::Transports {
            message: Some(proto::transports::Message::ListRequest(
                proto::TransportsListRequest {},
            )),
        };
        Self::send(state, msg);
    }

    fn set_enabled(state: &super::CliState, id: &str, enabled: bool) {
        let msg = proto::Transports {
            message: Some(proto::transports::Message::SetEnabled(
                proto::TransportSetEnabled {
                    id: id.to_string(),
                    enabled,
                },
            )),
        };
        Self::send(state, msg);
    }

    fn send(state: &super::CliState, msg: proto::Transports) {
        let mut buf = Vec::with_capacity(msg.encoded_len());
        msg.encode(&mut buf).expect("encoding cannot fail");
        Rpc::send_message(
            state,
            buf,
            super::rpc::proto::Modules::Transports.into(),
            "".to_string(),
        );
    }

    /// Render an incoming Transports RPC response.
    pub fn rpc(data: Vec<u8>) {
        match proto::Transports::decode(&data[..]) {
            Ok(msg) => match msg.message {
                Some(proto::transports::Message::List(list)) => Self::print_list(&list),
                Some(proto::transports::Message::SetEnabledResult(result)) => {
                    Self::print_set_enabled_result(&result)
                }
                Some(proto::transports::Message::ListRequest(_))
                | Some(proto::transports::Message::SetEnabled(_)) => {
                    // requests echoed back are nothing to render.
                }
                None => log::warn!("empty transports RPC response"),
            },
            Err(e) => log::error!("failed to decode transports RPC response: {}", e),
        }
    }

    fn print_list(list: &proto::TransportsList) {
        if list.transports.is_empty() {
            println!("(no transports registered)");
            return;
        }
        println!(
            "{:<10} | {:<20} | {:<10} | {:>7} | {:>13} | {:>11}",
            "id", "label", "status", "enabled", "runtime_toggle", "local_only"
        );
        for t in &list.transports {
            println!(
                "{:<10} | {:<20} | {:<10} | {:>7} | {:>13} | {:>11}",
                t.id,
                t.label,
                t.status,
                t.enabled,
                t.supports_runtime_toggle,
                t.is_local_only,
            );
        }
    }

    fn print_set_enabled_result(result: &proto::TransportSetEnabledResult) {
        if result.success {
            println!("transport '{}' updated", result.id);
        } else {
            println!(
                "transport '{}' update FAILED: {}",
                result.id, result.error
            );
        }
    }
}
