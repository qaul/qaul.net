use crate::QaulCore;
use iron::{
    BeforeMiddleware,
    prelude::*,
    headers::{Authorization, Bearer},
    typemap,
};
use libqaul::{
    Identity,
    UserAuth, 
};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
    },
};

pub struct CurrentUser;

impl typemap::Key for CurrentUser { type Value = UserAuth; }

#[derive(Clone)]
pub (crate) struct Authenticator{
    pub tokens: Arc<Mutex<HashMap<String, Identity>>>,
}

impl Authenticator {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl typemap::Key for Authenticator { type Value = Self; }

impl BeforeMiddleware for Authenticator {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.headers.get::<Authorization<Bearer>>()
            .and_then(|bearer| self.tokens.lock().unwrap().get(&bearer.token)
                .map(|identity| UserAuth::Trusted(*identity, bearer.token.clone())))
            .and_then(|ua| req.extensions.insert::<CurrentUser>(ua));

        req.extensions.insert::<Authenticator>(self.clone());

        Ok(())
    }
}
