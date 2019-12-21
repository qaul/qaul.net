use crate::{error::GenericError, QaulCore};
use iron::{
    headers::{Authorization, Bearer},
    prelude::*,
    typemap, BeforeMiddleware,
};
use libqaul::{users::UserAuth, Identity};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Use this key to get the logged in user of the request
///
/// ```
/// # use iron::prelude::*;
/// # use libqaul_http::CurrentUser;
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

impl typemap::Key for CurrentUser {
    type Value = UserAuth;
}

#[derive(Clone)]
pub(crate) struct Authenticator {
    pub tokens: Arc<Mutex<HashMap<String, Identity>>>,
}

impl Authenticator {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl typemap::Key for Authenticator {
    type Value = Self;
}

impl BeforeMiddleware for Authenticator {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Authenticator>(self.clone());

        // attempt to log in with the authorization header
        if let Some(bearer) = req.headers.get::<Authorization<Bearer>>() {
            match self.tokens.lock().unwrap().get(&bearer.token) {
                Some(identity) => {
                    req.extensions
                        .insert::<CurrentUser>(UserAuth(*identity, bearer.token.clone()));
                }
                None => {
                    return Err(GenericError::new("Invalid Login Token".into())
                        .detail("The authorization header contains a token that is either no longer valid or never was valid".into())
                        .into());
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anneal::RequestBuilder;
    use iron::method::Method;
    use libqaul::Qaul;

    fn setup() -> (RequestBuilder, Authenticator, UserAuth, String) {
        let qaul = Qaul::dummy();
        let user_auth = qaul.users().create("a".into()).unwrap();
        let UserAuth(ident, key) = user_auth.clone();

        let authenticator = Authenticator::new();
        {
            authenticator
                .tokens
                .lock()
                .unwrap()
                .insert(key.clone(), ident);
        }

        let mut rb = RequestBuilder::default();
        rb.add_middleware(authenticator.clone());

        (rb, authenticator, user_auth, key)
    }

    #[test]
    fn no_login() {
        let (rb, _, _, _) = setup();
        rb.request(|req| {
            assert_eq!(req.extensions.get::<CurrentUser>(), None);
        });
    }

    #[test]
    fn valid_token_login() {
        let (mut rb, _, user_auth, key) = setup();
        rb.set_header(Authorization(Bearer { token: key }))
            .request(|req| {
                assert_eq!(req.extensions.get::<CurrentUser>(), Some(&user_auth));
            });
    }

    #[test]
    fn invalid_token_login() {
        let (mut rb, authenticator, user_auth, _) = setup();
        rb.set_header(Authorization(Bearer {
            token: "i am not valid".into(),
        }))
        .set_chain(vec![])
        .request(|mut req| {
            assert!(authenticator.before(&mut req).is_err());
        });
    }
}
