pub struct Authenticator;
use crate::{
    error::QaulResult,
    users::User,
    Qaul,
};

impl Qaul {
    pub fn authenticate(&self, token: &str) -> QaulResult<Option<User>> {
        unimplemented!();
    }
}
