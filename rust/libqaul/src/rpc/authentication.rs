// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Authentication Module
//!
//! This module handles authentication on the libqaul node side.
//! It manages challenge-response authentication using Argon2 password verification,
//! tracks active authentication challenges, and maintains authenticated sessions.

use crate::node::user_accounts::UserAccounts;
use crate::rpc::Rpc;
use crate::utilities::timestamp::Timestamp;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use libp2p::PeerId;
use std::collections::BTreeMap;
use std::sync::RwLock;

/// Protobuf message definitions for authentication RPC
pub use qaul_proto::qaul_rpc_authentication as proto;
/// Shared RPC response / error types used by the generated service dispatch.
use qaul_proto::qaul_common::{Ack, RpcError};

/// Active authentication challenge for a user
#[allow(dead_code)]
#[derive(Clone)]
pub struct AuthChallenge {
    pub nonce: u64,
    pub user_id: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
}

/// Instance-based authentication state.
/// Replaces the global NONCE_COUNTER, ACTIVE_CHALLENGES, AUTHENTICATED_USERS
/// statics for multi-instance use.
pub struct AuthenticationState {
    /// Counter for generating unique nonces.
    pub nonce_counter: RwLock<u64>,
    /// Map of active authentication challenges indexed by user ID.
    pub active_challenges: RwLock<BTreeMap<Vec<u8>, AuthChallenge>>,
    /// Map of authenticated users with their session expiration times.
    pub authenticated_users: RwLock<BTreeMap<Vec<u8>, u64>>,
}

impl AuthenticationState {
    /// Create a new AuthenticationState.
    pub fn new() -> Self {
        Self {
            nonce_counter: RwLock::new(1),
            active_challenges: RwLock::new(BTreeMap::new()),
            authenticated_users: RwLock::new(BTreeMap::new()),
        }
    }
}

pub struct Authentication {}

#[allow(dead_code)]
impl Authentication {
    /// Generate the next unique nonce
    fn next_nonce(state: &crate::QaulState) -> u64 {
        let mut c = state.auth.nonce_counter.write().unwrap();
        let nonce = *c;
        *c += 1;
        nonce
    }

    /// Create an authentication challenge for a user
    pub fn create_challenge(state: &crate::QaulState, user_id: PeerId) -> Result<u64, String> {
        println!(
            "LIBQAUL: Creating challenge for user_id: {:?}",
            user_id.to_bytes()
        );
        // Verify user exists in the system
        if UserAccounts::get_by_id(state, user_id).is_none() {
            return Err("User not found".to_string());
        }

        log::info!("Storing challenge with key: {:?}", user_id.to_bytes());

        let nonce = Self::next_nonce(state);
        let now = Timestamp::get_timestamp();
        let user_id_bytes = user_id.to_bytes();

        let challenge = AuthChallenge {
            nonce,
            user_id: user_id_bytes.clone(),
            created_at: now,
            expires_at: now + 9999999999,
        };

        let mut challenges = state.auth.active_challenges.write().unwrap();
        challenges.insert(user_id_bytes, challenge);
        challenges.retain(|_, c| now < c.expires_at);

        Ok(nonce)
    }

