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
    mime,
};
use std::{
    net::ToSocketAddrs,
    sync::Arc,
};
use lazy_static::lazy_static;
use router::Router;

mod auth;
use auth::Authenticator;
pub use auth::CurrentUser;

pub mod models;

mod jsonapi;
pub use jsonapi::{JsonApi, JsonApiGaurd};

lazy_static! { pub static ref JSONAPI_MIME : mime::Mime = mime::Mime(
        mime::TopLevel::Application,
        mime::SubLevel::Ext(String::from("vnd.api+json")),
        Vec::new()); 
}

fn core_route_blackhole(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(Status::MethodNotAllowed))
}

pub struct ApiServer {
    authenticator: Authenticator,
    listening: Listening,
}

impl ApiServer {
    pub fn new<A: ToSocketAddrs>(qaul: &Qaul, addr: A) -> HttpResult<Self> {
        let mut router = Router::new();

        let mut login_chain = Chain::new(auth::login);
        login_chain.link_before(JsonApiGaurd);
        router.post("/login", login_chain, "login_post");
        router.any("/login", core_route_blackhole, "login");

        router.get("/logout", auth::logout, "logout_get");
        router.any("/logout", core_route_blackhole, "logout");

        let mut chain = Chain::new(router);
        chain.link_before(QaulCore::new(qaul)); 
        chain.link_before(jsonapi::JsonApi); 

        let authenticator = Authenticator::new();
        chain.link_before(authenticator.clone()); 

        let listening = Iron::new(chain).http(addr)?;

        Ok(Self{ 
            authenticator: authenticator.clone(), 
            listening 
        })
    }

    /// According to https://github.com/hyperium/hyper/issues/338 this _probably_
    /// does nothing, but i'm providing it in the hope that in the future
    /// someone will figure out how to shutdown a webserver without crashing it
    pub fn close(&mut self) -> HttpResult<()> {
        self.listening.close()
    }
}

struct QaulCore{
    qaul: Arc<Qaul>,
}

impl QaulCore {
    pub fn new(qaul: &Qaul) -> Self {
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

