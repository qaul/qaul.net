use libqaul::Identity;
use json_api::{ResourceObject, Attributes};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserGrant {
    pub token: String,
}

impl Attributes for UserGrant { fn kind() -> String { "user_grant".into() } }

