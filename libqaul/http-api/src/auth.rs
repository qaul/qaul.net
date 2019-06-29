use iron::{
    BeforeMiddleware,
    prelude::*,
    headers::{Authorization, Bearer},
    typemap,
};
use libqaul::User;
use crate::QaulCore;
use persistent::Read;

pub struct Authenticator;
impl typemap::Key for Authenticator { type Value = Option<User>; }

impl BeforeMiddleware for Authenticator {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let user = match req.headers.get::<Authorization<Bearer>>() {
            Some(bearer) => {
                let token = bearer.token.clone(); // Otherwise rustc will yell
                req.get::<Read<QaulCore>>()
                .unwrap().authenticate(&token)?
            },
            None => None,
        };
        req.extensions.insert::<Authenticator>(user);

        Ok(())
    }
}
