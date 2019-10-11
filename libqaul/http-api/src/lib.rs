use iron::{
    error::HttpResult,
    middleware::{BeforeMiddleware, Handler},
    mime,
    prelude::*,
    typemap, Listening,
};
use lazy_static::lazy_static;
use mount::Mount;

mod authenticator;
use authenticator::Authenticator;
pub use authenticator::CurrentUser;

pub mod core;

pub mod models;
pub mod error;
pub mod endpoints;

mod method;
pub use method::MethodGaurd;

mod jsonapi;
pub use jsonapi::{JsonApi, JsonApiGaurd};

lazy_static! { 
    /// A static `Mime` object representing `application/vnd.api+json`
    pub static ref JSONAPI_MIME : mime::Mime = mime::Mime(
        mime::TopLevel::Application,
        mime::SubLevel::Ext(String::from("vnd.api+json")),
        Vec::new());
}

/// The core of the qaul.net HTTP API
pub struct ApiServer {
    #[allow(unused)]
    authenticator: Authenticator,
    listening: Listening,
}

impl ApiServer {
    pub fn new<A: ToSocketAddrs>(qaul: &Qaul, addr: A) -> HttpResult<Self> {
        let mut mount = Mount::new();

        let mut login_chain = Chain::new(endpoints::login);
        login_chain.link_before(MethodGaurd::post());
        login_chain.link_before(JsonApiGaurd);
        mount.mount("/api/login", login_chain);

        let mut logout_chain = Chain::new(endpoints::logout);
        logout_chain.link_before(MethodGaurd::get());
        mount.mount("/api/logout", logout_chain);

        let mut chain = Chain::new(mount);
        chain.link_before(QaulCore::new(qaul)); 
        chain.link_before(jsonapi::JsonApi); 

        let authenticator = Authenticator::new();
        chain.link_before(authenticator.clone());

        let listening = Iron::new(chain).http(addr)?;

        Ok(Self{ 
            authenticator: authenticator.clone(), 
            listening, 
        })
    }

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
/// # use qaul_http::QaulCore;
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
    fn new(qaul: &Qaul) -> Self {
        Self {
            qaul: Arc::new(qaul.clone()),
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
