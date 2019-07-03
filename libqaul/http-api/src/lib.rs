use libqaul::{
    Qaul,
    QaulResult,
    UserAuth, 
    Identity,
};
use iron::{
    error::HttpResult,
    Listening,
    typemap,
    prelude::*,
    status::Status,
    middleware::BeforeMiddleware,
};
use std::{
    net::ToSocketAddrs,
    sync::Arc,
};

mod auth;
use auth::Authenticator;

// stand in for a real handler 
// coming soon to a pull request near you
fn not_really_a_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(Status::ImATeapot))
}

pub struct ApiServer {
    authenticator: Authenticator,
    listening: Listening,
}

impl ApiServer {
    pub fn new<A: ToSocketAddrs>(qaul: Arc<Qaul>, addr: A) -> HttpResult<Self> {
        let authenticator = Authenticator::new();

        let mut chain = Chain::new(not_really_a_handler);
        chain.link_before(QaulCore::new(qaul));
        chain.link_before(authenticator.clone());

        let listening = Iron::new(chain).http(addr)?;

        Ok(Self{ 
            authenticator: authenticator.clone(), 
            listening 
        })
    }

    /// According to https://github.com/hyperium/hyper/issues/338 this _probably_
    /// does nothing, but i'm providing it in the hope that in the future
    /// some one will figure out how to shutdown a webserver without crashing it
    pub fn close(&mut self) -> HttpResult<()> {
        self.listening.close()
    }
}

struct QaulCore{
    qaul: Arc<Qaul>,
}

impl QaulCore {
    pub fn new(qaul: Arc<Qaul>) -> Self {
        Self{ qaul }
    }
}

impl typemap::Key for QaulCore { type Value = Arc<Qaul>; }

impl BeforeMiddleware for QaulCore {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Self>(self.qaul.clone());
        Ok(())
    }
}

