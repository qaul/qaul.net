// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use super::rpc::Rpc;
use super::user_accounts::UserAccounts;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use prost::Message;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.auth.rs");
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    #[serde(default)]
    user_accounts: Vec<ConfigUserAccount>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ConfigUserAccount {
    name: String,
    id: String,
    password_hash: Option<String>,
    password_salt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionInfo {
    pub user_id: Vec<u8>,
    pub username: String,
    pub session_token: String,
    pub created_at: u64,
}

pub struct Auth;

impl Auth {
    /// Get session file path
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

    /// Initiate login process - first request salt
    pub fn initiate_login(username: String) {
        println!("Requesting users list...");

        UserAccounts::set_pending_username(username);

        let users_request = proto::UsersRequest {};

        let proto_message = proto::AuthRpc {
            message: Some(proto::auth_rpc::Message::UsersRequest(users_request)),
        };

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

    /// Process received RPC message
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

    fn handle_users_response(response: proto::UsersResponse) {
        if let Some(pending_username) = UserAccounts::get_pending_username() {
            if let Some(user_info) = response
                .users
                .iter()
                .find(|u| u.username == pending_username)
            {
                if user_info.has_password {
                    if !user_info.salt.clone().expect("REASON").is_empty() {
                        println!("Found user with password");

                        UserAccounts::set_pending_auth_salt(
                            user_info.salt.clone().expect("REASON"),
                        );
                        UserAccounts::set_pending_user_id(user_info.user_id.clone());

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
                println!("User '{}' not found", pending_username);
                println!("Available users:");
                for user in &response.users {
                    println!("  - {}", user.username);
                }
                UserAccounts::clear_pending_auth();
            }
        }
    }

    fn handle_auth_challenge(challenge: proto::AuthChallenge) {
        if let Some(pending) = UserAccounts::get_pending_auth() {
            if let Some(salt_str) = pending.salt {
                let argon2 = Argon2::default();

                match SaltString::from_b64(&salt_str) {
                    Ok(salt) => match argon2.hash_password(pending.password.as_bytes(), &salt) {
                        Ok(hash1) => {
                            let nonce_str = challenge.nonce.to_string();
                            let combined = format!("{}{}", hash1.to_string(), nonce_str);

                            let salt2 = SaltString::generate(&mut OsRng);

                            match argon2.hash_password(combined.as_bytes(), &salt2) {
                                Ok(hash2) => {
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

    fn handle_auth_result(result: proto::AuthResult) {
        if result.success {
            println!("Authentication successful!");

            if let Some(pending) = UserAccounts::get_pending_auth() {
                if let Some(user_id) = pending.user_id {
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