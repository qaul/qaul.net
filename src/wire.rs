//! Wire serialiser formats

use bincode::{self, Result};
use serde::{de::DeserializeOwned, Serialize};

/// A generic trait for anything that can be serialised
pub(crate) trait Encoder<T: Sized> {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(data: &Vec<u8>) -> Result<T>;
}

// Blanket impl for anything than can be `Encoder<T>`
impl<T> Encoder<T> for T
where
    T: Serialize + DeserializeOwned,
{
    fn encode(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
    }

    fn decode(data: &Vec<u8>) -> Result<T> {
        bincode::deserialize(data)
    }
}
