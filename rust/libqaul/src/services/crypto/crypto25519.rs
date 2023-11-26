// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Curve 25519 Operations
//!
//! Conversation of Ed25519 to Montgomery Curve25519 representation.

use libp2p;
use libp2p::identity::{Keypair, PublicKey};
use sha2::{Digest, Sha512};

pub struct Crypto25519 {}

impl Crypto25519 {
    /// Convert ed25519 private key to montgomery form
    ///
    /// libqaul's Keypair is an `ed25519::KeyPair`
    /// To get to the montgomery form, the following actions are done:
    ///
    /// 1) Get the `SecretKey` from the KeyPair structure.
    /// 2) Convert the `EdwardsPoint` to a `MontgomeryPoint` by hashing it with sha512.
    /// 3) This gives us the bytes of the curve25519_dalek secret key in the Montgomery form to be used with for the X25519 Diffie-Hellman.
    ///
    /// ## References
    ///
    /// * libqaul's `Keypair` = [`libp2p::identity::ed25519::Keypair`](https://docs.rs/libp2p/latest/libp2p/identity/enum.Keypair.html)
    /// * `EdwardsPoint` = [`curve25519_dalek::edwards::EdwardsPoint`](https://doc.dalek.rs/curve25519_dalek/edwards/struct.EdwardsPoint.html)
    /// * `MontgomeryPoint` = [`curve25519_dalek::montgomery::MontgomeryPoint`](https://doc.dalek.rs/curve25519_dalek/montgomery/struct.MontgomeryPoint.html)
    /// * [`x25519_dalek::PublicKey`](https://doc.dalek.rs/x25519_dalek/struct.PublicKey.html)
    ///
    pub fn private_key_to_montgomery(key: Keypair) -> Option<Vec<u8>> {
        // get ed25519 keypair
        #[allow(irrefutable_let_patterns)]
        if let Ok(ed25519_keypair) = key.try_into_ed25519() {
            // get dalek keypair as bytes
            //
            // unfortunately the dalek keypair is private in the
            // libp2p keypair structure, therefore we have to
            // make the detour via the bytes.
            let ed25519_dalek_keypair_bytes = ed25519_keypair.to_bytes();

            // create dalek ed25519 keypair
            if let Ok(ed25519_dalek_signingkey) =
                ed25519_dalek::SigningKey::from_keypair_bytes(&ed25519_dalek_keypair_bytes)
            {
                // get dalek secret key as bytes
                let ed25519_dalek_secret_bytes = ed25519_dalek_signingkey.to_bytes();

                // make sure the transformation was correct and we have the correct secret key
                {
                    let retransformed_ed25519_keypair = libp2p::identity::ed25519::Keypair::from(
                        libp2p::identity::ed25519::SecretKey::try_from_bytes(
                            ed25519_dalek_secret_bytes,
                        )
                        .unwrap(),
                    );
                    assert!(
                        ed25519_dalek_keypair_bytes == retransformed_ed25519_keypair.to_bytes(),
                        "secret key transformation failed"
                    );
                }

                // transform into dalek curve25519 secret key as bytes
                let mut curve25519_dalek_secret: [u8; 32] = [0; 32];
                let hash = Sha512::digest(ed25519_dalek_secret_bytes.as_ref());
                curve25519_dalek_secret.copy_from_slice(&hash[..32]);

                return Some(curve25519_dalek_secret.to_vec());
            }
        }

        None
    }

    /// Convert ed25519 public key to montgomery form
    ///
    /// Libqaul's PublicKey is a `CompressedEdwardsY` point from the `curve25519_dalek` library.
    /// To get to the montgomery form, the following actions are done:
    ///
    /// 1) Decompress `CompressedEdwardsY` point, which returns an `EdwardsPoint`.
    /// 2) Convert the `EdwardsPoint` to a `MontgomeryPoint`
    /// 3) The bytes of the `MontgomeryPoint` are equal to the `x25519_dalek::PublicKey`
    ///
    /// ## References
    ///
    /// * libqaul's `PublicKey` = [`libp2p::identity::ed25519::PublicKey`](https://docs.rs/libp2p/latest/libp2p/identity/ed25519/struct.PublicKey.html)
    /// * `EdwardsPoint` = [`curve25519_dalek::edwards::EdwardsPoint`](https://doc.dalek.rs/curve25519_dalek/edwards/struct.EdwardsPoint.html)
    /// * `CompressedEdwardsY` = [`curve25519_dalek::edwards::CompressedEdwardsY`](https://doc.dalek.rs/curve25519_dalek/edwards/struct.CompressedEdwardsY.html)
    /// * `MontgomeryPoint` = [`curve25519_dalek::montgomery::MontgomeryPoint`](https://doc.dalek.rs/curve25519_dalek/montgomery/struct.MontgomeryPoint.html)
    /// * [`x25519_dalek::PublicKey`](https://doc.dalek.rs/x25519_dalek/struct.PublicKey.html)
    ///
    pub fn public_key_to_montgomery(key: PublicKey) -> Option<x25519_dalek::PublicKey> {
        // get ed25519 structure
        #[allow(irrefutable_let_patterns)]
        if let Ok(ed25519_pub) = key.try_into_ed25519() {
            // convert to dalek public key in bytes form
            let dalek_pub_bytes = ed25519_pub.to_bytes();

            // generate Montgomery form
            // x25519_dalek::PublicKey internal is private, we have to go via bytes
            let montgomery_bytes = curve25519_dalek::edwards::CompressedEdwardsY(dalek_pub_bytes)
                .decompress()
                .expect("An Ed25519 public key is a valid point by construction.")
                .to_montgomery()
                .0;

            return Some(x25519_dalek::PublicKey::from(montgomery_bytes));
        }

        None
    }
}
