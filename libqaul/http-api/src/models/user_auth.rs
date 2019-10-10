use super::ConversionError;
use base64::{decode_config, encode_config, URL_SAFE};
use identity::ID_LEN;
use japi::{Attributes, ResourceObject};
use libqaul::Identity;
use serde_derive::{Deserialize, Serialize};

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

impl Attributes for UserAuth {
    fn kind() -> String {
        "user_auth".into()
    }
}

impl UserAuth {
    pub fn identity(obj: &ResourceObject<UserAuth>) -> Result<Identity, ConversionError> {
        let raw_id = decode_config(&obj.id, URL_SAFE)?;
        if raw_id.len() != ID_LEN {
            return Err(ConversionError::BadIdLength(raw_id.len()));
        }
        let id = Identity::truncate(&raw_id);
        Ok(id)
    }

    pub fn from_identity(
        id: Identity,
        secret: String,
        grant_type: GrantType,
    ) -> ResourceObject<UserAuth> {
        let id = encode_config(id.as_ref(), URL_SAFE);
        ResourceObject::new(id, Some(UserAuth { secret, grant_type }))
    }
}
