//! Types related to data being moved around and its verification.
use crate::HashBytes;
use blake2::{Blake2b, Digest};
use generic_array::GenericArray;

/// The actual content of a message, along with the mechanism to validate that
/// no transmission errors occurred (message digest).
#[derive(PartialEq, Eq, Debug)]
pub struct Payload {
    // TODO DESIGN: add a payload type field?
    // Upcast to u64 so that there is no issue knowing what the length of a Payload is
    // when sent to different platforms.
    length: u64,
    payload: Vec<u8>,
    digest: HashBytes
}

// Compute and return the 64-bit Blake2b digest of the given data.
fn blake2b_digest(data: &[u8]) -> HashBytes { Blake2b::digest(data) }

/// All the things that can go wrong while working with a Payload.
#[derive(PartialEq, Eq, Debug)]
pub enum PayloadError {
    /// The given payload did not match the length included in the wrapper.
    IncorrectLengthError,
    /// The given payload did not match its associated message digest.
    InvalidDigestError,
}

impl Payload {
    /// Place a binary payload into a `Payload` metadata package.
    pub fn pack<T: Into<Vec<u8>>>(payload: T) -> Self {
        let payload = payload.into();
        let digest = blake2b_digest(&payload);
        Self {
            length: payload.len() as u64,
            payload,
            digest
        }
    }

    /// Attempt to extract a binary payload from a `Payload` metadata package.
    /// Fails if the message was corrupted in any way.
    pub fn unpack(self) -> Result<Vec<u8>, PayloadError> {
        let digest = blake2b_digest(&self.payload); 

        if self.length != self.payload.len() as u64 {
            Err(PayloadError::IncorrectLengthError)
        } else if digest != self.digest {
            Err(PayloadError::InvalidDigestError)
        }  else {
            Ok(self.payload)
        }
    }
}

#[test]
fn pack_and_unpack_success() {
    let payload = Payload::pack(b"Hello, world!".to_vec());
    let data = payload.unpack().expect("Failed to unpack message.");
    assert_eq!(b"Hello, world!".to_vec(), data);
}

#[test]
fn pack_and_unpack_bad_length() {
    let mut payload = Payload::pack(b"Hello, world!".to_vec());
    payload.length -= 1; 
    assert_eq!(Err(PayloadError::IncorrectLengthError), payload.unpack());
}


