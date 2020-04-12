//! libqaul http web server
//!
//! The web server serves the following things
//!
//! * the static files of the EmberJS webGUI
//! * the REST API for the webGUI
//! * the RPC API

use libqaul_rpc::Responder;

use async_std::{sync::Arc, task};

use tide::{self};
use tide_naive_static_files::StaticFilesEndpoint as StaticEp;

mod rest;
mod rpc;

/// State structure for the libqaul http server
pub struct HttpServer;

impl HttpServer {
    pub fn block(addr: &str, path: String, rpc: Responder) {
        let mut app = tide::new();
        let rpc_state = Arc::new(rpc);
        let rest_state = rpc_state.clone();

        // REST Endpoint
        app.at("/rest")
            .strip_prefix()
            .nest(rest::routes::rest_routes(rest_state));

        // RPC Endpoint
        app.at("/rpc")
            .strip_prefix()
            .nest(rpc::rpc_routes(rpc_state));

        // static file handler for the webui, assumes the webui exists
        app.at("/")
            //.strip_prefix()
            .get(StaticEp { root: path.into() });

        task::block_on(async move { app.listen(addr).await }).unwrap();
    }
}
