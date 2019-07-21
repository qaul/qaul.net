use base64::{encode_config, decode_config, URL_SAFE};
use serde_derive::{Serialize, Deserialize};
use json_api::{ResourceObject, Attributes};
use super::ConversionError;
use libqaul::Identity;
use identity::ID_LEN;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UserAuth {
    secret: Option<String>,
}

impl Attributes for UserAuth { fn kind() -> String { "user_auth".into() } }

impl UserAuth {
    pub fn from_identity(ident: Identity, secret: String) -> ResourceObject<UserAuth> {
        let id = encode_config(ident.as_ref(), URL_SAFE);
        let secret = Some(secret);
        ResourceObject::new(id, Some(UserAuth{ secret }))
    }

    pub fn into_identity(obj: ResourceObject<UserAuth>) -> 
    Result<(Identity, Option<String>), ConversionError> {
        let raw_id = decode_config(&obj.id, URL_SAFE)?;
        if raw_id.len() != ID_LEN {
            return Err(ConversionError::BadIdLength(raw_id.len()));
        }
        let id = Identity::truncate(&raw_id);
        let secret = obj.attributes.and_then(|a| a.secret);
        Ok((id, secret))
    }
}
