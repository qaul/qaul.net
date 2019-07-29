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
use super::AuthError;

/// Use this key to get the logged in user of the request
///
/// ```
/// fn handler(req: &mut Request) -> IronResult<Response> {
///     // Some(UserAuth) if an authenticated user is is requesting this endpoint
///     // None otherwise
///     let user = req.extensions.get::<CurrentUser>();
/// }
/// ```
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
        req.extensions.insert::<Authenticator>(self.clone());

        if let Some(bearer) = req.headers.get::<Authorization<Bearer>>() {
            match self.tokens.lock().unwrap().get(&bearer.token) {
                Some(identity) => {
                    req.extensions.insert::<CurrentUser>(
                        UserAuth::Trusted(*identity, bearer.token.clone()));
                },
                None => {
                    return Err(AuthError::InvalidToken.into());
                },
            }
        }

        Ok(())
    }
}
