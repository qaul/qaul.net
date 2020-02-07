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

use serde::{Deserialize, Serialize, Serializer, de::{Deserializer, Visitor, SeqAccess}};
use std::{
    fmt::{self, Debug, Display, Formatter},
    string::ToString,
};

/// Length of the identity buffer
pub const ID_LEN: usize = 16;

/// A Ratman network identity
///
/// Note: this type implements `Eq`, which is a much better way of
/// comparing two instances of Identity information than making a
/// string comparison.  While this might be convenient in certain API
/// settings, we can't make the promise that the `Display`
/// implementation will never change.
#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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

impl Serialize for Identity {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if ser.is_human_readable() {
            self.to_string().serialize(ser)
        } else {
            self.serialize(ser)
        }
    }
}

impl<'de> Deserialize<'de> for Identity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
    where D: Deserializer<'de> {
        use serde::de::Error;

        struct IdentityVisitor;

        impl IdentityVisitor {
            fn from_str<E: Error>(v: &str) -> Result<Identity, E> {
                let v : Vec<u8> = v.split(" ")
                    .map(|s| hex::decode(s).map_err(|e| E::custom(e)))
                    // I don't like this way of propagating errors up but the alternative
                    // is a for loop which i also don't like
                    .collect::<Result<Vec<Vec<u8>>, E>>()?
                    .into_iter()
                    .flatten()
                    .collect();

                Self::from_bytes(&v)
            }

            fn from_bytes<E: Error, V: AsRef<[u8]>>(v: V) -> Result<Identity, E> {
                let v = v.as_ref();
                if v.len() != ID_LEN {
                    return Err(E::custom(format!("Expected {} bytes, got {}", ID_LEN, v.len()))); 
                }

                Ok(Identity(v.iter()
                    .enumerate()
                    .take(ID_LEN)
                    .fold([0; ID_LEN], |mut buf, (i, u)| {
                        buf[i] = *u;
                        buf
                    })))
            }
        }

        impl<'de> Visitor<'de> for IdentityVisitor {
            type Value = Identity; 

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "Either a {l} byte array or a hex string representing {l} bytes", l = ID_LEN)
            }

            fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> { 
                Self::from_str(v)
            }

            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Self::from_str(&v)
            }

            fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                Self::from_bytes(v)
            }

            fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Self::from_bytes(v)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> 
            where A: SeqAccess<'de> {
                let mut v = Vec::new();
                while let Some(b) = seq.next_element::<u8>()? {
                    v.push(b);
                }

                Self::from_bytes(v)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(IdentityVisitor)
        } else {
            deserializer.deserialize_bytes(IdentityVisitor)
        }
    }
}
