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

// NONCE COUNTER
// ACTIVE_CHALLENGES = TREE COULD BE USED
// AUTHENTICATED_USERS = TREE

pub struct Authentication{}

impl Authentication {

    pub fn init() {}

    fn next_nonce() {}

    pub fn create_challenge() {}

    pub fn verify_challenge() {}

    fn mark_authenticated() {}

    pub fn is_autheticated() {}

    pub fn logout() {}

    fn cleanup_expired_challenge(
        /*
        Should I reuse them after a while?
         */
    ) {}

    pub fn rpc() {}

    fn handle_auth_request() {}

    fn handle_auth_response() {}

    fn send_auth_result() {}
}