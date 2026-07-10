// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Identifying a user/node on the protocol. Inlcuding the user/node profile.
use libp2p::identity::PublicKey;
use sha2::{Digest, Sha256};

use crate::{QaulState, node::{Node, user_accounts::UserAccount}, router_v2::Result};

#[derive(Debug)]
pub struct Multikey(PublicKey);

impl From<PublicKey> for Multikey {
    fn from(value: PublicKey) -> Self {
        Self(value)
    }
}

impl Multikey {
    pub fn encode(&self) -> Vec<u8> {
        self.0.encode_protobuf()
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let key = PublicKey::try_decode_protobuf(bytes)?;
        Ok(Multikey(key))
    }

    pub fn to_id(&self) -> [u8; 8] {
        let hash = Sha256::digest(self.encode());
        let mut id = [0u8; 8];
        id.copy_from_slice(&hash[..8]);
        id
    }

    pub fn verify(&self,  msg: &[u8], sig: &[u8]) -> bool {
        self.0.verify(msg, sig)
    }
}

#[derive(Debug)]
pub struct Profile {
    pub multikey: Multikey,
    pub version: u32,
    pub name: String,
    pub self_signature: [u8; 64],
}

impl Profile {
    pub fn sign_input(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(40);
        buf.extend_from_slice(&self.multikey.encode());
        buf.extend_from_slice(&self.version.to_be_bytes());
        buf.extend_from_slice(&self.name.as_bytes());
        buf
    }
}

pub fn delegation_signing_input(host_full_multikey: &[u8], timeout: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(host_full_multikey);
    buf.extend_from_slice(&timeout.to_be_bytes());
    buf
}

/// Simple struct holder to sign chunks for the MANIFEST_DELTA
pub struct ChunkSigningCtx<'a> {
    pub origin_multikey: &'a [u8],
    pub manifest_version: u32,
    pub chunk_index: u8,
    pub chunk_count: u8,
    pub flags: u8,
    pub canonical_entries: &'a [u8],
}

impl<'a> ChunkSigningCtx<'a> {
    pub fn signing_input(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.origin_multikey);
        buf.extend_from_slice(&self.manifest_version.to_be_bytes());
        buf.extend_from_slice(&self.chunk_index.to_be_bytes());
        buf.extend_from_slice(&self.chunk_count.to_be_bytes());
        buf.extend_from_slice(&self.flags.to_be_bytes());
        buf.extend_from_slice(&self.canonical_entries);
        buf
    }
}

impl UserAccount {
    pub fn multikey(&self) -> Multikey {
        let pk = &self.keys.clone();
        pk.public().into()
    }

    pub fn routing_user_id(&self) -> [u8; 8] {
        self.multikey().to_id()
    }

    pub fn sign_with_user(&self, buf: &[u8]) -> [u8; 64] {
        let pk = &self.keys.clone();
        pk
            .sign(buf)
            .expect("ed25519 sign")
            .try_into()
            .expect("ed25519 signatures are 64 bytes")
    }
}

pub fn get_host_multikey(state: &QaulState) -> Multikey {
    let keys = Node::get_keys(state);
    keys.public().into()
}

pub fn get_host_id(state: &QaulState) -> [u8; 8] {
    get_host_multikey(state).to_id()
}

