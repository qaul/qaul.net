//! An internal security subsystem
//!
//! This code is responsible for creating identities (keys, and
//! `Identity`), verifying signatures, and encrypting messages

use crate::Identity;
use async_std::sync::Mutex;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

/// A near-stateless security handler
pub(crate) struct Sec {
    rng: Mutex<OsRng>,
}

/// A keypair, and Identity
pub(crate) struct KeyId {
    pub keypair: Keypair,
    pub id: Identity,
}

impl Sec {
    /// Create a new security handler
    pub(crate) fn new() -> Self {
        Self {
            rng: Mutex::new(OsRng {}),
        }
    }

    /// Generate an Id and keypair for a new user
    pub(crate) async fn generate(&self) -> KeyId {
        let mut rng = self.rng.lock().await;
        let keypair = Keypair::generate(&mut *rng);
        let id = Identity::from_bytes(keypair.public.as_bytes());

        KeyId { keypair, id }
    }
}
