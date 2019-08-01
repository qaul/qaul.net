use base64::{encode_config, decode_config, URL_SAFE};
use serde_derive::{Serialize, Deserialize};
use json_api::{ResourceObject, Attributes};
use super::ConversionError;
use libqaul::Identity;
use identity::ID_LEN;

/// The type of the requested grant
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GrantType {
    Token,
    Cookie,
}

/// A requested login grant
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UserAuth {
    pub secret: String,
    pub grant_type: GrantType,
}

impl Attributes for UserAuth { fn kind() -> String { "user_auth".into() } }

impl UserAuth {
    pub fn identity(obj: &ResourceObject<UserAuth>) -> 
    Result<Identity, ConversionError> {
        let raw_id = decode_config(&obj.id, URL_SAFE)?;
        if raw_id.len() != ID_LEN {
            return Err(ConversionError::BadIdLength(raw_id.len()));
        }
        let id = Identity::truncate(&raw_id);
        Ok(id)
    }
}
