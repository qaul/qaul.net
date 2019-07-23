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

#[derive(Clone)]
pub struct Authenticator{
    tokens: Arc<Mutex<HashMap<String, Identity>>>,
}

impl Authenticator {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl typemap::Key for Authenticator { type Value = UserAuth; }

impl BeforeMiddleware for Authenticator {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        if let Some(bearer) = req.headers.get::<Authorization<Bearer>>() {
            if let Some(identity) = self.tokens.lock().unwrap().get(&bearer.token) {
                req.extensions.insert::<Self>(UserAuth::Trusted(*identity, bearer.token));
            }
        }

        Ok(())
    }
}
