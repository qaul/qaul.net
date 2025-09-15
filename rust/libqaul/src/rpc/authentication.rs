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
use state::InitCell;
use std::collections::BTreeMap;
use std::sync::RwLock;

/// Protobuf message definitions for authentication RPC
pub mod proto {
    include!("qaul.rpc.authentication.rs");
}

/// Active authentication challenge for a user
#[allow(dead_code)]
#[derive(Clone)]
pub struct AuthChallenge {
    pub nonce: u64,
    pub qaul_id: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
}

/// Global counter for generating unique nonces
/// Monotonically increasing to ensure each challenge has a unique identifier
static NONCE_COUNTER: InitCell<RwLock<u64>> = InitCell::new();
/// Map of active authentication challenges indexed by user ID
static ACTIVE_CHALLENGES: InitCell<RwLock<BTreeMap<Vec<u8>, AuthChallenge>>> = InitCell::new();
/// Map of authenticated users with their session expiration times
static AUTHENTICATED_USERS: InitCell<RwLock<BTreeMap<Vec<u8>, u64>>> = InitCell::new();

pub struct Authentication {}

#[allow(dead_code)]
impl Authentication {
    /// Initialize the authentication system
    /// Sets up the global state for nonce generation, challenge tracking,
    /// and authenticated user sessions.
    pub fn init() {
        NONCE_COUNTER.set(RwLock::new(1));
        ACTIVE_CHALLENGES.set(RwLock::new(BTreeMap::new()));
        AUTHENTICATED_USERS.set(RwLock::new(BTreeMap::new()));
    }

    /// Generate the next unique nonce
    fn next_nonce() -> u64 {
        let mut counter = NONCE_COUNTER.get().write().unwrap();
        let nonce = *counter;
        *counter += 1;
        nonce
    }

    /// Create an authentication challenge for a user
    pub fn create_challenge(qaul_id: PeerId) -> Result<u64, String> {
        println!(
            "LIBQAUL: Creating challenge for qaul_id: {:?}",
            qaul_id.to_bytes()
        );
        // Verify user exists in the system
        if UserAccounts::get_by_id(qaul_id).is_none() {
            return Err("User not found".to_string());
        }

        let nonce = Self::next_nonce();

        let now = Timestamp::get_timestamp();
        let qaul_id_bytes = qaul_id.to_bytes();

        // could also consider having the qaul_id as Vec<u8> in the args, but this is better
        let challenge = AuthChallenge {
            nonce,
            qaul_id: qaul_id_bytes.clone(),
            created_at: now,
            expires_at: now + 600 // Change to never expired(as discussed)
        };

        // Store challenge and cleanup any expired ones
        let mut challenges = ACTIVE_CHALLENGES.get().write().unwrap();
        // Debug: print what we're storing
        log::info!("Storing challenge with key: {:?}", qaul_id_bytes);

        challenges.insert(qaul_id_bytes, challenge); // Use same key format
        Self::cleanup_expired_challenge(&mut challenges, now);

        Ok(nonce)
    }

    /// Verify a challenge response from the client
    // Validates that:
    // 1. An active challenge exists for the user
    // 2. The challenge hasn't expired
    // 3. The response correctly incorporates the password hash and nonce
    pub fn verify_challenge(qaul_id: PeerId, challenge_hash: Vec<u8>) -> Result<bool, String> {
        let now = Timestamp::get_timestamp();
        let qaul_id_bytes = qaul_id.to_bytes();

        log::info!("Verifying challenge at timestamp: {}", now);
        log::info!("Looking for challenge with key: {:?}", qaul_id_bytes);

        // Get and print all active challenges
        let challenge = {
            let challenges = ACTIVE_CHALLENGES.get().read().unwrap();
            // Self::cleanup_expired_challenge(&mut challenges, now);
            log::info!("Total active challenges: {}", challenges.len());

            // Debug: print all keys in the map
            for (key, challenge) in challenges.iter() {
                log::info!(
                    "Challenge key: {:?}, expires_at: {}, current_time: {}",
                    key,
                    challenge.expires_at,
                    now
                );
            }

            // Clone the challenge to avoid holding the lock
            challenges.get(&qaul_id_bytes).cloned()
        };

        let challenge = challenge.ok_or("No active challenge or challenge expired".to_string())?;
        if now > challenge.expires_at {
            // Remove expired challenge
            let mut challenges = ACTIVE_CHALLENGES.get().write().unwrap();
            challenges.remove(&qaul_id.to_bytes());
            return Err("Challenge is expired".to_string());
        }

        let user = UserAccounts::get_by_id(qaul_id).ok_or("User not found".to_string())?;

        let stored_hash = match user.password_hash {
            Some(hash) => hash,
            None => {
                // Remove challenge after successful auth
                let mut challenges = ACTIVE_CHALLENGES.get().write().unwrap();
                challenges.remove(&qaul_id.to_bytes());

                Self::mark_authenticated(qaul_id);
                return Ok(true);
            }
        };

        let nonce_str = challenge.nonce.to_string();
        let combined = format!("{}{}", stored_hash, nonce_str);

        let challenge_hash_str = String::from_utf8(challenge_hash)
            .map_err(|_| "Invalid challenge hash encoding".to_string())?;

        // Parse the received hash
        let received_hash = PasswordHash::new(&challenge_hash_str)
            .map_err(|e| format!("Invalid password hash format: {}", e))?;

        let argon2 = Argon2::default();
        match argon2.verify_password(combined.as_bytes(), &received_hash) {
            Ok(()) => {
                // Remove challenge after successful verification
                let mut challenges = ACTIVE_CHALLENGES.get().write().unwrap();
                challenges.remove(&qaul_id.to_bytes());
                Self::mark_authenticated(qaul_id);
                Ok(true)
            }
            Err(_) => {
                // Keep challenge for retry
                Ok(false)
            }
        }
    }

