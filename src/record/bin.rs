//! Binary body representation

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Result},
};

/// A large binary object that is streamed from disk
///
/// By itself this object contains nothing but a file descriptor.  You
/// need to call `load()` on it to resolve the future and load the
/// data from disk into memory.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bin {
    /// The path to the file on disk
    path: String,
}

impl Bin {
    /// Resolve the file descriptor into a list of bytes
    pub fn load(&mut self) -> Result<Vec<u8>> {
        let mut vec = vec![];
        let mut f = File::open(&self.path)?;
        f.read_to_end(&mut vec)?;
        Ok(vec)
    }
}