pub fn sign_with_host(state: &QaulState, buf: &[u8]) -> [u8; 64] {
    let keypair = Node::get_keys(state);
    keypair
        .sign(buf)
        .expect("ed25519 sign")
        .try_into()
        .expect("ed25519 signatures are 64 bytes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    fn sign_into_array(kp: &Keypair, input: &[u8]) -> [u8; 64] {
        kp.sign(input)
            .expect("ed25519 sign")
            .try_into()
            .expect("ed25519 signatures are 64 bytes")
    }

    fn fresh_profile(name: &str, version: u32) -> (Keypair, Profile) {
        let kp = Keypair::generate_ed25519();
        let multikey: Multikey = kp.public().into();
        let profile = Profile {
            multikey,
            version,
            name: name.into(),
            self_signature: [0u8; 64],
        };
        (kp, profile)
    }

    #[test]
    fn multikey_to_id_is_deterministic() {
        let (_, profile) = fresh_profile("alice", 1);
        let id1 = profile.multikey.to_id();
        let id2 = profile.multikey.to_id();
        assert_eq!(id1, id2, "same multikey must always derive the same id");
    }

    #[test]
    fn multikey_to_id_differs_across_keys() {
        let (_, p_a) = fresh_profile("alice", 1);
        let (_, p_b) = fresh_profile("alice", 1);
        assert_ne!(
            p_a.multikey.to_id(),
            p_b.multikey.to_id(),
            "distinct keypairs must produce distinct ids"
        );
    }

    #[test]
    fn multikey_encode_decode_roundtrips_through_protobuf() {
        let (_, profile) = fresh_profile("alice", 1);
        let bytes = profile.multikey.encode();
        let decoded = Multikey::decode(&bytes).expect("decode succeeds");
        assert_eq!(
            profile.multikey.to_id(),
            decoded.to_id(),
            "encode then decode must preserve the underlying key"
        );
    }

    #[test]
    fn profile_sign_verify_roundtrip() {
        let (kp, mut profile) = fresh_profile("alice", 1);
        profile.self_signature = sign_into_array(&kp, &profile.sign_input());

        assert!(
            kp.public()
                .verify(&profile.sign_input(), &profile.self_signature),
            "signature must verify against the same input that produced it"
        );
    }

    #[test]
    fn profile_verify_fails_on_tampered_name() {
        let (kp, mut profile) = fresh_profile("alice", 1);
        profile.self_signature = sign_into_array(&kp, &profile.sign_input());

        profile.name = "bob".into();

        assert!(
            !kp.public()
                .verify(&profile.sign_input(), &profile.self_signature),
            "verification must fail when the signed name has changed"
        );
    }

    #[test]
    fn profile_verify_fails_on_tampered_version() {
        let (kp, mut profile) = fresh_profile("alice", 1);
        profile.self_signature = sign_into_array(&kp, &profile.sign_input());

        profile.version = profile.version.wrapping_add(1);

        assert!(
            !kp.public()
                .verify(&profile.sign_input(), &profile.self_signature),
            "verification must fail when the signed version has changed"
        );
    }

    #[test]
    fn profile_verify_fails_on_tampered_signature() {
        let (kp, mut profile) = fresh_profile("alice", 1);
        profile.self_signature = sign_into_array(&kp, &profile.sign_input());

        // Flip every bit of the first signature byte.
        profile.self_signature[0] ^= 0xFF;

        assert!(
            !kp.public()
                .verify(&profile.sign_input(), &profile.self_signature),
            "verification must fail when the signature bytes are altered"
        );
    }

    #[test]
    fn delegation_sign_verify_roundtrip() {
        let user_kp = Keypair::generate_ed25519();
        let host_kp = Keypair::generate_ed25519();
        let host_mk_bytes = Multikey::from(host_kp.public()).encode();
        let timeout = 1_700_000_000_000_u64;

        let sig = user_kp
            .sign(&delegation_signing_input(&host_mk_bytes, timeout))
            .expect("ed25519 sign");

        assert!(
            user_kp
                .public()
                .verify(&delegation_signing_input(&host_mk_bytes, timeout), &sig),
            "delegation must verify with original host and timeout"
        );
    }

    ///the signing input includes the timeout, so
    /// altering it after signature production must break verification.
    #[test]
    fn delegation_verify_fails_on_tampered_timeout() {
        let user_kp = Keypair::generate_ed25519();
        let host_kp = Keypair::generate_ed25519();
        let host_mk_bytes = Multikey::from(host_kp.public()).encode();
        let timeout = 1_700_000_000_000_u64;

        let sig = user_kp
            .sign(&delegation_signing_input(&host_mk_bytes, timeout))
            .expect("ed25519 sign");

        let tampered = delegation_signing_input(&host_mk_bytes, timeout + 1);
        assert!(
            !user_kp.public().verify(&tampered, &sig),
            "altering timeout after signing must invalidate the signature"
        );
    }

    #[test]
    fn delegation_verify_fails_on_tampered_host_multikey() {
        let user_kp = Keypair::generate_ed25519();
        let host_kp_a = Keypair::generate_ed25519();
        let host_kp_b = Keypair::generate_ed25519();
        let host_a_bytes = Multikey::from(host_kp_a.public()).encode();
        let host_b_bytes = Multikey::from(host_kp_b.public()).encode();
        let timeout = 1_700_000_000_000_u64;

        let sig = user_kp
            .sign(&delegation_signing_input(&host_a_bytes, timeout))
            .expect("ed25519 sign");

        let tampered = delegation_signing_input(&host_b_bytes, timeout);
        assert!(
            !user_kp.public().verify(&tampered, &sig),
            "altering host multikey after signing must invalidate the signature"
        );
    }

    struct ChunkFixture {
        host_mk: Vec<u8>,
        manifest_version: u32,
        chunk_index: u8,
        chunk_count: u8,
        flags: u8,
        entries: Vec<u8>,
    }

    impl ChunkFixture {
        fn ctx(&self) -> ChunkSigningCtx<'_> {
            ChunkSigningCtx {
                origin_multikey: &self.host_mk,
                manifest_version: self.manifest_version,
                chunk_index: self.chunk_index,
                chunk_count: self.chunk_count,
                flags: self.flags,
                canonical_entries: &self.entries,
            }
        }
    }

    fn fresh_chunk_fixture() -> (Keypair, ChunkFixture) {
        let host_kp = Keypair::generate_ed25519();
        let host_mk = Multikey::from(host_kp.public()).encode();
        let fixture = ChunkFixture {
            host_mk,
            manifest_version: 7,
            chunk_index: 0,
            chunk_count: 1,
            flags: 0,
            entries: vec![1, 2, 3, 4, 5, 6, 7, 8],
        };
        (host_kp, fixture)
    }

    #[test]
    fn chunk_sign_verify_roundtrip() {
        let (host_kp, fixture) = fresh_chunk_fixture();
        let sig = host_kp
            .sign(&fixture.ctx().signing_input())
            .expect("ed25519 sign");

        assert!(
            host_kp
                .public()
                .verify(&fixture.ctx().signing_input(), &sig),
            "chunk signature must verify with the same inputs"
        );
    }

    /// flags must be in the signed content so an
    /// attacker cannot flip the is_gateway bit without invalidating
    /// the signature.
    #[test]
    fn chunk_verify_fails_on_tampered_flags() {
        let (host_kp, mut fixture) = fresh_chunk_fixture();
        let sig = host_kp
            .sign(&fixture.ctx().signing_input())
            .expect("ed25519 sign");

        fixture.flags ^= 0x01;

        assert!(
            !host_kp
                .public()
                .verify(&fixture.ctx().signing_input(), &sig),
            "flipping the flags byte must invalidate the signature"
        );
    }

    #[test]
    fn chunk_verify_fails_on_tampered_manifest_version() {
        let (host_kp, mut fixture) = fresh_chunk_fixture();
        let sig = host_kp
            .sign(&fixture.ctx().signing_input())
            .expect("ed25519 sign");

        fixture.manifest_version = fixture.manifest_version.wrapping_add(1);

        assert!(
            !host_kp
                .public()
                .verify(&fixture.ctx().signing_input(), &sig),
            "altering manifest_version must invalidate the signature"
        );
    }

    #[test]
    fn chunk_verify_fails_on_tampered_chunk_index() {
        let (host_kp, mut fixture) = fresh_chunk_fixture();
        // Set chunk_count > 1 so chunk_index has somewhere to move.
        fixture.chunk_count = 3;
        let sig = host_kp
            .sign(&fixture.ctx().signing_input())
            .expect("ed25519 sign");

        fixture.chunk_index = 2;

        assert!(
            !host_kp
                .public()
                .verify(&fixture.ctx().signing_input(), &sig),
            "altering chunk_index must invalidate the signature"
        );
    }

    #[test]
    fn chunk_verify_fails_on_tampered_entries() {
        let (host_kp, mut fixture) = fresh_chunk_fixture();
        let sig = host_kp
            .sign(&fixture.ctx().signing_input())
            .expect("ed25519 sign");

        fixture.entries[0] ^= 0xFF;

        assert!(
            !host_kp
                .public()
                .verify(&fixture.ctx().signing_input(), &sig),
            "altering canonical_entries must invalidate the signature"
        );
    }
}
