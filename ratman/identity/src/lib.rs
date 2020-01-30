//! # A Ratman network identity abstraction
//!
//! Because Ratman is a userspace router with no concept of link layer
//! identities, network IDs are chosen to be fixed size byte arrays.
//! It's left to the implementing application to map these to some
//! useful source of identity.  This crate also provides a hashing
//! constructor behind the `digest` feature flag which can be used to
//! hash a secret to derive the identity value. 
//!
//! Whatever scheme is chosen, two principles about identity must not
//! be violated:
//!
//! 1. There are no identity collisions
//! 2. Identities don't change mid-route
//!
//! This crate is part of the qaul.net project.  The docs for this
//! crate are propably lacking because currently Ratman/ libqaul are
//! the only users of it.  If you have questions, don't hesitate to
//! [contact us]!
//!
//! [contact us]: https://docs.qaul.net/contributors/social/_intro.html

use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display, Formatter};
pub const ID_LEN: usize = 16;

/// A Ratman network identity
///
/// Note: this type implements `Eq`, which is a much better way of
/// comparing two instances of Identity information than making a
/// string comparison.  While this might be convenient in certain API
/// settings, we can't make the promise that the `Display`
/// implementation will never change.
#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identity([u8; ID_LEN]);

impl Debug for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<ID: {}>", hex::encode_upper(self))
    }
}

impl Display for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = hex::encode_upper(self);
        let mut v = s
            .as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .join(" ");
        v.insert(20, ' ');
        write!(f, "{}", v)
    }
}

impl Identity {
    /// Create an identity from the first 16 bytes of a vector
    ///
    /// This function will panic, if the provided vector isn't long
    /// enough, but extra data will simply be discarded.
    pub fn truncate<'vec, V: Into<&'vec Vec<u8>>>(vec: V) -> Self {
        let vec = vec.into();
        assert!(vec.len() >= 16);

        Self(
            vec.into_iter()
                .zip(0..16)
                .take(ID_LEN)
                .fold([0; ID_LEN], |mut buf, (u, i)| {
                    buf[i] = *u;
                    buf
                }),
        )
    }

    /// Create an identity using a digest function
    ///
    /// This allows you to pass arbitrary length data which will
    /// result in a precise ID length data output.  The hash function
    /// is the cryptographic [blake2] cipher, so it can be used to
    /// turn secrets into identity information.
    ///
    /// This function requires the `digest` feature.
    ///
    /// [blake2]: https://blake2.net/
    #[cfg(feature = "digest")]
    pub fn with_digest<'vec, V: Into<&'vec Vec<u8>>>(vec: V) -> Self {
        use blake2::{
            digest::{Input, VariableOutput},
            VarBlake2b,
        };

        let mut hasher = VarBlake2b::new(ID_LEN).unwrap();
        hasher.input(vec.into());
        Self::truncate(&hasher.vec_result())
    }
}

/// Implement RAW `From` binary array
impl From<[u8; ID_LEN]> for Identity {
    fn from(i: [u8; ID_LEN]) -> Self {
        Self(i)
    }
}

/// Implement RAW `From` binary (reference) array
impl From<&[u8; ID_LEN]> for Identity {
    fn from(i: &[u8; ID_LEN]) -> Self {
        Self(i.clone())
    }
}

/// Implement binary array `From` RAW
impl From<Identity> for [u8; ID_LEN] {
    fn from(i: Identity) -> Self {
        i.0
    }
}

/// Implement binary array `From` RAW reference
impl From<&Identity> for [u8; ID_LEN] {
    fn from(i: &Identity) -> Self {
        i.0.clone()
    }
}

/// Implement RAW identity to binary array reference
impl AsRef<[u8]> for Identity {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
