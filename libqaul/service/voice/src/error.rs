use crate::CallId;
use failure::{Error, Fail};
use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

/// The call database contains no call with the given id
#[derive(Debug)]
pub struct NoSuchCall(pub CallId);

impl Fail for NoSuchCall {}

impl Display for NoSuchCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "No such call with id '{}'", self.0)
    }
}
