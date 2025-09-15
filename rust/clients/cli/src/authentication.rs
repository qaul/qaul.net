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
use prost::Message;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
    /// Get the platform-specific path for session file storage
    fn get_session_file_path() -> PathBuf {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".qaul_session")
        }

        #[cfg(target_os = "windows")]
        {
            let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".qaul_session")
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            PathBuf::from(".qaul_session")
        }
    }

    /// Save session to file
    pub fn save_session(session: SessionInfo) {
        let path = Self::get_session_file_path();
        let json = serde_json::to_string_pretty(&session).unwrap();

        if let Err(e) = fs::write(&path, json) {
            log::error!("Failed to save session: {}", e);
        } else {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = fs::metadata(&path) {
                    let mut perms = metadata.permissions();
                    perms.set_mode(0o600);
                    let _ = fs::set_permissions(&path, perms);
                }
            }
            println!("Session saved");
        }
    }

    /// Load session from file
    pub fn load_session() -> Option<SessionInfo> {
        let path = Self::get_session_file_path();

        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
        } else {
            None
        }
    }

    /// Get session info
    pub fn get_session_info() -> Option<SessionInfo> {
        Self::load_session()
    }

    /// Clear session
    pub fn clear_session() {
        let path = Self::get_session_file_path();
        let _ = fs::remove_file(path);
    }

    /// Restore session on startup
    pub fn restore_session() {
        if let Some(session) = Self::load_session() {
            log::info!("Restored session for user: {}", session.username);
            UserAccounts::set_session_token(Some(session.session_token));
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
        UserAccounts::set_session_token(None);
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
                    println!("User has no password set");
                    UserAccounts::clear_pending_auth();
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
                    // Generate random session token
                    let session_token = bs58::encode(rand::random::<[u8; 32]>()).into_string();

                    let session = SessionInfo {
                        user_id,
                        username: pending.username,
                        session_token: session_token.clone(),
                        created_at: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };
                    // Persist session and set as active
                    Self::save_session(session);
                    UserAccounts::set_session_token(Some(session_token));
                }
            }

            UserAccounts::clear_pending_auth();
        } else {
            println!("Authentication failed: {}", result.error_message);
            UserAccounts::clear_pending_auth();
        }
    }
}