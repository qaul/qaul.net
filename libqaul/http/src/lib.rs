//! libqaul http server API

use libqaul_rpc::{
    json::{RequestEnv, ResponseEnv},
    Envelope, Responder,
};

use async_std::{sync::Arc, task};
use serde_json;

use tide::{self, Request, Response};
use tide_naive_static_files::StaticFilesEndpoint as StaticEp;

/// State structure for the libqaul http server
pub struct HttpServer;

impl HttpServer {
    pub fn block(addr: &str, path: String, rpc: Responder) {
        let mut app = tide::with_state(Arc::new(rpc));
        app.at("/api").post(|mut r: Request<Arc<Responder>>| {
            async move {
                let hopefully_json: String = dbg!(r.body_string().await).unwrap();
                let req_env: RequestEnv =
                    serde_json::from_str(hopefully_json.as_str()).expect("Malformed json envelope");
                let Envelope { id, data: req } = req_env.clone().into();

                // Call into libqaul via the rpc utilities
                let responder: Arc<_> = Arc::clone(r.state());
                let resp = responder.respond(req).await;

                let env = Envelope {
                    id,
                    data: resp,
                };

                // Build the reply envelope
                let resp_env: ResponseEnv = (env, req_env).into();
                Response::new(200).body_json(&resp_env).unwrap()
            }
        });

        // static file handler for the webui, assumes the webui exists
        app.at("/")
            .strip_prefix()
            .get(StaticEp { root: path.into() });
        task::block_on(async move { app.listen(addr).await }).unwrap();
    }
}
