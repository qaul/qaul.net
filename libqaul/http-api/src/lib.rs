use libqaul::{
    Qaul,
    QaulResult,
};
use iron::{
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

// stand in for a real handler 
// coming soon to a pull request near you
fn not_really_a_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(Status::ImATeapot))
}

pub struct ApiServer {
    listening: Listening,
}

impl ApiServer {
    pub fn new<A: ToSocketAddrs>(qaul: Arc<Qaul>, addr: A) -> QaulResult<ApiServer> {
        let mut chain = Chain::new(not_really_a_handler);
        chain.link_before(QaulCore::new(qaul));
        chain.link_before(auth::Authenticator);

        let listening = Iron::new(chain).http(addr)?;

        Ok(ApiServer{ listening })
    }

    /// According to https://github.com/hyperium/hyper/issues/338 this _probably_
    /// does nothing, but i'm providing it in the hope that in the future
    /// some one will figure out how to shutdown a webserver without crashing it
    pub fn close(&mut self) -> QaulResult<()> {
        Ok(self.listening.close()?)
    }
}

struct QaulCore{
    qaul: Arc<Qaul>,
}

impl QaulCore {
    pub fn new(qaul: Arc<Qaul>) -> QaulCore {
        QaulCore{
            qaul,
        }
    }
}

impl typemap::Key for QaulCore { type Value = Arc<Qaul>; }

impl BeforeMiddleware for QaulCore {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<QaulCore>(self.qaul.clone());
        Ok(())
    }
}
