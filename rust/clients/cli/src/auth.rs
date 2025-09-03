use super::rpc::Rpc;
use super::user_accounts::UserAccounts;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
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
    /// Get config.yaml path
    fn get_config_path() -> PathBuf {
        PathBuf::from("config.yaml")
    }

    /// Read user's salt from config.yaml
    fn get_user_salt(username: &str) -> Option<String> {
        let config_path = Self::get_config_path();

        if !config_path.exists() {
            log::error!("config.yaml not found");
            return None;
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => {
                match serde_yaml::from_str::<Config>(&content) {
                    Ok(config) => {
                        // find the user by name
                        config.user_accounts
                            .iter()
                            .find(|u| u.name == username)
                            .and_then(|u| u.password_salt.clone())
                    }
                    Err(e) => {
                        log::error!("failed to parse config.yaml: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                log::error!("failed to read config.yaml: {}", e);
                None
            }
        }
    }

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

    /// Initiate login process - request challenge
    pub fn initiate_login(username: String) {
        // get salt from config.yaml
        if let Some(salt) = Self::get_user_salt(&username) {
            // store salt for later use
            println!("DEBUG: Found salt for user: {}", username);

            UserAccounts::set_pending_auth_salt(salt);

            // get user ID and request challenge
            if let Some(user_id) = UserAccounts::get_user_id() {
                println!("DEBUG: Creating AuthRequest for user_id: {:?}", user_id);

                let auth_request = proto::AuthRequest {
                    qaul_id: user_id,
                };

                let proto_message = proto::AuthRpc {
                    message: Some(proto::auth_rpc::Message::AuthRequest(auth_request)),
                };

                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message.encode(&mut buf).unwrap();

                println!("DEBUG: Sending AuthRequest - {} bytes to module {}",
                         buf.len(), super::rpc::proto::Modules::Auth as i32);


                Rpc::send_message(
                    buf,
                    super::rpc::proto::Modules::Auth.into(),
                    "".to_string(),
                );

                println!("Requesting authentication challenge...");
            } else {
                println!("User account not found");
                UserAccounts::clear_pending_auth();
            }
        } else {
            println!("User '{}' not found or has no password set", username);
            UserAccounts::clear_pending_auth();
        }
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
            Ok(auth_rpc) => {
                match auth_rpc.message {
                    Some(proto::auth_rpc::Message::AuthChallenge(challenge)) => {
                        Self::handle_auth_challenge(challenge);
                    }
                    Some(proto::auth_rpc::Message::AuthResult(result)) => {
                        Self::handle_auth_result(result);
                    }
                    _ => {
                        log::error!("Unexpected auth RPC message");
                    }
                }
            }
            Err(error) => {
                log::error!("Failed to decode auth RPC: {:?}", error);
            }
        }
    }

    /// Handle authentication challenge
    fn handle_auth_challenge(challenge: proto::AuthChallenge) {
        if let Some(pending) = UserAccounts::get_pending_auth() {
            if let Some(salt_str) = pending.salt {
                let argon2 = Argon2::default();

                // parse the stored salt
                match SaltString::from_b64(&salt_str) {
                    Ok(salt) => {
                        // compute hash1 with the original salt
                        match argon2.hash_password(pending.password.as_bytes(), &salt) {
                            Ok(hash1) => {
                                // combine hash1 + nonce
                                let nonce_str = challenge.nonce.to_string();
                                let combined = format!("{}{}", hash1.to_string(), nonce_str);

                                // Generate new salt for hash2
                                let salt2 = SaltString::generate(&mut rand::thread_rng());

                                // Compute hash2
                                match argon2.hash_password(combined.as_bytes(), &salt2) {
                                    Ok(hash2) => {
                                        let response = proto::AuthResponse {
                                            challenge_hash: hash2.to_string().into_bytes(),
                                        };

                                        let proto_message = proto::AuthRpc {
                                            message: Some(proto::auth_rpc::Message::AuthResponse(response)),
                                        };

                                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                                        proto_message.encode(&mut buf).unwrap();

                                        Rpc::send_message(
                                            buf,
                                            super::rpc::proto::Modules::Auth.into(),
                                            "".to_string(),
                                        );

                                        println!("Authentication response sent");
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
                        }
                    }
                    Err(e) => {
                        println!("Invalid salt format: {}", e);
                        UserAccounts::clear_pending_auth();
                    }
                }
            } else {
                println!("No salt available");
                UserAccounts::clear_pending_auth();
            }
        } else {
            log::error!("No pending authentication");
        }
    }

    /// Handle authentication result
    fn handle_auth_result(result: proto::AuthResult) {
        if result.success {
            println!("Authentication successful!");

            if let Some(pending) = UserAccounts::get_pending_auth() {
                if let Some(user_id) = UserAccounts::get_user_id() {
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