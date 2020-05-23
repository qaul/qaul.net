use failure::Error;

mod nosuchcall;

pub use self::{
    nosuchcall::NoSuchCall,
};

pub type Result<T> = std::result::Result<T, Error>;
