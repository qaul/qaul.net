use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Result},
};

/// A blob record reference
pub type BlobRef = Arc<Blob>;

/// A large binary object that is streamed from disk
///
/// By itself this object contains nothing but a file descriptor.  You
/// need to call `load()` on it to resolve the future and load the
/// data from disk into memory.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blob {
    /// The path to the file on disk
    path: String,
}

impl Blob {
    
    
    /// Resolve the file descriptor into
    pub async fn load(&mut self) -> Result<Vec<u8>> {
        let mut vec = vec![];
        let mut f = File::open(&self.path)?;
        f.read_to_end(&mut vec)?;
        Ok(vec)
    }
}
