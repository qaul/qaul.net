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
    middleware::{
        BeforeMiddleware,
        Handler,
    },
    mime,
};
use std::{
    net::ToSocketAddrs,
    sync::Arc,
};
use lazy_static::lazy_static;

mod auth;
use auth::Authenticator;
pub use auth::CurrentUser;

pub mod models;

mod mount;
pub use mount::HotPlugError;

mod method;
pub use method::MethodGaurd;

mod jsonapi;
pub use jsonapi::{JsonApi, JsonApiGaurd};

mod cookie;
pub use crate::cookie::Cookies;

lazy_static! { 
    /// A static `Mime` object representing `application/vnd.api+json`
    pub static ref JSONAPI_MIME : mime::Mime = mime::Mime(
        mime::TopLevel::Application,
        mime::SubLevel::Ext(String::from("vnd.api+json")),
        Vec::new()); 
}

/// The core of the qaul.net HTTP API
pub struct ApiServer {
    authenticator: Authenticator,
    mount: mount::HotPlugMount,
    listening: Listening,
}

impl ApiServer {
    pub fn new<A: ToSocketAddrs>(qaul: &Qaul, addr: A) -> HttpResult<Self> {
        let mount = mount::HotPlugMount::new();

        let mut login_chain = Chain::new(auth::login);
        login_chain.link_before(MethodGaurd::post());
        login_chain.link_before(JsonApiGaurd);
        mount.mount_core("login".into(), login_chain);

        let mut logout_chain = Chain::new(auth::logout);
        logout_chain.link_before(MethodGaurd::get());
        mount.mount_core("logout".into(), logout_chain);

        let mut chain = Chain::new(mount.clone());
        chain.link(crate::cookie::CookieManager::new());
        chain.link_before(QaulCore::new(qaul)); 
        chain.link_before(jsonapi::JsonApi); 

        let authenticator = Authenticator::new();
        chain.link_before(authenticator.clone()); 

        let listening = Iron::new(chain).http(addr)?;

        Ok(Self{ 
            authenticator: authenticator.clone(), 
            mount,
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

    /// Mount a service's handler under `/{name}`
    ///
    /// Errors when you try to replace a core route like `/login`
    ///
    /// Returns `true` when a this service replaces a previous service mounted
    /// under the same path and `false` otherwise
    pub fn mount_service<T: Handler>(&self, name: String, handler: T) -> Result<bool, HotPlugError> {
        self.mount.mount(name, handler)
    }

    /// Unmount a service's handler
    ///
    /// Errors when you try to unmount a core route like `/login`
    ///
    /// Returns `true` when a service with that name existed and was unmounted, 
    /// `false` when no service of that name was found 
    pub fn unmount_service(&self, name: &str) -> Result<bool, HotPlugError> {
        self.mount.unmount(name)
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
pub struct QaulCore{
    qaul: Arc<Qaul>,
}

impl QaulCore {
    fn new(qaul: &Qaul) -> Self {
        Self{ qaul: Arc::new(qaul.clone()) }
    }
}

impl typemap::Key for QaulCore { type Value = Arc<Qaul>; }

impl BeforeMiddleware for QaulCore {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Self>(self.qaul.clone());
        Ok(())
    }
}

