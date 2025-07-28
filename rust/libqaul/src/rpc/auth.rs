use crate::node::user_accounts::UserAccounts;
use crate::rpc::Rpc;
use crate::utilities::timestamp::Timestamp;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use libp2p::PeerId;
use prost::Message;
use state::InitCell;
use std::collections::BTreeMap;
use std::sync::RwLock;

pub mod proto {
    include!("qaul.rpc.auth.rs");
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct AuthChallenge {
    pub nonce: u64,
    pub qaul_id: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
}

static NONCE_COUNTER: InitCell<RwLock<u64>> = InitCell::new();
static ACTIVE_CHALLENGES: InitCell<RwLock<BTreeMap<Vec<u8>, AuthChallenge>>> = InitCell::new();
static AUTHENTICATED_USERS: InitCell<RwLock<BTreeMap<Vec<u8>, u64>>> = InitCell::new();

pub struct Authentication {}

#[allow(dead_code)]
impl Authentication {
    pub fn init() {
        NONCE_COUNTER.set(RwLock::new(1));
        ACTIVE_CHALLENGES.set(RwLock::new(BTreeMap::new()));
        AUTHENTICATED_USERS.set(RwLock::new(BTreeMap::new()));
    }

    fn next_nonce() -> u64 {
        let mut counter = NONCE_COUNTER.get().write().unwrap();
        let nonce = *counter;
        *counter += 1;
        nonce
    }

    pub fn create_challenge(qaul_id: PeerId) -> Result<u64, String> {
        if UserAccounts::get_by_id(qaul_id).is_none() {
            return Err("User not found".to_string());
        }

        let nonce = Self::next_nonce();

        let now = Timestamp::get_timestamp();

        // could also consider having the qaul_id as Vec<u8> in the args, but this is better
        let challenge = AuthChallenge {
            nonce,
            qaul_id: qaul_id.to_bytes(),
            created_at: now,
            expires_at: now + 300, // I need to confirm this
        };

        let mut challenges = ACTIVE_CHALLENGES.get().write().unwrap();
        challenges.insert(qaul_id.to_bytes(), challenge);

        Self::cleanup_expired_challenge(&mut challenges, now);

        Ok(nonce)
    }

    pub fn verify_challenge(qaul_id: PeerId, challenge_hash: Vec<u8>) -> Result<bool, String> {
        let now = Timestamp::get_timestamp();

        let challenge = {
            let mut challenges = ACTIVE_CHALLENGES.get().write().unwrap();
            Self::cleanup_expired_challenge(&mut challenges, now);
            challenges.remove(&qaul_id.to_bytes())
        };

        let challenge = challenge.ok_or("No active challenge or challenge expired".to_string())?;

        if now > challenge.expires_at {
            return Err("Challenge is expired".to_string());
        }

        let user = UserAccounts::get_by_id(qaul_id).ok_or("User not found".to_string())?;

        let stored_hash = match user.password_hash {
            Some(hash) => hash,
            None => {
                Self::mark_authenticated(qaul_id);
                return Ok(true);
            }
        };

        let nonce_str = challenge.nonce.to_string();
        let combined = format!("{}{}", stored_hash, nonce_str);

        let challenge_hash_str = String::from_utf8(challenge_hash).unwrap();

        let received_hash = PasswordHash::new(&challenge_hash_str).unwrap();

        let argon2 = Argon2::default();
        match argon2.verify_password(combined.as_bytes(), &received_hash) {
            Ok(()) => {
                Self::mark_authenticated(qaul_id);
                Ok(true)
            }
            Err(_) => Ok(false)
        }
    }

    fn mark_authenticated(qaul_id: PeerId) {
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();
        let expires_at = Timestamp::get_timestamp() + 3600; // Qs. is 1 hr enough?
        authenticated.insert(qaul_id.to_bytes(), expires_at);
    }

    pub fn is_authenticated(qaul_id: PeerId) -> bool {
        let now = Timestamp::get_timestamp();
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();

        // probably we should again try to cleanup expired ssns
        authenticated.retain(|_, &mut expires_at| now < expires_at);
        authenticated.contains_key(&qaul_id.to_bytes())
    }

    pub fn logout(qaul_id: PeerId) {
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();
        authenticated.remove(&qaul_id.to_bytes());
    }

    fn cleanup_expired_challenge(
        challenges: &mut BTreeMap<Vec<u8>, AuthChallenge>, now: u64,
    ) {
        challenges.retain(|_, challenge| now < challenge.expires_at);
    }

    pub fn rpc(data: Vec<u8>, user_id: Vec<u8>) {
        match proto::AuthRpc::decode(&data[..]) {
            Ok(auth_rpc) => {
                match auth_rpc.message {
                    Some(proto::auth_rpc::Message::AuthRequest(auth_request)) => {
                        match PeerId::from_bytes(&auth_request.qaul_id) {
                            Ok(peer_id) => {
                                match Self::create_challenge(peer_id) {
                                    Ok(nonce) => {
                                        let challenge = proto::AuthRpc {
                                            message: Some(proto::auth_rpc::Message::AuthChallenge(
                                                proto::AuthChallenge {
                                                    nonce,
                                                    expires_at: Timestamp::get_timestamp() + 300,
                                                }
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
                                }
                            }
                            Err(_) => {
                                Self::send_auth_result(false, "Invalid qaul ID".to_string());
                            }
                        }
                    }
                    Some(proto::auth_rpc::Message::AuthResponse(auth_response)) => {
                        match PeerId::from_bytes(&user_id) {
                            Ok(peer_id) => {
                                match Self::verify_challenge(
                                    peer_id, auth_response.challenge_hash,
                                ) {
                                    Ok(success) => {
                                        if success {
                                            Self::send_auth_result(true, "Authentication successful".to_string());
                                        } else {
                                            Self::send_auth_result(false, "Invalid credentials".to_string());
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
                }
            }
            Err(e) => {
                log::error!("Failed to decode: {:}", e);
            }
        }
    }

    fn send_auth_result(success: bool, message: String) {
        let result = proto::AuthRpc {
            message: Some(proto::auth_rpc::Message::AuthResult(
                proto::AuthResult {
                    success,
                    error_message: message,
                }
            )),
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
}