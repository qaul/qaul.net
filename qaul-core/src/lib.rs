//! A common abstraction over several network backplanes

pub mod fake;

pub enum Error {
    Failed,
}

/// An interfaces that describes a network link
pub trait Link {
    fn init() -> Self;
    fn send(&mut self) -> Result<(), Error>;
    fn receive<F>(&mut self, cb: F) -> Result<(), Error>
    where
        F: Fn(()) -> Result<(), Error>;
}
