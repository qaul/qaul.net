use crate::{
    Cookies,
    models::GrantType,
    QaulCore,
};
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
/// # use iron::prelude::*;
/// # use qaul_http::CurrentUser;
/// fn handler(req: &mut Request) -> IronResult<Response> {
///     // Some(UserAuth) if an authenticated user is is requesting this endpoint
///     // None otherwise
///     let user = req.extensions.get::<CurrentUser>();
///
///     // ...
/// # Ok(Response::with(""))
/// # }
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

        // attempt to log in with the authorization header
        if let Some(bearer) = req.headers.get::<Authorization<Bearer>>() {
            match self.tokens.lock().unwrap().get(&bearer.token) {
                Some(identity) => {
                    req.extensions.insert::<CurrentUser>(
                        UserAuth::Trusted(*identity, bearer.token.clone()));
                },
                None => {
                    return Err(AuthError::InvalidToken(GrantType::Token).into());
                },
            }
        }

        // attempt to authenticate with the `bearer` cookie
        if let Some(cookie) = req.extensions.get::<Cookies>().unwrap().get("bearer") {
            match self.tokens.lock().unwrap().get(cookie.value()) {
                Some(identity) => {
                    let ua = UserAuth::Trusted(*identity, cookie.value().into());
                    if req.extensions.get::<CurrentUser>()
                            .map_or(false, |other_id| *other_id != ua) {
                        return Err(AuthError::DifferingLogins.into());
                    }
                    req.extensions.insert::<CurrentUser>(ua);
                },
                None => {
                    return Err(AuthError::InvalidToken(GrantType::Cookie).into());
                },
            }
        }

        Ok(())
    }
}
