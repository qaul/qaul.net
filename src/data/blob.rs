/// A blob record reference
pub type BlobRef = Arc<Blob>;

/// A large binary object that is streamed from disk
///
/// By itself this object contains nothing but a file descriptor.  You
/// need to call `load()` on it to resolve the future and load the
/// data from disk into memory.
pub struct Blob {
    /// The filedescriptor of the object
    fd: File,
}

impl Blob {
    /// Resolve the file descriptor into
    pub async fn load(&mut self) -> Result<Vec<u8>> {
        let mut vec = vec![];
        self.fd.read_to_end(&mut vec).await?;
        Ok(vec)
    }
}
