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
use prost::Message;
use std::collections::BTreeMap;
use std::sync::RwLock;

/// Protobuf message definitions for authentication RPC
pub use qaul_proto::qaul_rpc_authentication as proto;

/// Active authentication challenge for a user
#[allow(dead_code)]
#[derive(Clone)]
pub struct AuthChallenge {
    pub nonce: u64,
    pub qaul_id: Vec<u8>,
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

    /// Generate the next unique nonce (instance method).
    pub fn next_nonce(&self) -> u64 {
        next_nonce_impl(&self.nonce_counter)
    }

    /// Create an authentication challenge for a user (instance method).
    /// Takes user_accounts as an explicit parameter.
    pub fn create_challenge(
        &self,
        qaul_id: PeerId,
        user_accounts: &crate::node::user_accounts::UserAccountsState,
    ) -> Result<u64, String> {
        if user_accounts.get_by_id(qaul_id).is_none() {
            return Err("User not found".to_string());
        }

        let nonce = create_challenge_impl(
            &self.nonce_counter,
            &self.active_challenges,
            qaul_id,
        );
        Ok(nonce)
    }

    /// Verify a challenge response (instance method).
    /// Takes user_accounts as an explicit parameter.
    pub fn verify_challenge(
        &self,
        qaul_id: PeerId,
        challenge_hash: Vec<u8>,
        user_accounts: &crate::node::user_accounts::UserAccountsState,
    ) -> Result<bool, String> {
        let user = user_accounts
            .get_by_id(qaul_id)
            .ok_or("User not found".to_string())?;

        verify_challenge_impl(
            &self.active_challenges,
            &self.authenticated_users,
            qaul_id,
            challenge_hash,
            user.password_hash,
        )
    }

    /// Mark a user as authenticated (instance method).
    pub fn mark_authenticated(&self, qaul_id: PeerId) {
        mark_authenticated_impl(&self.authenticated_users, qaul_id);
    }

    /// Check if a user is authenticated (instance method).
    pub fn is_authenticated(&self, qaul_id: PeerId) -> bool {
        is_authenticated_impl(&self.authenticated_users, qaul_id)
    }

    /// Logout a user (instance method).
    pub fn logout(&self, qaul_id: PeerId) {
        logout_impl(&self.authenticated_users, qaul_id);
    }

}

/// Generate the next unique nonce from a given counter lock.
fn next_nonce_impl(counter: &RwLock<u64>) -> u64 {
    let mut c = counter.write().unwrap();
    let nonce = *c;
    *c += 1;
    nonce
}

/// Mark a user as authenticated in the given authenticated-users map.
fn mark_authenticated_impl(authenticated: &RwLock<BTreeMap<Vec<u8>, u64>>, qaul_id: PeerId) {
    let mut auth = authenticated.write().unwrap();
    let expires_at = Timestamp::get_timestamp() + (86400 * 365 * 100);
    auth.insert(qaul_id.to_bytes(), expires_at);
}

/// Check if a user is authenticated in the given authenticated-users map.
/// Also performs cleanup of expired sessions.
fn is_authenticated_impl(authenticated: &RwLock<BTreeMap<Vec<u8>, u64>>, qaul_id: PeerId) -> bool {
    let now = Timestamp::get_timestamp();
    let mut auth = authenticated.write().unwrap();
    auth.retain(|_, &mut expires_at| now < expires_at);
    auth.contains_key(&qaul_id.to_bytes())
}

/// Logout a user by removing their session from the given authenticated-users map.
fn logout_impl(authenticated: &RwLock<BTreeMap<Vec<u8>, u64>>, qaul_id: PeerId) {
    let mut auth = authenticated.write().unwrap();
    auth.remove(&qaul_id.to_bytes());
}

/// Create an authentication challenge for a user.
/// The caller is responsible for verifying that the user exists before calling this.
fn create_challenge_impl(
    nonce_counter: &RwLock<u64>,
    active_challenges: &RwLock<BTreeMap<Vec<u8>, AuthChallenge>>,
    qaul_id: PeerId,
) -> u64 {
    let nonce = next_nonce_impl(nonce_counter);
    let now = Timestamp::get_timestamp();
    let qaul_id_bytes = qaul_id.to_bytes();

    let challenge = AuthChallenge {
        nonce,
        qaul_id: qaul_id_bytes.clone(),
        created_at: now,
        expires_at: now + 9999999999,
    };

    let mut challenges = active_challenges.write().unwrap();
    challenges.insert(qaul_id_bytes, challenge);
    challenges.retain(|_, c| now < c.expires_at);

    nonce
}

