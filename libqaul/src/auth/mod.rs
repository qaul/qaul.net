pub struct Authenticator;
use common::{
    error::Result as QaulResult,
    identity::UserID,
};

impl Authenticator {
    pub fn authenticate(&self, token: &str) -> QaulResult<Option<UserID>> {
        unimplemented!();
    }
}
