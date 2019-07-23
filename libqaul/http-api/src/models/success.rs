use json_api::{ResourceObject, Attributes};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Success {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Attributes for Success { fn kind() -> String { "success".into() } }
