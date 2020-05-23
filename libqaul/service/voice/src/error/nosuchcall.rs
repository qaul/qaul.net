use {
    crate::CallId,
    failure::Fail,
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct NoSuchCall(pub CallId);

impl Fail for NoSuchCall {}

impl Display for NoSuchCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "No such call with id '{}'", self.0)
    }
}
