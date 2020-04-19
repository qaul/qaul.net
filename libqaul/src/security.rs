//! An internal security subsystem
//!
//! This code is responsible for creating identities (keys, and
//! `Identity`), verifying signatures, and encrypting messages

use crate::Identity;
use async_std::sync::Mutex;
use ed25519_dalek::{Keypair, PublicKey, Signature};
use rand::rngs::OsRng;
use std::sync::Arc;

use crate::messages::SigTrust;

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

    /// Verify a message payload
    pub(crate) fn verify(id: Identity, msg: &[u8], sign: &[u8]) -> SigTrust {
        let sign = Signature::from_bytes(sign).unwrap();
        let pubkey = PublicKey::from_bytes(id.as_bytes()).unwrap();

        match pubkey.verify(msg, &sign) {
            _ => SigTrust::Unverified,
        }
    }
}