    /// Mark a user as authenticated with a session
    fn mark_authenticated(qaul_id: PeerId) {
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();
        let expires_at = Timestamp::get_timestamp() + 86400; // Qs. is 1 hr enough?
        authenticated.insert(qaul_id.to_bytes(), expires_at);
    }

    /// Check if a user has an active authenticated session
    /// Also performs cleanup of expired sessions
    pub fn is_authenticated(qaul_id: PeerId) -> bool {
        let now = Timestamp::get_timestamp();
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();

        // probably we should again try to cleanup expired ssns
        authenticated.retain(|_, &mut expires_at| now < expires_at);
        authenticated.contains_key(&qaul_id.to_bytes())
    }

    /// Logout a user by removing their authenticated session
    pub fn logout(qaul_id: PeerId) {
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();
        authenticated.remove(&qaul_id.to_bytes());
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
    pub fn rpc(data: Vec<u8>, user_id: Vec<u8>) {
        match proto::AuthRpc::decode(&data[..]) {
            Ok(auth_rpc) => match auth_rpc.message {
                Some(proto::auth_rpc::Message::UsersRequest(_)) => {
                    Self::handle_users_request();
                }
                Some(proto::auth_rpc::Message::AuthRequest(auth_request)) => {
                    match PeerId::from_bytes(&auth_request.qaul_id) {
                        Ok(peer_id) => match Self::create_challenge(peer_id) {
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
                                    buf,
                                    crate::rpc::proto::Modules::Auth.into(),
                                    "".to_string(),
                                    Vec::new(),
                                );
                            }
                            Err(e) => {
                                Self::send_auth_result(false, e);
                            }
                        },
                        Err(_) => {
                            Self::send_auth_result(false, "Invalid qaul ID".to_string());
                        }
                    }
                }
                Some(proto::auth_rpc::Message::AuthResponse(auth_response)) => {
                    log::info!("Received auth response for user_id: {:?}", user_id);
                    match PeerId::from_bytes(&user_id) {
                        Ok(peer_id) => {
                            log::info!("Converted to PeerId: {:?}", peer_id.to_bytes());
                            match Self::verify_challenge(peer_id, auth_response.challenge_hash) {
                                Ok(success) => {
                                    if success {
                                        Self::send_auth_result(
                                            true,
                                            "Authentication successful".to_string(),
                                        );
                                    } else {
                                        Self::send_auth_result(
                                            false,
                                            "Invalid credentials".to_string(),
                                        );
                                    }
                                }
                                Err(e) => {
                                    Self::send_auth_result(false, e);
                                }
                            }
                        }
                        Err(_) => {
                            Self::send_auth_result(false, "Invalid user ID".to_string());
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
    fn send_auth_result(success: bool, message: String) {
        let result = proto::AuthRpc {
            message: Some(proto::auth_rpc::Message::AuthResult(proto::AuthResult {
                success,
                error_message: message,
            })),
        };

        let mut buf = Vec::with_capacity(result.encoded_len());
        result.encode(&mut buf).unwrap();

        Rpc::send_message(
            buf,
            crate::rpc::proto::Modules::Auth.into(),
            "".to_string(),
            Vec::new(),
        )
    }

    /// Handle request for list of users
    fn handle_users_request() {
        let config = crate::storage::configuration::Configuration::get();

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
            buf,
            crate::rpc::proto::Modules::Auth.into(),
            "".to_string(),
            Vec::new(),
        );
    }
}