    /// Verify a challenge response from the client
    pub fn verify_challenge(state: &crate::QaulState, user_id: PeerId, challenge_hash: Vec<u8>) -> Result<bool, String> {
        let now = Timestamp::get_timestamp();
        let user_id_bytes = user_id.to_bytes();

        log::info!("Verifying challenge at timestamp: {}", now);
        log::info!("Looking for challenge with key: {:?}", user_id_bytes);

        // Debug: print all active challenges
        {
            let challenges = state.auth.active_challenges.read().unwrap();
            log::info!("Total active challenges: {}", challenges.len());
            for (key, challenge) in challenges.iter() {
                log::info!(
                    "Challenge key: {:?}, expires_at: {}, current_time: {}",
                    key,
                    challenge.expires_at,
                    now
                );
            }
        }

        let user = UserAccounts::get_by_id(state, user_id).ok_or("User not found".to_string())?;

        let challenge = {
            let challenges = state.auth.active_challenges.read().unwrap();
            challenges.get(&user_id_bytes).cloned()
        };

        let challenge = challenge.ok_or("No active challenge or challenge expired".to_string())?;

        let stored_hash = match user.password_hash {
            Some(hash) => hash,
            None => {
                // No password set - remove challenge and authenticate
                let mut challenges = state.auth.active_challenges.write().unwrap();
                challenges.remove(&user_id_bytes);
                Self::mark_authenticated(state, user_id);
                return Ok(true);
            }
        };

        let nonce_str = challenge.nonce.to_string();
        let combined = format!("{}{}", stored_hash, nonce_str);

        let challenge_hash_str = String::from_utf8(challenge_hash)
            .map_err(|_| "Invalid challenge hash encoding".to_string())?;

        let received_hash = PasswordHash::new(&challenge_hash_str)
            .map_err(|e| format!("Invalid password hash format: {}", e))?;

        let argon2 = Argon2::default();
        match argon2.verify_password(combined.as_bytes(), &received_hash) {
            Ok(()) => {
                Self::mark_authenticated(state, user_id);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    /// Mark a user as authenticated with a session
    fn mark_authenticated(state: &crate::QaulState, user_id: PeerId) {
        let mut auth = state.auth.authenticated_users.write().unwrap();
        let expires_at = Timestamp::get_timestamp() + (86400 * 365 * 100);
        auth.insert(user_id.to_bytes(), expires_at);
    }

    /// Check if a user has an active authenticated session
    pub fn is_authenticated(state: &crate::QaulState, user_id: PeerId) -> bool {
        let now = Timestamp::get_timestamp();
        let mut auth = state.auth.authenticated_users.write().unwrap();
        auth.retain(|_, &mut expires_at| now < expires_at);
        auth.contains_key(&user_id.to_bytes())
    }

    /// Logout a user by removing their authenticated session
    pub fn logout(state: &crate::QaulState, user_id: PeerId) {
        let mut auth = state.auth.authenticated_users.write().unwrap();
        auth.remove(&user_id.to_bytes());
    }

    /// Process incoming authentication RPC messages.
    ///
    /// Thin shim over the generated service dispatcher: it decodes the
    /// `AuthRpc` envelope, routes the request to the matching
    /// `AuthRpcService` method, and encodes the reply — then sends it back
    /// on the AUTH module channel. All per-method logic lives in the
    /// `AuthRpcService` impl below.
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
        let ctx = crate::RequestContext {
            state,
            user_id,
            request_id: request_id.clone(),
        };
        let response_bytes = proto::dispatch::<crate::RequestContext, Authentication>(&ctx, data);
        Rpc::send_message(
            state,
            response_bytes,
            crate::rpc::proto::Modules::Auth.into(),
            request_id,
            Vec::new(),
        );
    }
}

/// Generated-service implementation: the per-method business logic that the
/// `AuthRpcService` dispatcher routes to. Each method returns a typed
/// `Result<Resp, RpcError>`; the dispatcher maps `Err(_)` to the uniform
/// `error` oneof variant on the wire.
///
/// The dispatcher threads a [`crate::RequestContext`], which carries the node
/// `state` plus the caller's `user_id`/`request_id` from the outer `QaulRpc`
/// envelope. The login handshake (`request_challenge` / `respond_challenge`)
/// reads the *target* `user_id` from the request body — there is no
/// authenticated caller yet, so it can't come from the context. The
/// self-scoped operations (`logout_session` / `session_status`) instead act on
/// the caller's `ctx.user_id`, so they carry no id in the request and cannot
/// act on another account.
impl proto::AuthRpcService<crate::RequestContext<'_>> for Authentication {
    /// List the user accounts available on this node (for the login picker).
    fn users(
        ctx: &crate::RequestContext<'_>,
        _req: proto::UsersRequest,
    ) -> Result<proto::UsersResponse, RpcError> {
        let state = ctx.state;
        let config = crate::storage::configuration::Configuration::get(state);

        let mut users_list = Vec::new();
        for user_config in &config.user_accounts {
            let user_id = match user_config.id.parse::<PeerId>() {
                Ok(id) => id.to_bytes(),
                Err(_) => continue,
            };

            let salt = if let Some(ref s) = user_config.password_salt {
                Some(s.clone())
            } else if let Some(ref hash) = user_config.password_hash {
                let parts: Vec<&str> = hash.split('$').collect();
                if parts.len() >= 5 {
                    Some(parts[4].to_string())
                } else {
                    None
                }
            } else {
                None
            };

            users_list.push(proto::UserInfo {
                username: user_config.name.clone(),
                user_id,
                salt,
                has_password: user_config.password_hash.is_some(),
            });
        }

        Ok(proto::UsersResponse {
            users: users_list,
            error_message: String::new(),
        })
    }

    /// Step 1 of login: create and return an authentication challenge.
    fn request_challenge(
        ctx: &crate::RequestContext<'_>,
        req: proto::AuthRequest,
    ) -> Result<proto::AuthChallenge, RpcError> {
        let state = ctx.state;
        let peer_id = PeerId::from_bytes(&req.user_id).map_err(|_| RpcError {
            code: 1,
            message: "Invalid qaul ID".to_string(),
            details: String::new(),
        })?;

        match Self::create_challenge(state, peer_id) {
            Ok(nonce) => Ok(proto::AuthChallenge {
                nonce,
                expires_at: Timestamp::get_timestamp() + 300,
            }),
            Err(e) => Err(RpcError {
                code: 2,
                message: e,
                details: String::new(),
            }),
        }
    }

    /// Step 2 of login: verify the challenge response and authenticate.
    ///
    /// Authentication outcomes (success / wrong credentials / expired
    /// challenge) are returned as an `AuthResult`, mirroring the previous
    /// hand-written handler. Only a malformed `user_id` surfaces as an
    /// `RpcError`.
    fn respond_challenge(
        ctx: &crate::RequestContext<'_>,
        req: proto::AuthResponse,
    ) -> Result<proto::AuthResult, RpcError> {
        let state = ctx.state;
        let peer_id = PeerId::from_bytes(&req.user_id).map_err(|_| RpcError {
            code: 1,
            message: "Invalid user ID".to_string(),
            details: String::new(),
        })?;

        match Self::verify_challenge(state, peer_id, req.challenge_hash) {
            Ok(true) => Ok(proto::AuthResult {
                success: true,
                error_message: "Authentication successful".to_string(),
            }),
            Ok(false) => Ok(proto::AuthResult {
                success: false,
                error_message: "Invalid credentials".to_string(),
            }),
            Err(e) => Ok(proto::AuthResult {
                success: false,
                error_message: e,
            }),
        }
    }

    /// Drop the daemon-side authenticated session for the calling user.
    ///
    /// Identity is taken from the request context (the outer envelope), not the
    /// request body — a caller can only log itself out.
    fn logout_session(
        ctx: &crate::RequestContext<'_>,
        _req: proto::LogoutRequest,
    ) -> Result<Ack, RpcError> {
        let peer_id = PeerId::from_bytes(&ctx.user_id).map_err(|_| RpcError {
            code: 1,
            message: "Invalid caller identity".to_string(),
            details: String::new(),
        })?;

        Self::logout(ctx.state, peer_id);
        Ok(Ack {})
    }

    /// Report whether the calling user currently has an active authenticated
    /// session. Identity comes from the request context (the outer envelope).
    fn session_status(
        ctx: &crate::RequestContext<'_>,
        _req: proto::SessionStatusRequest,
    ) -> Result<proto::SessionStatusResponse, RpcError> {
        let peer_id = PeerId::from_bytes(&ctx.user_id).map_err(|_| RpcError {
            code: 1,
            message: "Invalid caller identity".to_string(),
            details: String::new(),
        })?;

        Ok(proto::SessionStatusResponse {
            authenticated: Self::is_authenticated(ctx.state, peer_id),
        })
    }
}
