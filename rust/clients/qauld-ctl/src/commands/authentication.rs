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
//! - `auth logout` and `auth status` have no dedicated proto messages;
//!   they're stubbed out and exit non-zero with a "not implemented"
//!   message so scripts get a clear signal.

use prost::Message;

use crate::{cli::AuthSubcmd, commands::RpcCommand, proto::Modules};

use qaul_proto::qaul_rpc_authentication as proto;

impl RpcCommand for AuthSubcmd {
    fn expects_response(&self) -> bool {
        // Only Login produces a response (the AuthChallenge).
        // Logout and Status have no wire message and short-circuit
        // inside `encode_request` with an Err.
        matches!(self, AuthSubcmd::Login { .. })
    }

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
            AuthSubcmd::Logout => Err(
                "auth logout: not yet wired to a proto message; daemon-side session \
                 lifecycle is still implicit. Use shell mode or follow up once the \
                 logout RPC is added."
                    .into(),
            ),
            AuthSubcmd::Status => Err(
                "auth status: not yet wired to a proto message; daemon-side session \
                 lifecycle is still implicit."
                    .into(),
            ),
        }
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        let envelope = proto::AuthRpc::decode(data)
            .map_err(|e| format!("auth: malformed response: {e}"))?;
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
            _ => Err("auth: unprocessable RPC response".into()),
        }
    }
}
