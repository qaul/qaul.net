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
        if let Some(bearer) = req.headers.get::<Authorization<Bearer>>() {
            if let Some(identity) = self.tokens.lock().unwrap().get(&bearer.token) {
                req.extensions.insert::<CurrentUser>(
                    UserAuth::Trusted(*identity, bearer.token.clone()));
            }
        }

        req.extensions.insert::<Authenticator>(self.clone());

        Ok(())
    }
}
