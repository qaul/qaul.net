// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group-file envelope encryption primitives
//!
//! Hybrid ("envelope") encryption for group files: the file body is
//! encrypted **once** under a random per-file symmetric key
//! (`file_key`), and that key is distributed to each group member
//! under their existing per-peer Noise session (a `FileKeyEnvelope`).
//! This replaces encrypting the whole file once per recipient, cutting
//! the per-file cost from O(members × file_size) to
//! O(file_size) + O(members × 32 bytes).
//!
//! See `docs/proposals/Efficient-Group-File-Encryption.md`.
//!
//! The body cipher reuses the Noise ChaCha20-Poly1305 `CipherState`
//! (already a dependency) rather than pulling in a second AEAD crate.
//! Because `file_key` is freshly random for every file and used for
//! exactly one body, a fixed nonce is safe (no key+nonce pair is ever
//! reused). The envelope↔body binding that the spec's AAD would
//! provide is instead carried by `body_digest`, which the receiver
//! verifies before decrypting.

use noise_protocol::CipherState;
use noise_rust_crypto::ChaCha20Poly1305;
use rand::Rng;
use sha2::{Digest, Sha256};

/// Length of a file content key in bytes (ChaCha20-Poly1305 key).
pub const FILE_KEY_LEN: usize = 32;

/// Fixed nonce for the single body encryption under a per-file key.
/// Safe because `file_key` is random per file and never reused.
const BODY_NONCE: u64 = 0;

/// Generate a fresh random 32-byte file content key.
pub fn generate_file_key() -> Vec<u8> {
    let mut rng = rand::rng();
    let mut key = vec![0u8; FILE_KEY_LEN];
    rng.fill(&mut key[..]);
    key
}

/// Encrypt a file body once under `file_key`. Returns the ciphertext
/// (plaintext length + 16-byte Poly1305 tag).
pub fn encrypt_body(file_key: &[u8], body: &[u8]) -> Vec<u8> {
    let mut cipher: CipherState<ChaCha20Poly1305> = CipherState::new(file_key, BODY_NONCE);
    cipher.encrypt_vec(body)
}

/// Decrypt a file body produced by [`encrypt_body`]. Returns `None`
/// on authentication failure (wrong key or tampered ciphertext).
pub fn decrypt_body(file_key: &[u8], body_ct: &[u8]) -> Option<Vec<u8>> {
    let mut cipher: CipherState<ChaCha20Poly1305> = CipherState::new(file_key, BODY_NONCE);
    cipher.decrypt_vec(body_ct).ok()
}

/// SHA-256 digest of an encrypted body, used to bind a
/// `FileKeyEnvelope` to exactly one body. The receiver verifies
/// `body_digest(body_ct) == envelope.body_digest` before decrypting,
/// so a member cannot be tricked into decrypting one file's body under
/// another file's key.
pub fn body_digest(body_ct: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(body_ct);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_round_trip() {
        let key = generate_file_key();
        assert_eq!(key.len(), FILE_KEY_LEN);
        let body = b"the quick brown fox jumps over the lazy dog".to_vec();
        let ct = encrypt_body(&key, &body);
        assert_ne!(ct, body, "ciphertext must differ from plaintext");
        assert_eq!(ct.len(), body.len() + 16, "ct = plaintext + 16-byte tag");
        let pt = decrypt_body(&key, &ct).expect("decrypt with correct key");
        assert_eq!(pt, body);
    }

    #[test]
    fn wrong_key_fails() {
        let body = b"secret group file".to_vec();
        let ct = encrypt_body(&generate_file_key(), &body);
        assert!(
            decrypt_body(&generate_file_key(), &ct).is_none(),
            "a different key must not decrypt"
        );
    }

    #[test]
    fn tampered_ciphertext_fails() {
        let key = generate_file_key();
        let mut ct = encrypt_body(&key, b"important bytes");
        let last = ct.len() - 1;
        ct[last] ^= 0x01; // flip a bit in the tag
        assert!(
            decrypt_body(&key, &ct).is_none(),
            "AEAD must reject a tampered ciphertext"
        );
    }

    #[test]
    fn digest_binds_body() {
        let key = generate_file_key();
        let ct_a = encrypt_body(&key, b"file A body");
        let ct_b = encrypt_body(&key, b"file B body");
        assert_eq!(body_digest(&ct_a).len(), 32);
        assert_ne!(
            body_digest(&ct_a),
            body_digest(&ct_b),
            "different bodies must have different digests"
        );
        // digest is stable
        assert_eq!(body_digest(&ct_a), body_digest(&ct_a));
    }

    #[test]
    fn two_file_keys_differ() {
        assert_ne!(
            generate_file_key(),
            generate_file_key(),
            "each file key must be independently random"
        );
    }
}
