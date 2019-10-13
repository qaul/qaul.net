use base64::{encode_config, decode_config, URL_SAFE};
use crate::error::GenericError;
use serde_derive::{Serialize, Deserialize};
use japi::{ResourceObject, Attributes};
use libqaul::Identity;
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
    pub fn identity(obj: &ResourceObject<UserAuth>) -> 
    Result<Identity, GenericError> {
        let raw_id = decode_config(&obj.id, URL_SAFE).map_err(|e| {
            GenericError::new("Invalid Identity".into())
                .detail(format!("Failed to decode identity, base 64 invalid: {}", e))
                .pointer("/data/id".into())
        })?;
        if raw_id.len() != ID_LEN {
            return Err(GenericError::new("Invalid Identity".into())
                .detail(format!("Decoded identity is {} bytes long when it should be {}", 
                    raw_id.len(), ID_LEN))
                .pointer("/data/id".into()));
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
