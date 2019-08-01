use base64::DecodeError;
use identity::ID_LEN;
use std::{
    error::Error,
    fmt::{Formatter, Result, Display},
};

/// The error type returned when converting a `String` to an `Identity` fails
#[derive(Debug)]
pub enum ConversionError {
    Base64Decode(DecodeError),
    BadIdLength(usize),
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Conversion Error: ")?;
        match self {
            ConversionError::Base64Decode(e) => write!(f, "Base 64 Decode ({})", e),
            ConversionError::BadIdLength(len) => 
                write!(f, "Bad Id Length (expected {} bytes, got {})", ID_LEN, len), 
        }
    }
}

impl Error for ConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConversionError::Base64Decode(e) => Some(e),
            _ => None,
        }
    }
}

impl From<DecodeError> for ConversionError {
    fn from(e: DecodeError) -> Self {
        ConversionError::Base64Decode(e)
    }
}
