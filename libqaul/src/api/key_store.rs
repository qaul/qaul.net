//! Service API keystore access
//!
//! Provide simple read-only access to a users own keystore. Keys are
//! gathered and managed automatically. This endpoint is primarily
//! meant for service UIs to indicate to users whether or not
//! encrypted communication with another user is possible.

use super::UserAuth;
use crate::{Qaul, QaulResult};

impl Qaul {
    /// Check if another user's public key is available
    pub fn keystore_key_exists(user: UserAuth, id: &str) -> QaulResult<bool> {
        Ok(false)
    }
}
