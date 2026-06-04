// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! WebRTC voice/video signalling subcommands.
//!
//! Only compiled when the `rtc` feature is enabled. Mirrors the
//! request/accept/decline/end/list flow exposed by libqaul's RTC RPC.
//! Frame-level audio/video traffic (`RtcOutgoing`/`RtcIncoming`) is
//! intentionally not surfaced here — that's a streaming concern that
//! doesn't fit the single-shot model.

use prost::Message;

use crate::{cli::RtcSubcmd, commands::RpcCommand, proto::Modules};

use super::uuid_string_to_bin;

use qaul_proto::qaul_rpc_rtc as proto;

impl RpcCommand for RtcSubcmd {
    fn expects_response(&self) -> bool {
        // Only `list` round-trips; request/accept/decline/end are
        // fire-and-forget signals that libqaul ack's via subscribe
        // events, not direct responses.
        matches!(self, RtcSubcmd::List)
    }

    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let envelope = match self {
            RtcSubcmd::List => proto::RtcRpc {
                message: Some(proto::rtc_rpc::Message::RtcSessionListRequest(
                    proto::RtcSessionListRequest {},
                )),
            },
            RtcSubcmd::Request { group_id } => proto::RtcRpc {
                message: Some(proto::rtc_rpc::Message::RtcSessionRequest(
                    proto::RtcSessionRequest {
                        group_id: uuid_string_to_bin(group_id.clone())?,
                    },
                )),
            },
            RtcSubcmd::Accept { group_id } => management(group_id, 1)?,
            RtcSubcmd::Decline { group_id } => management(group_id, 2)?,
            RtcSubcmd::End { group_id } => management(group_id, 3)?,
        };
        Ok((envelope.encode_to_vec(), Modules::Rtc))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        let envelope = proto::RtcRpc::decode(data)
            .map_err(|e| format!("rtc: malformed response: {e}"))?;
        match envelope.message {
            Some(proto::rtc_rpc::Message::RtcSessionListResponse(list)) => {
                if json {
                    let sessions: Vec<serde_json::Value> = list
                        .sessions
                        .iter()
                        .map(|s| {
                            serde_json::json!({
                                "group_id": bs58::encode(&s.group_id).into_string(),
                                "session_type": s.session_type,
                                "state": s.state,
                                "created_at": s.created_at,
                            })
                        })
                        .collect();
                    println!("{}", serde_json::to_string_pretty(&sessions)?);
                } else if list.sessions.is_empty() {
                    println!("(no active RTC sessions)");
                } else {
                    println!("RTC sessions:");
                    for s in list.sessions {
                        println!(
                            "  group_id={} type={} state={} created_at={}",
                            bs58::encode(&s.group_id).into_string(),
                            s.session_type,
                            s.state,
                            s.created_at,
                        );
                    }
                }
                Ok(())
            }
            _ => Err("rtc: unprocessable RPC response".into()),
        }
    }
}

fn management(
    group_id: &str,
    option: u32,
) -> Result<proto::RtcRpc, Box<dyn std::error::Error>> {
    Ok(proto::RtcRpc {
        message: Some(proto::rtc_rpc::Message::RtcSessionManagement(
            proto::RtcSessionManagement {
                group_id: uuid_string_to_bin(group_id.to_string())?,
                option,
            },
        )),
    })
}
