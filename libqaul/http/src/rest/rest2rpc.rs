//! # REST to RPC transformation
//!
//! TODOs:
//! * Error Handling: there should be no panic anymore
//!   * Return early with error when wrong input is delivered
//!

use crate::Responder;
use async_std::sync::Arc;
use libqaul_rpc::{
    json::{JsonAuth, JsonMap, RequestEnv},
    Envelope,
};
use mime::APPLICATION_JSON;
use serde_json::json;
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
    let auth: Option<JsonAuth> = match r.header("Authorization") {
        None => None,
        Some(s) => {
            if let Ok(json_auth) = serde_json::from_str(s) {
                Some(json_auth)
            } else {
                return Response::new(400)
                    .body_string(
                        "{\"Error\":\"Malformed json in authentication header\"}".to_string(),
                    )
                    .set_mime(APPLICATION_JSON);
            }
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

    // change data to diff if it is a PATCH request
    if method == "modify" {
        for (key, value) in data.iter_mut() {
            if key != "id" {
                *value = json!({ "set": value });
            }
        }
    }

    // get values from URI parameters
    if let Some(params) = uri_params {
        params.iter().for_each(|v| {
            if let Ok(x) = r.param(v) {
                data.insert(v.to_string(), serde_json::Value::String(x));
            } else {
                // return Response::new(400)
                //     .body_string("{\"Error\":\"URI parameter parsing error.\"}".to_string())
                //     .set_mime(APPLICATION_JSON);
            }
        });
    }

    // TODO: query parameters for query/list requests

    let rpc_req = RequestEnv {
        id: String::from(r.uri().path()),
        kind: kind.to_string(),
        method: method.to_string(),
        page: None,
        auth,
        data,
    };

    let Envelope { id: _, data: req } = match rpc_req.clone().generate_envelope() {
        Ok(env) => env,
        Err(e) => {
            // If there was an error parsing the envelope, return it
            return Response::new(500).body_string(e);
        }
    };

    // Call into libqaul via the rpc utilities
    let responder: Arc<_> = Arc::clone(r.state());
    let resp = responder.respond(req).await;

    // Return success or error code values
    let return_code = match &resp {
        libqaul_rpc::Response::Error(s) => {
            if s.starts_with("Not authorised") {
                401
            } else {
                400
            }
        }
        _ => 200,
    };

    Response::new(return_code).body_json(&resp).unwrap()
}
