//! `RATMAN` identity management
//!
//! ## Why?!
//!
//! Because `RATMAN` does routing based on higher-abstraction IDs
//! that don't neccessarily map lineraly to an IP address
//! (or because some networks might not _use_ IP addresses),
//! we need to introduce an ID abstraction to do routing with.
//!
//! An ID in it's simplest form is the hash of a private key
//! (i.e. fingerprint).
//!
//! This library provides two things:
//!
//! 1. A small serialisable datastructure that represents an ID
//! 2. Some utility functions to _generate_ an ID based on input
//!
//! This input doesn't have to be the same for all users of RATMAN.
//! In `qaul.net` we use the hash of a private key,
//! but in your application you are welcome to use something else.
//!
//! As long as two principles aren't violated, RATMAN will work:
//!
//! 1. IDs don't collide on a network
//! 2. IDs don't change mid-frame-transport
//!
//! ## License
//!
//! This library is part of `RATMAN` and as such licensed under the
//! GNU General Public License 3.0 (or later).
//! You will have received a copy of this license
//! with the source code of this project.

pub const ID_LEN: usize = 12;

/// A RATMAN identity
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Identity([u8; ID_LEN]);

impl Identity {
    /// Create an identity from the first 12 bytes of a vector
    pub fn truncate<'vec, V: Into<&'vec Vec<u8>>>(vec: V) -> Self {
        Self(
            vec.into()
                .into_iter()
                .zip(0..)
                .take(ID_LEN)
                .fold([0; ID_LEN], |mut buf, (u, i)| {
                    buf[i] = *u;
                    buf
                }),
        )
    }

    /// Create an identity using a digest function
    ///
    /// This allows you to pass arbitrary length data
    /// which will result in a precise ID length data output.
    ///
    /// This process can cause collisions!
    #[cfg(feature = "digest")]
    pub fn with_digest<'vec, V: Into<&'vec Vec<u8>>>(vec: V) -> Self {
        unimplemented!()
    }
}

/// Implement RAW `From` binary array
impl From<[u8; 12]> for Identity {
    fn from(i: [u8; 12]) -> Self {
        Self(i)
    }
}

/// Implement RAW `From` binary (reference) array
impl From<&[u8; 12]> for Identity {
    fn from(i: &[u8; 12]) -> Self {
        Self(i.clone())
    }
}
