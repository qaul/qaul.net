// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Authentication / session subcommands.
//!
//! The auth proto defines a challenge-response handshake
//! (AuthRequest -> AuthChallenge -> AuthResponse -> AuthResult)
//! which is inherently multi-round-trip and does not fit qauld-ctl's
//! single-shot model. We surface it as best we can:
//!
//! - `auth login` sends the initial `AuthRequest` and prints the
//!   resulting `AuthChallenge`. Responding to the challenge from
//!   single-shot mode is not yet supported — that needs shell mode
//!   or the planned embedded transport (Phase 3) — so we currently
//!   exit with a clear error if the daemon answers with `AuthResult`.
//! - `auth logout` sends a `LogoutRequest` and `auth status` a
//!   `SessionStatusRequest`. Both act on the calling account, i.e. the
//!   default user resolved by the preflight request.

use prost::Message;

use crate::{cli::AuthSubcmd, commands::RpcCommand, proto::Modules};

use qaul_proto::qaul_rpc_authentication as proto;

impl RpcCommand for AuthSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        match self {
            AuthSubcmd::Login { username, .. } => {
                // The proto's AuthRequest carries a user_id (peer id bytes),
                // not a username. Until we have a username->id lookup over
                // the wire (UsersRequest), we treat the username argument as
                // a base58-encoded peer id.
                let user_id = bs58::decode(username)
                    .into_vec()
                    .map_err(|e| format!("auth: username must be base58 peer id: {e}"))?;
                let envelope = proto::AuthRpc {
                    message: Some(proto::auth_rpc::Message::AuthRequest(proto::AuthRequest {
                        user_id,
                    })),
                };
                Ok((envelope.encode_to_vec(), Modules::Auth))
            }
            AuthSubcmd::Logout => {
                let envelope = proto::AuthRpc {
                    message: Some(proto::auth_rpc::Message::LogoutRequest(
                        proto::LogoutRequest {},
                    )),
                };
                Ok((envelope.encode_to_vec(), Modules::Auth))
            }
            AuthSubcmd::Status => {
                let envelope = proto::AuthRpc {
                    message: Some(proto::auth_rpc::Message::SessionStatusRequest(
                        proto::SessionStatusRequest {},
                    )),
                };
                Ok((envelope.encode_to_vec(), Modules::Auth))
            }
        }
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        let envelope =
            proto::AuthRpc::decode(data).map_err(|e| format!("auth: malformed response: {e}"))?;
        match envelope.message {
            Some(proto::auth_rpc::Message::AuthChallenge(c)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "nonce": c.nonce,
                            "expires_at": c.expires_at,
                        }))?
                    );
                } else {
                    println!("Auth challenge received:");
                    println!("  nonce: {}", c.nonce);
                    println!("  expires_at: {}", c.expires_at);
                }
                Err(
                    "auth login is multi-step (challenge-response); responding to the \
                     challenge from single-shot mode is not yet supported. The challenge \
                     above has been printed for inspection."
                        .into(),
                )
            }
            Some(proto::auth_rpc::Message::AuthResult(r)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "success": r.success,
                            "error": r.error_message,
                        }))?
                    );
                } else if r.success {
                    println!("Authenticated.");
                } else {
                    eprintln!("Auth failed: {}", r.error_message);
                }
                if !r.success {
                    return Err(r.error_message.into());
                }
                Ok(())
            }
            Some(proto::auth_rpc::Message::SessionStatusResponse(s)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "authenticated": s.authenticated,
                        }))?
                    );
                } else if s.authenticated {
                    println!("Logged in: this account has an active session.");
                } else {
                    println!("Logged out: this account has no active session.");
                }
                Ok(())
            }
            Some(proto::auth_rpc::Message::Ack(_)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({ "ok": true }))?
                    );
                } else {
                    println!("Logged out.");
                }
                Ok(())
            }
            Some(proto::auth_rpc::Message::Error(e)) => {
                Err(format!("auth: {} (code {})", e.message, e.code).into())
            }
            _ => Err("auth: unprocessable RPC response".into()),
        }
    }
}
