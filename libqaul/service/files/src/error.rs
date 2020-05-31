pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    NoSuchFile,
    LibQaul(libqaul::error::Error),
}

impl From<libqaul::error::Error> for Error {
    fn from(lq: libqaul::error::Error) -> Self {
        Self::LibQaul(lq)
    }
}
