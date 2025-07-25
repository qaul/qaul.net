use crate::node::user_accounts::UserAccounts;
use crate::utilities::timestamp::Timestamp;
use libp2p::PeerId;
use state::InitCell;
use std::collections::BTreeMap;
use std::fmt::format;
use std::sync::RwLock;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use toml::to_string;
use crate::utilities::qaul_id::QaulId;
// pub mod proto {
//     include!("qau.rpc.auth.rs");
// }

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

    pub fn verify_challenge(qaul_id: PeerId, challenge_hash: Vec<u8>) -> Result<bool, String>{
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

    pub fn is_autheticated(qaul_id: PeerId) -> bool {
        let now = Timestamp::get_timestamp();
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();

        // probably we should again try to cleanup expired ssns
        authenticated.retain(|_, &mut expires_at| now < expires_at);
        authenticated.contains_key(&qaul_id.to_bytes())
    }

    pub fn logout(qaul_id: PeerId)  {
        let mut authenticated = AUTHENTICATED_USERS.get().write().unwrap();
        authenticated.remove(&qaul_id.to_bytes());
    }

    fn cleanup_expired_challenge(
        challenges: &mut BTreeMap<Vec<u8>, AuthChallenge>, now: u64,
    ) {
        challenges.retain(|_, challenge| now < challenge.expires_at);
    }

    pub fn rpc() {}

    fn handle_auth_request() {}

    fn handle_auth_response() {}

    fn send_auth_result() {}
}