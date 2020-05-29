use failure::Error;

mod nosuchcall;
mod nosuchstream;

pub use self::{
    nosuchcall::NoSuchCall,
    nosuchstream::NoSuchStream,
};

pub type Result<T> = std::result::Result<T, Error>;
