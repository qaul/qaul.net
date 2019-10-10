use chrono::offset::Utc;
use japi::{Attributes, ResourceObject};
use serde_derive::{Deserialize, Serialize};

/// Returned by endpoints that have successfully completed their task yet have
/// no actual data to return
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Success {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Attributes for Success {
    fn kind() -> String {
        "success".into()
    }
}

impl Success {
    /// Will create a success message with the given message
    /// as an attribute and the current time in milliseconds
    /// as a (hopefully) unique id
    pub fn from_message(message: String) -> ResourceObject<Self> {
        let id = format!("{}", Utc::now().timestamp_millis());
        let attr = Some(Self {
            message: Some(message),
        });
        ResourceObject::new(id, attr)
    }
}
