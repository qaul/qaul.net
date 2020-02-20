//! Json enveloping code

use libqaul::{users::UserAuth, Identity};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

pub(crate) type JsonMap = BTreeMap<String, JsonValue>;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JsonEnvelope {
    /// The request ID
    pub id: String,
    /// Auth data for the request
    pub auth: Option<JsonAuth>,
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

/// A struct wrapper for UserAuth
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JsonAuth {
    id: Identity,
    token: String,
}

impl From<JsonAuth> for UserAuth {
    fn from(ja: JsonAuth) -> Self {
        Self(ja.id, ja.token)
    }
}
