use std::collections::BTreeMap;
use std::sync::RwLock;
use config::FileFormat::Ini;
use libp2p::PeerId;
use state::InitCell;
use crate::node::user_accounts::{UserAccount, UserAccounts};
use crate::utilities::timestamp::Timestamp;

pub mod proto {
    include!("qau.rpc.auth.rs");
}

#[derive(Clone)]
pub struct AuthChallenge {
    pub nonce : u64;
    pub qaul_id: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
}

static NONCE_COUNTER : InitCell<RwLock<u64>> = InitCell::new();
static ACTIVE_CHALLENGES: InitCell<RwLock<BTreeMap<Vec<u8>, AuthChallenge>>> = InitCell::new();
static AUTHENTICATED_USERS : InitCell<RwLock<BTreeMap<Vec<u8>, u64>>> = InitCell::new();

pub struct Authentication{}

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

    pub fn create_challenge(qaul_id: Vec<u8>) -> Result<u64, String> {
        let peer_id = PeerId::from_bytes(&qaul_id)
            .map_err(|_| "Invalid qaulId".to_string())?;

        if UserAccounts::get_by_id(peer_id).is_none() {
            return Err("User not found".to_string());
        }

        let nonce = Self::next_nonce();

        let now = Timestamp::get_timestamp();
        let challenge = AuthChallenge {
            nonce,
            qaul_id: qaul_id.clone(),
            created_at: now,
            expires_at: now + 300, // I need to confirm this
        };

        let mut challenges = ACTIVE_CHALLENGES.get().write().unwrap();
        challenges.insert(qaul_id, challenge);

        Self::cleanup_expired_challenge(&mut challenges, now);

        Ok(nonce)
    }

    pub fn verify_challenge() {}

    fn mark_authenticated() {}

    pub fn is_autheticated() {}

    pub fn logout() {}

    fn cleanup_expired_challenge(
        challenges: &mut BTreeMap<Vec<u8>, AuthChallenge>, now: u64
    ) {
        challenges.retain(|_, challenge| now < challenge.expires_at);
    }

    pub fn rpc() {}

    fn handle_auth_request() {}

    fn handle_auth_response() {}

    fn send_auth_result() {}
}