// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Authentication Module
//!
//! This module handles user authentication for qaul CLI applications.
//! It implements a secure challenge-response authentication mechanism using
//! argon2 password hashing and manages persistent session tokens.

use super::rpc::Rpc;
use super::user_accounts::UserAccounts;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use libqaul::storage::configuration::Configuration;
use prost::Message;
use serde::{Deserialize, Serialize};

/// protobuf message definitions for authentication RPC
pub mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.authentication.rs");
}

/// Session information persisted to filesystem
///
/// Sessions are stored in ~/.qaul_session with restricted permissions (0600 on Unix)
/// to prevent unauthorized access to session tokens.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionInfo {
    pub user_id: Vec<u8>,
    pub username: String,
    // Random session token for authentication
    pub session_token: String,
    pub created_at: u64,
}

/// Main authentication handler for the qaul CLI
pub struct Auth;

impl Auth {
    /// Generate a simple token using Argon2
    fn generate_token(user_id: &str, username: &str) -> String {
        use argon2::password_hash::rand_core::OsRng;

        let input = format!("{}:{}", user_id, username);
        let argon2 = Argon2::default();

        // Generate a salt once or use a fixed valid one
        let salt = SaltString::generate(&mut OsRng);

        match argon2.hash_password(input.as_bytes(), &salt) {
            Ok(hash) => {
                hash.to_string()
            },
            Err(_) => {
                bs58::encode(input.as_bytes()).into_string()
            }
        }
    }

    /// Save token to config
    fn save_token_to_config(user_id: String, token: String) {
        {
            let mut config = Configuration::get_mut();
            for user in &mut config.user_accounts {
                if user.id == user_id {
                    user.session_token = Some(token);
                    break;
                }
            }
        }
        Configuration::save();
    }

    /// Get session info
    /// Load token from config instead of file
    pub fn get_session_info() -> Option<SessionInfo> {
        let config = Configuration::get();

        // Find first user with a token
        for user in &config.user_accounts {
            if let Some(token) = &user.session_token {
                return Some(SessionInfo {
                    user_id: user.id.as_bytes().to_vec(),
                    username: user.name.clone(),
                    session_token: token.clone(),
                    created_at: 0,
                });
            }
        }
        None
    }

    /// Clear session - just remove token from config
    pub fn clear_session() {
        // Find and clear the active session token
        {
            let mut config = Configuration::get_mut();
            for user in &mut config.user_accounts {
                if user.session_token.is_some() {
                    user.session_token = None;
                    break;
                }
            }
        }
        Configuration::save();
        UserAccounts::set_session_token(None);
    }

    /// Restore session on startup - load from config
    pub fn restore_session() {
        let config = Configuration::get();

        // Find user with token and restore
        for user in &config.user_accounts {
            if let Some(token) = &user.session_token {
                log::info!("Restored session for user: {}", user.name);
                UserAccounts::set_session_token(Some(token.clone()));
                break; // Only one active session
            }
        }
    }

    /// Initiate the login process for a user
    /// Starts authentication by requesting the list of users from the node
    /// to validate the username and retrieve salt for password hashing.
    pub fn initiate_login(username: String) {
        println!("Requesting users list...");

        // store username for later use in authentication flow
        UserAccounts::set_pending_username(username);

        let users_request = proto::UsersRequest {};

        let proto_message = proto::AuthRpc {
            message: Some(proto::auth_rpc::Message::UsersRequest(users_request)),
        };
        // encode and send the request
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).unwrap();

