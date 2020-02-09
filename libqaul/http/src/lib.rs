//! libqaul http server API
#![allow(unused)]

use tide::{self, Server, Request, Response};
use libqaul::Qaul;
use async_std::sync::Arc;

/// State structure for the libqaul http server
pub struct HttpServer {
    qaul: Arc<Qaul>,
    app: Server<()>,
        
}

impl HttpServer {
    pub fn new(qaul: Arc<Qaul>) -> Self {
        let mut app = tide::new();
        app.at("/").get(|mut r: Request<()>| async move {
            Response::new(200)
        });

        Self { qaul, app }
    }
}

