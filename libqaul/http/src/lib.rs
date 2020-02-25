//! libqaul http server API
#![allow(unused)]

use async_std::sync::Arc;
use libqaul::Qaul;
use tide::{self, Request, Response, Server};

/// State structure for the libqaul http server
pub struct HttpServer {
    qaul: Arc<Qaul>,
    app: Server<()>,
}

impl HttpServer {
    pub fn new(qaul: Arc<Qaul>) -> Self {
        let mut app = tide::new();
        app.at("/")
            .get(|mut r: Request<()>| async move { Response::new(200) });

        Self { qaul, app }
    }
}
