// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Crypto CLI commands
//!
//! Read and write the per-node Noise session-rotation configuration
//! exposed by `libqaul::services::crypto::Crypto::rpc`.

use super::rpc::Rpc;
use prost::Message;

/// protobuf RPC definition
use qaul_proto::qaul_rpc_crypto as proto;

/// Crypto module CLI handler
pub struct Crypto {}

impl Crypto {
    /// Entry point for the `crypto …` CLI command.
    ///
    /// Supported sub-commands:
    ///
    /// - `config`                  — fetch and print the current `CryptoRotation`.
    /// - `config enable`           — enable rotation.
    /// - `config disable`          — disable rotation.
    /// - `config period <secs>`    — set `period_seconds`.
    /// - `config volume <n>`       — set `volume_messages`.
    /// - `config grace <secs>`     — set `grace_period_seconds`.
    /// - `config grace-volume <n>` — set `grace_volume_messages`.
    /// - `events [limit]`          — print recent rotation events.
    pub fn cli(command: &str) {
        let command = command.trim();

        // `events` — print the event log and return.
        if command == "events" || command.starts_with("events ") {
            let limit = command
                .strip_prefix("events")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            Self::get_events(limit);
            return;
        }

        // everything else is `config` or a subcommand of it.
        let rest = match command.strip_prefix("config") {
            Some(r) => r.trim(),
            None => {
                log::error!("unknown crypto command '{}'", command);
                return;
            }
        };

        if rest.is_empty() {
            Self::get_config();
            return;
        }

        // `config <verb> [arg]`
        let (verb, arg) = match rest.split_once(' ') {
            Some((v, a)) => (v, Some(a.trim())),
            None => (rest, None),
        };

        match (verb, arg) {
            ("enable", None) => Self::set_partial(|req| req.enabled = Some(true)),
            ("disable", None) => Self::set_partial(|req| req.enabled = Some(false)),
            ("period", Some(a)) => Self::set_u64(a, |req, v| req.period_seconds = Some(v)),
            ("volume", Some(a)) => Self::set_u64(a, |req, v| req.volume_messages = Some(v)),
            ("grace", Some(a)) => Self::set_u64(a, |req, v| req.grace_period_seconds = Some(v)),
            ("grace-volume", Some(a)) => {
                Self::set_u64(a, |req, v| req.grace_volume_messages = Some(v))
            }
            _ => {
                log::error!(
                    "unknown crypto config command '{}'. Valid: enable | disable | \
                     period <secs> | volume <n> | grace <secs> | grace-volume <n>",
                    rest
                );
            }
        }
    }

    /// Fire a `GetRotationEventsRequest`.
    fn get_events(limit: u32) {
        let msg = proto::Crypto {
            message: Some(proto::crypto::Message::GetEventsRequest(
                proto::GetRotationEventsRequest {
                    since_ms: 0,
                    limit,
                },
            )),
        };
        Self::send(msg);
    }

    /// Fire a `GetConfigRequest`.
    fn get_config() {
        let msg = proto::Crypto {
            message: Some(proto::crypto::Message::GetConfigRequest(
                proto::GetConfigRequest {},
            )),
        };
        Self::send(msg);
    }

    /// Build a `SetConfigRequest` with every field `None`, let the
    /// caller flip exactly the fields they want, and send.
    fn set_partial(f: impl FnOnce(&mut proto::SetConfigRequest)) {
        let mut req = proto::SetConfigRequest {
            enabled: None,
            period_seconds: None,
            volume_messages: None,
            grace_period_seconds: None,
            grace_volume_messages: None,
        };
        f(&mut req);
        let msg = proto::Crypto {
            message: Some(proto::crypto::Message::SetConfigRequest(req)),
        };
        Self::send(msg);
    }

    /// Parse a `u64` from CLI input and fire a partial `SetConfigRequest`.
    fn set_u64(raw: &str, f: impl FnOnce(&mut proto::SetConfigRequest, u64)) {
        match raw.parse::<u64>() {
            Ok(v) => Self::set_partial(|req| f(req, v)),
            Err(e) => log::error!("expected a non-negative integer, got '{}': {}", raw, e),
        }
    }

    fn send(msg: proto::Crypto) {
        let mut buf = Vec::with_capacity(msg.encoded_len());
        msg.encode(&mut buf).expect("encoding cannot fail");
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Crypto.into(),
            "".to_string(),
        );
    }

    /// Render an incoming Crypto RPC response. Called from rpc.rs
    /// when a `Modules::Crypto` message arrives.
    pub fn rpc(data: Vec<u8>) {
        match proto::Crypto::decode(&data[..]) {
            Ok(msg) => match msg.message {
                Some(proto::crypto::Message::GetConfigResponse(resp)) => {
                    Self::print_config(&resp);
                }
                Some(proto::crypto::Message::SetConfigResponse(resp)) => {
                    if resp.success {
                        println!("crypto rotation config updated:");
                    } else {
                        println!("crypto rotation config update FAILED: {}", resp.error);
                    }
                    if let Some(applied) = resp.applied {
                        Self::print_config(&applied);
                    }
                }
                Some(proto::crypto::Message::GetEventsResponse(resp)) => {
                    Self::print_events(&resp);
                }
                Some(proto::crypto::Message::GetConfigRequest(_))
                | Some(proto::crypto::Message::SetConfigRequest(_))
                | Some(proto::crypto::Message::GetEventsRequest(_)) => {
                    // client should never see its own requests echoed back
                }
                None => log::warn!("empty crypto RPC response"),
            },
            Err(e) => log::error!("failed to decode crypto RPC response: {}", e),
        }
    }

    fn print_config(cfg: &proto::GetConfigResponse) {
        println!("  enabled               : {}", cfg.enabled);
        println!("  period_seconds        : {}", cfg.period_seconds);
        println!("  volume_messages       : {}", cfg.volume_messages);
        println!("  grace_period_seconds  : {}", cfg.grace_period_seconds);
        println!("  grace_volume_messages : {}", cfg.grace_volume_messages);
    }

    fn print_events(resp: &proto::GetRotationEventsResponse) {
        if resp.events.is_empty() {
            println!("(no rotation events recorded)");
            return;
        }
        println!(
            "{:<15} | {:<25} | {:<52} | {:>11} | {:>11}",
            "timestamp_ms", "kind", "remote_id", "primary", "draining"
        );
        for e in &resp.events {
            let kind = match proto::RotationEventKind::try_from(e.kind) {
                Ok(proto::RotationEventKind::Rotated) => "Rotated",
                Ok(proto::RotationEventKind::GraceExpired) => "GraceExpired",
                Ok(proto::RotationEventKind::MessageDroppedPastGrace) => {
                    "MessageDroppedPastGrace"
                }
                _ => "Unspecified",
            };
            let remote = bs58::encode(&e.remote_id).into_string();
            println!(
                "{:<15} | {:<25} | {:<52} | {:>11} | {:>11}",
                e.timestamp_ms, kind, remote, e.primary_session_id, e.draining_session_id
            );
        }
    }
}
