//! # REST to RPC transformation
//!
//! TODOs:
//! * Error Handling: there should be no panic anymore
//!   * Return early with error when wrong input is delivered
//!

use async_std::sync::Arc;
use libqaul_rpc::{
    json::{JsonAuth, JsonMap, RequestEnv, ResponseEnv},
    Envelope, Responder,
};
use serde_json;
use std::collections::BTreeMap;
use tide::{self, Request, Response};

/// Convert the REST call to RPC
pub async fn rest2rpc(r: Request<Arc<Responder>>, kind: &str, method: &str) -> Response {
    rest2rpc_params(r, kind, method, None).await
}

/// Convert the REST call to RPC with inserting the URI paramters into the data BTree
pub async fn rest2rpc_params(
    mut r: Request<Arc<Responder>>,
    kind: &str,
    method: &str,
    uri_params: Option<Vec<&str>>,
) -> Response {
    // get Authorization from header
    // TODO: Error handling when JSON is malformed
    let auth: Option<JsonAuth> = match r.header("Authorization") {
        None => None,
        Some(s) => {
            let json_auth: JsonAuth =
                serde_json::from_str(s).expect("Malformed json in authentication header");
            Some(json_auth)
        }
    };

    // get request body for certain methods
    // TODO: should return an HTTP error: when there is no body on POST or PATCH
    let mut data: JsonMap = match r.body_string().await {
        Ok(o) => match serde_json::from_str(o.as_str()) {
            Ok(o2) => o2,
            Err(_) => BTreeMap::new(),
        },
        Err(_) => BTreeMap::new(),
    };

    // get values from URI parameters
    if let Some(params) = uri_params {
        params.iter().for_each(|v| {
            if let Ok(x) = r.param(v) {
                data.insert(v.to_string(), serde_json::Value::String(x));
            } else {
                // TODO: return HTTP error
            }
        });
    }

    // TODO: query parameters for query/list requests

    let rpc_req = RequestEnv {
        id: String::from(r.uri().path()),
        kind: kind.to_string(),
        method: method.to_string(),
        page: None,
        auth: auth,
        data: data,
    };

    let Envelope { id, data: req } = rpc_req.clone().into();

    // Call into libqaul via the rpc utilities
    let responder: Arc<_> = Arc::clone(r.state());
    let resp = responder.respond(req).await;

    // Return success or error code values
    let return_code = match &resp {
        libqaul_rpc::Response::Error(s) => {
            if s.starts_with("Not authorised") {
                403
            } else {
                400
            }
        }
        _ => 200,
    };

    let env = Envelope { id, data: resp };

    // Build the reply envelope
    let resp_env: ResponseEnv = (env, rpc_req).into();

    Response::new(return_code).body_json(&resp_env).unwrap()
}
