use libqaul::{
    Qaul,
    QaulResult,
};
use iron::{
    Listening,
    typemap,
    prelude::*,
    status::Status,
};
use persistent::Read;
use std::net::ToSocketAddrs;

mod auth;

struct QaulCore;
impl typemap::Key for QaulCore { type Value = Qaul; }

// stand in for a real handler 
// coming soon to a pull request near you
fn not_really_a_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(Status::ImATeapot))
}

pub struct ApiServer {
    listening: Listening,
}

impl ApiServer {
    pub fn new<A: ToSocketAddrs>(qaul: Qaul, addr: A) -> QaulResult<ApiServer> {
        let mut chain = Chain::new(not_really_a_handler);
        // TODO: write middleware so this isn't in a read
        chain.link(Read::<QaulCore>::both(qaul));
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
