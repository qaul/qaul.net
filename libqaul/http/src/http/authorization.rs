//! # REST authorization middleware
//! 
//! Add it like this to the tide app:
//! ```
//! app_rest.middleware(authorization::Rest2Rpc::new());
//! ```

use futures::future::BoxFuture;

use tide::{
    middleware::{Middleware, Next},
    Request, Response,
};

#[derive(Clone, Default, Debug)]
pub struct Rest2Rpc {
    headers: u32,
}

impl Rest2Rpc {
    /// Construct a new instance with an empty list of headers.
    pub fn new() -> Rest2Rpc {
        Rest2Rpc::default()
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for Rest2Rpc {
    fn handle<'a>(&'a self, req: Request<State>, next: Next<'a, State>) -> BoxFuture<'a, Response> {
        Box::pin(async move {
            // Change the http request befor it is processed by the endpoint
            println!("Rest2Rpc Middleware: Incoming request from {} on url {}", req.method(), req.uri());

            // send the request to the endpoint
            let mut res = next.run(req).await;

            // Change the response of the endpoint before it is sent to the client
            println!("Rest2Rpc Middleware: Outgoing response with status {}", res.status());

            res
        })
    }
}
