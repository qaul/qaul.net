//! Types related to data being moved around and its verification
use crate::HashBytes;

pub struct Payload {
    // TODO DESIGN: add a payload type field?
    // Upcast to u64 so that there is no issue knowing what the length of a Payload is when sent to different platforms,
    // such as 32 or 128 (?)
    length: u64,
    payload: Vec<u8>,
    signature: HashBytes
}

pub enum PayloadError {
    IncorrectLengthError,
    InvalidSignatureError,
}

impl Payload {
    pub fn pack(payload: Vec<u8>) -> Self {
        Self {
            length: payload.len() as u64,
            payload,
            signature: [0; 32] // TODO: Actually do something here
        }
    }

    pub fn unpack(self) -> Result<Vec<u8>, PayloadError> {
        // TODO: Actually validate the signature
        if self.length != self.payload.len() as u64 {
            Err(PayloadError::IncorrectLengthError)
        } else {
            Ok(self.payload)
        }
    }
}


