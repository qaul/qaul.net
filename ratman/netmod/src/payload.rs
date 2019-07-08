use blake2::{Blake2b, Digest as _};
use generic_array::typenum::U64;
use generic_array::GenericArray;

pub type Digest = GenericArray<u8, U64>;

/// The actual content of a frame, along with the mechanism to validate that
/// no transmission errors occurred (message digest of payload type and data).
pub struct Payload {
    // The length of the data in this payload
    length: u32,
    // Message digest used to ensure integrity
    digest: Digest,
    // The actual data being transmitted
    data: Vec<u8>
}

// Compute and return the 64-bit Blake2b digest of the given data.
fn blake2b_digest(data: &[u8]) -> Digest {
    Blake2b::digest(data)
}

/// Errors that can occur while working with a Payload.
#[derive(PartialEq, Eq, Debug)]
pub enum PayloadError {
    /// The given payload did not match the length included in the wrapper.
    IncorrectLengthError,
    /// The given payload did not match its associated message digest.
    InvalidDigestError,
}

impl Payload {
    /// Place a binary payload into a `Payload` metadata package.
    pub fn pack<T: Into<Vec<u8>>>(data: T) -> Self {
        let data = data.into();
        let digest = blake2b_digest(&data);
        Self {
            length: data.len() as u32,
            data,
            digest,
        }
    }

    /// Attempt to extract a binary payload from a `Payload` metadata package.
    /// Fails if the message was corrupted in any way.
    pub fn unpack(self) -> Result<Vec<u8>, PayloadError> {
        let digest = blake2b_digest(&self.data);

        if self.length != self.data.len() as u32 {
            Err(PayloadError::IncorrectLengthError)
        } else if digest != self.digest {
            Err(PayloadError::InvalidDigestError)
        } else {
            Ok(self.data)
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
