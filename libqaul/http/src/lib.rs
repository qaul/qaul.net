use iron::{
    error::HttpResult,
    middleware::{BeforeMiddleware, Handler},
    mime,
    prelude::*,
    typemap, Listening,
};
use lazy_static::lazy_static;
use libqaul::Qaul;
use router::Router;
use std::{net::ToSocketAddrs, sync::Arc};
#[cfg(feature = "messaging")]
use text_messaging::Messaging;

mod authenticator;
use authenticator::Authenticator;
pub use authenticator::CurrentUser;

pub mod endpoints;
pub mod error;
pub mod models;

mod method;
pub use method::MethodGaurd;

mod jsonapi;
pub use jsonapi::{JsonApi, JsonApiGaurd};

#[cfg(feature = "messaging")]
mod qaul_messaging;
#[cfg(feature = "messaging")]
pub use qaul_messaging::QaulMessaging;

#[cfg(test)]
pub mod test_utils;

lazy_static! {
    /// A static `Mime` object representing `application/vnd.api+json`
    pub static ref JSONAPI_MIME : mime::Mime = mime::Mime(
        mime::TopLevel::Application,
        mime::SubLevel::Ext(String::from("vnd.api+json")),
        Vec::new());
}

pub struct ServerBuilder {
    chain: Chain,
}

impl ServerBuilder {
    pub fn new(qaul: Arc<Qaul>) -> Self {
        let mut router = Router::new();
        endpoints::route(&mut router);

        let mut chain = Chain::new(router);
        chain.link_before(QaulCore::new(qaul));
        chain.link_before(JsonApi);

        ServerBuilder { chain }
    }

    #[cfg(feature = "messaging")]
    pub fn messaging(mut self, messaging: &Messaging) -> Self {
        self.chain.link_before(QaulMessaging::new(messaging));
        self
    }

    pub fn start<A: ToSocketAddrs>(mut self, addr: A) -> HttpResult<ApiServer> {
        let authenticator = Authenticator::new();
        self.chain.link_before(authenticator.clone());

        let listening = Iron::new(self.chain).http(addr)?;

        Ok(ApiServer {
            authenticator,
            listening,
        })
    }
}

/// The core of the qaul.net HTTP API
pub struct ApiServer {
    authenticator: Authenticator,
    listening: Listening,
}

impl ApiServer {
    /// According to
    /// [https://github.com/hyperium/hyper/issues/338](https://github.com/hyperium/hyper/issues/338)
    /// this _probably_ does nothing, but i'm providing it in the hope that in the
    /// future someone will figure out how to shutdown a webserver without crashing it
    pub fn close(&mut self) -> HttpResult<()> {
        self.listening.close()
    }
}

/// Use this key to get a `Qaul` instance from the `Request` object
///
/// ```
/// # use iron::prelude::*;
/// # use libqaul_http::QaulCore;
/// fn handler(req: &mut Request) -> IronResult<Response> {
///     let qaul = req.extensions.get::<QaulCore>().unwrap();
///     
///     // ...
/// # Ok(Response::with(""))
/// # }
/// ```
pub struct QaulCore {
    qaul: Arc<Qaul>,
}

impl QaulCore {
    fn new(qaul: Arc<Qaul>) -> Self {
        Self {
            qaul: qaul.clone(),
        }
    }
}

impl typemap::Key for QaulCore {
    type Value = Arc<Qaul>;
}

impl BeforeMiddleware for QaulCore {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Self>(self.qaul.clone());
        Ok(())
    }
}
