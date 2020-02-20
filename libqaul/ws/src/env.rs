//! Json enveloping code

use libqaul::{users::UserAuth, Identity};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

pub(crate) type JsonMap = BTreeMap<String, JsonValue>;

/// A struct wrapper for UserAuth
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct JsonAuth {
    id: Identity,
    token: String,
}

impl From<JsonAuth> for UserAuth {
    fn from(ja: JsonAuth) -> Self {
        Self(ja.id, ja.token)
    }
}

/// A json specific request envelope
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RequestEnv {
    /// The request ID
    pub id: String,
    /// Auth data for the request
    pub auth: Option<JsonAuth>,
    /// An optional page selector
    pub page: Option<String>,
    /// Operation method
    pub method: String,
    /// Request scope
    pub kind: String,
    /// The rest of the nested data
    ///
    /// We keep this as a map because we need to inject the auth
    /// information into it later on, because the API expects it to be
    /// in eath RPC struct, while the json interface likes it in the
    /// envelope
    #[serde(default)]
    pub data: JsonMap,
}

/// A json specific repsonse envelope
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ResponseEnv {
    /// Response ID, same as request ID
    pub id: String,
    /// Mirrored auth token
    #[serde(skip)]
    pub auth: Option<JsonAuth>,
    /// Request method
    pub method: String,
    /// Request scope
    pub kind: String,
    /// Optional object count
    #[serde(skip)]
    pub total: Option<usize>,
    /// Optional pagination info
    #[serde(skip)]
    pub next: Option<String>,
    /// Response data
    pub data: JsonMap,
}
