use {
    crate::StreamId,
    failure::Fail,
    std::fmt::{Display, Formatter, Result},
};

/// The call database contains no call with the given id
#[derive(Debug)]
pub struct NoSuchStream(pub StreamId);

impl Fail for NoSuchStream {}

impl Display for NoSuchStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "No such stream with id '{}'", self.0)
    }
}

