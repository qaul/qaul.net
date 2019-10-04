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

#[cfg(test)]
mod test {
    use anneal::RequestBuilder;
    use crate::cookie::CookieManager;
    use cookie::{CookieJar, Cookie};
    use iron::method::Method;
    use super::*;
    use libqaul::{
        Qaul,
        UserUpdate
    };

    fn setup() -> (RequestBuilder, Authenticator, UserAuth, String) {
        let qaul = Qaul::start();
        let user_auth = qaul.user_create("a".into()).unwrap();
        let (ident, key) = qaul.user_authenticate(user_auth.clone()).unwrap();

        let authenticator = Authenticator::new();
        { authenticator.tokens.lock().unwrap().insert(key.clone(), ident); } 

        let mut rb = RequestBuilder::default();
        rb.add_middleware(CookieManager::new().0);
        rb.add_middleware(authenticator.clone());

        (rb, authenticator, user_auth, key)
    }

    #[test]
    fn no_login() {
        let (rb, _, _, _) = setup();
        rb.request(|mut req| {
                assert_eq!(req.extensions.get::<CurrentUser>(), None);
            });
    }

    #[test]
    fn valid_token_login() {
        let (mut rb, _, user_auth, key) = setup();
        rb.set_header(Authorization(Bearer { token: key }))
            .request(|mut req| {
                assert_eq!(req.extensions.get::<CurrentUser>(), Some(&user_auth));
            });
    }

    #[test]
    fn invalid_token_login() {
        let (mut rb, authenticator, user_auth, _) = setup();
        rb.set_header(Authorization(Bearer { token: "i am not valid".into() }))
            .set_chain(vec![Box::new(CookieManager::new().0)])
            .request(|mut req| {
                assert!(authenticator.before(&mut req).is_err());
            });
    }

    #[test]
    fn valid_login_cookie() {
        let (mut rb, _, user_auth, key) = setup();
        let mut jar = CookieJar::new();
        jar.add(Cookie::new("bearer", key));
        rb.set_cookies(&jar)
            .request(|mut req| {
                assert_eq!(req.extensions.get::<CurrentUser>(), Some(&user_auth));
            });
    }

    #[test]
    fn invalid_login_cookie() {
        let (mut rb, authenticator, user_auth, key) = setup();
        let mut jar = CookieJar::new();
        jar.add(Cookie::new("bearer", "i'm not the right key"));
        rb.set_cookies(&jar)
            .set_chain(vec![Box::new(CookieManager::new().0)])
            .request(|mut req| {
                assert!(authenticator.before(&mut req).is_err());
            });
    }

    // this test ensures that if you log in as two seperate users you'll fail to authenticate
    #[test]
    fn two_rights_dont_make_a_left() {
        let qaul = Qaul::start();
        let user_auth = qaul.user_create("a".into()).unwrap();
        let (ident, key) = qaul.user_authenticate(user_auth.clone()).unwrap();
        let user_auth2 = qaul.user_create("b".into()).unwrap();
        let (ident2, key2) = qaul.user_authenticate(user_auth2.clone()).unwrap();

        let authenticator = Authenticator::new();
        { 
            let mut tokens = authenticator.tokens.lock().unwrap();
            tokens.insert(key.clone(), ident); 
            tokens.insert(key2.clone(), ident2); 
        }

        let mut jar = CookieJar::new();
        jar.add(Cookie::new("bearer", key2));
        RequestBuilder::default()
            .set_header(Authorization(Bearer { token: key }))
            .set_cookies(&jar)
            .add_middleware(CookieManager::new().0)
            .request(|mut req| {
                assert!(authenticator.before(&mut req).is_err());
            });
    }

    // this test ensures that if you user both a cookie and a bearer token to log in as the same
    // user everything works
    #[test]
    fn unless_theyre_180_degrees() {
        let qaul = Qaul::start();
        let user_auth = qaul.user_create("a".into()).unwrap();
        let (ident, key) = qaul.user_authenticate(user_auth.clone()).unwrap();

        let authenticator = Authenticator::new();
        { 
            let mut tokens = authenticator.tokens.lock().unwrap();
            tokens.insert(key.clone(), ident); 
        }

        let mut jar = CookieJar::new();
        jar.add(Cookie::new("bearer", key.clone()));
        RequestBuilder::default()
            .set_header(Authorization(Bearer { token: key }))
            .set_cookies(&jar)
            .add_middleware(CookieManager::new().0)
            .add_middleware(authenticator)
            .request(|mut req| {
                assert_eq!(req.extensions.get::<CurrentUser>(), Some(&user_auth));
            });
    }
}