/// Verify a challenge response.
/// `stored_hash` is the user's password hash (None means no password set).
/// Returns Ok(true) on success, Ok(false) on wrong credentials, Err on missing challenge / format error.
fn verify_challenge_impl(
    active_challenges: &RwLock<BTreeMap<Vec<u8>, AuthChallenge>>,
    authenticated_users: &RwLock<BTreeMap<Vec<u8>, u64>>,
    qaul_id: PeerId,
    challenge_hash: Vec<u8>,
    stored_hash: Option<String>,
) -> Result<bool, String> {
    let qaul_id_bytes = qaul_id.to_bytes();

    let challenge = {
        let challenges = active_challenges.read().unwrap();
        challenges.get(&qaul_id_bytes).cloned()
    };

    let challenge = challenge.ok_or("No active challenge or challenge expired".to_string())?;

    let stored_hash = match stored_hash {
        Some(hash) => hash,
        None => {
            // No password set - remove challenge and authenticate
            let mut challenges = active_challenges.write().unwrap();
            challenges.remove(&qaul_id_bytes);
            mark_authenticated_impl(authenticated_users, qaul_id);
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
            mark_authenticated_impl(authenticated_users, qaul_id);
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

pub struct Authentication {}

#[allow(dead_code)]
impl Authentication {
    /// Initialize the authentication system.
    /// State is now owned by QaulState, so this is a no-op.
    pub fn init() {
        // State is created by QaulState::new(); nothing to do here.
    }

    /// Generate the next unique nonce
    fn next_nonce(state: &crate::QaulState) -> u64 {
        next_nonce_impl(&state.auth.nonce_counter)
    }

    /// Create an authentication challenge for a user
    pub fn create_challenge(state: &crate::QaulState, qaul_id: PeerId) -> Result<u64, String> {
        println!(
            "LIBQAUL: Creating challenge for qaul_id: {:?}",
            qaul_id.to_bytes()
        );
        // Verify user exists in the system
        if UserAccounts::get_by_id(state, qaul_id).is_none() {
            return Err("User not found".to_string());
        }

        log::info!("Storing challenge with key: {:?}", qaul_id.to_bytes());

        let auth = &state.auth;
        let nonce = create_challenge_impl(
            &auth.nonce_counter,
            &auth.active_challenges,
            qaul_id,
        );
        Ok(nonce)
    }

    /// Verify a challenge response from the client
    // Validates that:
    // 1. An active challenge exists for the user
    // 2. The challenge hasn't expired
    // 3. The response correctly incorporates the password hash and nonce
    pub fn verify_challenge(state: &crate::QaulState, qaul_id: PeerId, challenge_hash: Vec<u8>) -> Result<bool, String> {
        let now = Timestamp::get_timestamp();
        let qaul_id_bytes = qaul_id.to_bytes();

        log::info!("Verifying challenge at timestamp: {}", now);
        log::info!("Looking for challenge with key: {:?}", qaul_id_bytes);

        let auth = &state.auth;

        // Debug: print all active challenges
        {
            let challenges = auth.active_challenges.read().unwrap();
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

        let user = UserAccounts::get_by_id(state, qaul_id).ok_or("User not found".to_string())?;

        verify_challenge_impl(
            &auth.active_challenges,
            &auth.authenticated_users,
            qaul_id,
            challenge_hash,
            user.password_hash,
        )
    }

    /// Mark a user as authenticated with a session
    fn mark_authenticated(qaul_id: PeerId) {
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();
        // Session expires after 24 hours (86400 seconds * 1000 milliseconds)
        let expires_at = Timestamp::get_timestamp() + (86400 * 1000);
        authenticated.insert(qaul_id.to_bytes(), expires_at);
    }

    /// Check if a user has an active authenticated session
    /// Also performs cleanup of expired sessions
    pub fn is_authenticated(state: &crate::QaulState, qaul_id: PeerId) -> bool {
        is_authenticated_impl(&state.auth.authenticated_users, qaul_id)
    }

    /// Logout a user by removing their authenticated session
    pub fn logout(state: &crate::QaulState, qaul_id: PeerId) {
        logout_impl(&state.auth.authenticated_users, qaul_id);
    }

    /// Remove expired challenges from the active challenges map
    fn cleanup_expired_challenge(challenges: &mut BTreeMap<Vec<u8>, AuthChallenge>, now: u64) {
        challenges.retain(|_, challenge| now < challenge.expires_at);
    }

    /// Process incoming authentication RPC messages
    // Routes messages to appropriate handlers based on message type:
    // - UsersRequest: Send list of available users
    // - AuthRequest: Create and send authentication challenge
    // - AuthResponse: Verify challenge response and authenticate
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
        match proto::AuthRpc::decode(&data[..]) {
            Ok(auth_rpc) => match auth_rpc.message {
                Some(proto::auth_rpc::Message::UsersRequest(_)) => {
                    Self::handle_users_request(state, request_id);
                }
                Some(proto::auth_rpc::Message::AuthRequest(auth_request)) => {
                    match PeerId::from_bytes(&auth_request.qaul_id) {
                        Ok(peer_id) => match Self::create_challenge(state, peer_id) {
                            Ok(nonce) => {
                                let challenge = proto::AuthRpc {
                                    message: Some(proto::auth_rpc::Message::AuthChallenge(
                                        proto::AuthChallenge {
                                            nonce,
                                            expires_at: Timestamp::get_timestamp() + 300,
                                        },
                                    )),
                                };

                                let mut buf = Vec::with_capacity(challenge.encoded_len());
                                challenge.encode(&mut buf).unwrap();

                                Rpc::send_message(
                                    state,
                                    buf,
                                    crate::rpc::proto::Modules::Auth.into(),
                                    request_id,
                                    Vec::new(),
                                );
                            }
                            Err(e) => {
                                Self::send_auth_result(state, false, e, request_id);
                            }
                        },
                        Err(_) => {
                            Self::send_auth_result(
                                state,
                                false,
                                "Invalid qaul ID".to_string(),
                                request_id,
                            );
                        }
                    }
                }
                Some(proto::auth_rpc::Message::AuthResponse(auth_response)) => {
                    log::info!("Received auth response for user_id: {:?}", user_id);
                    match PeerId::from_bytes(&user_id) {
                        Ok(peer_id) => {
                            log::info!("Converted to PeerId: {:?}", peer_id.to_bytes());
                            match Self::verify_challenge(state, peer_id, auth_response.challenge_hash) {
                                Ok(success) => {
                                    if success {
                                        Self::send_auth_result(
                                            state,
                                            true,
                                            "Authentication successful".to_string(),
                                            request_id,
                                        );
                                    } else {
                                        Self::send_auth_result(
                                            state,
                                            false,
                                            "Invalid credentials".to_string(),
                                            request_id,
                                        );
                                    }
                                }
                                Err(e) => {
                                    Self::send_auth_result(state, false, e, request_id);
                                }
                            }
                        }
                        Err(_) => {
                            Self::send_auth_result(
                                state,
                                false,
                                "Invalid user ID".to_string(),
                                request_id,
                            );
                        }
                    }
                }
                _ => {
                    log::error!("Unsupported auth RPC message");
                }
            },
            Err(e) => {
                log::error!("Failed to decode: {:?}", e);
            }
        }
    }

    /// Send authentication result to the client
    fn send_auth_result(state: &crate::QaulState, success: bool, message: String, request_id: String) {
        let result = proto::AuthRpc {
            message: Some(proto::auth_rpc::Message::AuthResult(proto::AuthResult {
                success,
                error_message: message,
            })),
        };

        let mut buf = Vec::with_capacity(result.encoded_len());
        result.encode(&mut buf).unwrap();

        Rpc::send_message(
            state,
            buf,
            crate::rpc::proto::Modules::Auth.into(),
            request_id,
            Vec::new(),
        )
    }

    /// Handle request for list of users
    fn handle_users_request(state: &crate::QaulState, request_id: String) {
        let config = crate::storage::configuration::Configuration::get(state);

        let mut users_list = Vec::new();

        // Build list of users from configuration
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

        let response = proto::UsersResponse {
            users: users_list,
            error_message: String::new(),
        };

        let rpc_message = proto::AuthRpc {
            message: Some(proto::auth_rpc::Message::UsersResponse(response)),
        };

        let mut buf = Vec::with_capacity(rpc_message.encoded_len());
        rpc_message.encode(&mut buf).unwrap();

        Rpc::send_message(
            state,
            buf,
            crate::rpc::proto::Modules::Auth.into(),
            request_id,
            Vec::new(),
        );
    }
}
