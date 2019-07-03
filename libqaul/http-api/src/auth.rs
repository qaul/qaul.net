use crate::QaulCore;
use iron::{
    BeforeMiddleware,
    prelude::*,
    headers::{Authorization, Bearer},
    typemap,
};
use libqaul::{
    Identity
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

impl typemap::Key for Authenticator { type Value = Option<Identity>; }

impl BeforeMiddleware for Authenticator {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let identity = req.headers.get::<Authorization<Bearer>>()
            .and_then(|bearer| self.tokens.lock().unwrap()
                      .get(&bearer.token)
                      .map(|identity| *identity));
        req.extensions.insert::<Self>(identity);

        Ok(())
    }
}
