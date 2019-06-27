use iron::{
    BeforeMiddleware,
    prelude::*,
    headers::{Authorization, Bearer},
    typemap,
};
use common::{
    identity::UserID,
};
use libqaul;
use persistent::Write;

struct CoreAuthenticator;
impl typemap::Key for CoreAuthenticator { type Value = libqaul::Authenticator; }

struct Authenticator;
impl typemap::Key for Authenticator { type Value = Option<UserID>; }

impl BeforeMiddleware for Authenticator {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let user_id = match req.headers.get::<Authorization<Bearer>>() {
            Some(bearer) => {
                let token = bearer.token.clone(); // Otherwise rustc will yell
                req.get::<Write<CoreAuthenticator>>()
                .unwrap()
                .lock().unwrap()
                .authenticate(&token)?
            },
            None => None,
        };
        req.extensions.insert::<Authenticator>(user_id);

        Ok(())
    }
}