        Rpc::send_message(buf, super::rpc::proto::Modules::Auth.into(), "".to_string());
    }

    /// Handle logout
    pub fn logout(_user_id: Vec<u8>) {
        Self::clear_session();
        println!("Logged out successfully");
    }

    /// Process incoming authentication RPC messages
    /// Routes different message types to appropriate handlers:
    /// - UsersResponse: Validates user and requests authentication
    /// - AuthChallenge: Computes challenge response
    /// - AuthResult: Handles authentication success/failure
    pub fn rpc(data: Vec<u8>) {
        match proto::AuthRpc::decode(&data[..]) {
            Ok(auth_rpc) => match auth_rpc.message {
                Some(proto::auth_rpc::Message::UsersResponse(response)) => {
                    Self::handle_users_response(response);
                }
                Some(proto::auth_rpc::Message::AuthChallenge(challenge)) => {
                    Self::handle_auth_challenge(challenge);
                }
                Some(proto::auth_rpc::Message::AuthResult(result)) => {
                    Self::handle_auth_result(result);
                }
                _ => {
                    log::error!("Unexpected auth RPC message");
                }
            },
            Err(error) => {
                log::error!("Failed to decode auth RPC: {:?}", error);
            }
        }
    }

    /// Handle users list response from node
    /// validates that the requested user exists and has a password set,
    /// then initiates the authentication challenge if valid.
    fn handle_users_response(response: proto::UsersResponse) {
        if let Some(pending_username) = UserAccounts::get_pending_username() {
            // find the user in the response
            if let Some(user_info) = response
                .users
                .iter()
                .find(|u| u.username == pending_username)
            {
                // check if user has password authentication enabled
                if user_info.has_password {
                    if !user_info.salt.clone().expect("REASON").is_empty() {
                        println!("Found user with password");
                        // store salt and user ID for challenge response
                        UserAccounts::set_pending_auth_salt(
                            user_info.salt.clone().expect("REASON"),
                        );
                        UserAccounts::set_pending_user_id(user_info.user_id.clone());
                        // request authentication challenge from node
                        let auth_request = proto::AuthRequest {
                            qaul_id: user_info.user_id.clone(),
                        };

                        let proto_message = proto::AuthRpc {
                            message: Some(proto::auth_rpc::Message::AuthRequest(auth_request)),
                        };

                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message.encode(&mut buf).unwrap();

                        Rpc::send_message(
                            buf,
                            super::rpc::proto::Modules::Auth.into(),
                            "".to_string(),
                        );

                        println!("Requesting authentication challenge...");
                    } else {
                        println!("User has password but no salt available");
                        UserAccounts::clear_pending_auth();
                    }
                } else {
                    println!("User has no password set, so authenticating without password");
                    // this would be improved as the token implementation is completed
                    let user_id_str = bs58::encode(&user_info.user_id).into_string();
                    let token = Self::generate_token(&user_id_str, &pending_username);
                    Self::save_token_to_config(user_id_str, token.clone());

                    UserAccounts::set_session_token(Some(token));
                    UserAccounts::clear_pending_auth();

                    println!("Authentication successful!");
                }
            } else {
                // user not found, show available users for debugging
                println!("User '{}' not found", pending_username);
                println!("Available users:");
                for user in &response.users {
                    println!("  - {}", user.username);
                }
                UserAccounts::clear_pending_auth();
            }
        }
    }

    /// Handle authentication challenge from server
    /// Computes the challenge response using a double Argon2 hash:
    /// 1. Hash the password with the user's salt
    /// 2. Combine with nonce and hash again with a new salt
    fn handle_auth_challenge(challenge: proto::AuthChallenge) {
        if let Some(pending) = UserAccounts::get_pending_auth() {
            if let Some(salt_str) = pending.salt {
                let argon2 = Argon2::default();
                // Parse the stored salt
                match SaltString::from_b64(&salt_str) {
                    Ok(salt) => match argon2.hash_password(pending.password.as_bytes(), &salt) {
                        Ok(hash1) => {
                            let nonce_str = challenge.nonce.to_string();
                            let combined = format!("{}{}", hash1.to_string(), nonce_str);
                            // Generate new salt for second hash
                            let salt2 = SaltString::generate(&mut OsRng);

                            match argon2.hash_password(combined.as_bytes(), &salt2) {
                                Ok(hash2) => {
                                    // Send challenge response to server
                                    let response = proto::AuthResponse {
                                        challenge_hash: hash2.to_string().into_bytes(),
                                    };

                                    let proto_message = proto::AuthRpc {
                                        message: Some(proto::auth_rpc::Message::AuthResponse(
                                            response,
                                        )),
                                    };

                                    let mut buf = Vec::with_capacity(proto_message.encoded_len());
                                    proto_message.encode(&mut buf).unwrap();

                                    Rpc::send_message(
                                        buf,
                                        super::rpc::proto::Modules::Auth.into(),
                                        "".to_string(),
                                    );

                                    println!("Sending authentication response...");
                                }
                                Err(e) => {
                                    println!("Failed to compute challenge response: {}", e);
                                    UserAccounts::clear_pending_auth();
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to compute password hash: {}", e);
                            UserAccounts::clear_pending_auth();
                        }
                    },
                    Err(e) => {
                        println!("Invalid salt format: {}", e);
                        UserAccounts::clear_pending_auth();
                    }
                }
            }
        } else {
            log::error!("No pending authentication");
        }
    }

    /// Handle authentication result from server
    /// On success, generates a random session token and persists the session.
    /// On failure, displays error message and clears pending authentication.
    fn handle_auth_result(result: proto::AuthResult) {
        if result.success {
            println!("Authentication successful!");

            if let Some(pending) = UserAccounts::get_pending_auth() {
                if let Some(user_id) = pending.user_id {
                    let user_id_str = bs58::encode(&user_id).into_string();
                    // Generate random session token
                    let token = Self::generate_token(&user_id_str, &pending.username);
                    Self::save_token_to_config(user_id_str, token.clone());

                    UserAccounts::set_session_token(Some(token));
                }
            }

            UserAccounts::clear_pending_auth();
        } else {
            println!("Authentication failed: {}", result.error_message);
            UserAccounts::clear_pending_auth();
        }
    }
}