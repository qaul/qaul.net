use crate::{cli::CryptoSubcmd, commands::RpcCommand, proto::Modules};
use prost::Message;

use qaul_proto::qaul_rpc_crypto as proto;

impl CryptoSubcmd {
    fn print_config_human(cfg: &proto::GetConfigResponse) {
        println!("crypto rotation config:");
        println!("  enabled               : {}", cfg.enabled);
        println!("  period_seconds        : {}", cfg.period_seconds);
        println!("  volume_messages       : {}", cfg.volume_messages);
        println!("  grace_period_seconds  : {}", cfg.grace_period_seconds);
        println!("  grace_volume_messages : {}", cfg.grace_volume_messages);
    }

    fn config_to_json(cfg: &proto::GetConfigResponse) -> serde_json::Value {
        serde_json::json!({
            "enabled": cfg.enabled,
            "period_seconds": cfg.period_seconds,
            "volume_messages": cfg.volume_messages,
            "grace_period_seconds": cfg.grace_period_seconds,
            "grace_volume_messages": cfg.grace_volume_messages,
        })
    }

    fn kind_str(kind: i32) -> &'static str {
        match proto::RotationEventKind::try_from(kind) {
            Ok(proto::RotationEventKind::Rotated) => "Rotated",
            Ok(proto::RotationEventKind::GraceExpired) => "GraceExpired",
            Ok(proto::RotationEventKind::MessageDroppedPastGrace) => "MessageDroppedPastGrace",
            _ => "Unspecified",
        }
    }
}

impl RpcCommand for CryptoSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let msg = match self {
            CryptoSubcmd::Config => proto::Crypto {
                message: Some(proto::crypto::Message::GetConfigRequest(
                    proto::GetConfigRequest {},
                )),
            },
            CryptoSubcmd::Enable => proto::Crypto {
                message: Some(proto::crypto::Message::SetConfigRequest(
                    proto::SetConfigRequest {
                        enabled: Some(true),
                        ..Default::default()
                    },
                )),
            },
            CryptoSubcmd::Disable => proto::Crypto {
                message: Some(proto::crypto::Message::SetConfigRequest(
                    proto::SetConfigRequest {
                        enabled: Some(false),
                        ..Default::default()
                    },
                )),
            },
            CryptoSubcmd::Set {
                period_seconds,
                volume_messages,
                grace_period_seconds,
                grace_volume_messages,
            } => {
                if period_seconds.is_none()
                    && volume_messages.is_none()
                    && grace_period_seconds.is_none()
                    && grace_volume_messages.is_none()
                {
                    return Err(
                        "crypto set: pass at least one of --period-seconds, \
                         --volume-messages, --grace-period-seconds, --grace-volume-messages"
                            .into(),
                    );
                }
                proto::Crypto {
                    message: Some(proto::crypto::Message::SetConfigRequest(
                        proto::SetConfigRequest {
                            enabled: None,
                            period_seconds: *period_seconds,
                            volume_messages: *volume_messages,
                            grace_period_seconds: *grace_period_seconds,
                            grace_volume_messages: *grace_volume_messages,
                        },
                    )),
                }
            }
            CryptoSubcmd::Rotate { user_id } => {
                let remote_id = bs58::decode(user_id)
                    .into_vec()
                    .map_err(|e| format!("invalid user-id '{}': {}", user_id, e))?;
                proto::Crypto {
                    message: Some(proto::crypto::Message::TriggerRotationRequest(
                        proto::TriggerRotationRequest { remote_id },
                    )),
                }
            }
            CryptoSubcmd::Events { limit, since_ms } => proto::Crypto {
                message: Some(proto::crypto::Message::GetEventsRequest(
                    proto::GetRotationEventsRequest {
                        since_ms: *since_ms,
                        limit: *limit,
                    },
                )),
            },
        };

        Ok((msg.encode_to_vec(), Modules::Crypto))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        let crypto = proto::Crypto::decode(data)?;
        match crypto.message {
            Some(proto::crypto::Message::GetConfigResponse(resp)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&Self::config_to_json(&resp))?
                    );
                } else {
                    Self::print_config_human(&resp);
                }
            }
            Some(proto::crypto::Message::SetConfigResponse(resp)) => {
                let applied_json = resp
                    .applied
                    .as_ref()
                    .map(Self::config_to_json)
                    .unwrap_or(serde_json::Value::Null);
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "success": resp.success,
                            "error": resp.error,
                            "applied": applied_json,
                        }))?
                    );
                } else if resp.success {
                    println!("crypto rotation config updated:");
                    if let Some(applied) = resp.applied {
                        Self::print_config_human(&applied);
                    }
                } else {
                    println!("crypto rotation config update FAILED: {}", resp.error);
                    if let Some(applied) = resp.applied {
                        Self::print_config_human(&applied);
                    }
                }
            }
            Some(proto::crypto::Message::TriggerRotationResponse(resp)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "success": resp.success,
                            "error": resp.error,
                            "new_session_id": resp.new_session_id,
                            "previous_session_id": resp.previous_session_id,
                        }))?
                    );
                } else if resp.success {
                    println!(
                        "rotation triggered: previous_session={} new_session={}",
                        resp.previous_session_id, resp.new_session_id
                    );
                } else {
                    println!("rotation trigger FAILED: {}", resp.error);
                }
            }
            Some(proto::crypto::Message::GetEventsResponse(resp)) => {
                if json {
                    let events: Vec<serde_json::Value> = resp
                        .events
                        .iter()
                        .map(|e| {
                            serde_json::json!({
                                "timestamp_ms": e.timestamp_ms,
                                "kind": Self::kind_str(e.kind),
                                "remote_id": bs58::encode(&e.remote_id).into_string(),
                                "primary_session_id": e.primary_session_id,
                                "draining_session_id": e.draining_session_id,
                            })
                        })
                        .collect();
                    println!("{}", serde_json::to_string_pretty(&events)?);
                } else if resp.events.is_empty() {
                    println!("(no rotation events recorded)");
                } else {
                    println!(
                        "{:<15} | {:<25} | {:<52} | {:>11} | {:>11}",
                        "timestamp_ms", "kind", "remote_id", "primary", "draining"
                    );
                    for e in &resp.events {
                        println!(
                            "{:<15} | {:<25} | {:<52} | {:>11} | {:>11}",
                            e.timestamp_ms,
                            Self::kind_str(e.kind),
                            bs58::encode(&e.remote_id).into_string(),
                            e.primary_session_id,
                            e.draining_session_id,
                        );
                    }
                }
            }
            _ => {
                log::error!("unprocessable RPC crypto message");
            }
        }
        Ok(())
    }
}
