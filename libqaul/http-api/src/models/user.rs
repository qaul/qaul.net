use base64::{encode_config, decode_config, URL_SAFE};
use libqaul::{
    ApiUser,
    Identity,
};
use json_api::{ResourceObject, Attributes};
use serde_derive::{Serialize, Deserialize};
use std::collections::BTreeMap;
use identity::ID_LEN;
use super::ConversionError;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    real_name: Option<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    bio: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
}

impl Attributes for User {
    fn kind() -> String { "user".into() }
}

impl User {
    pub fn from_service_user(user: ApiUser) -> ResourceObject<User> {
        let id = encode_config(user.id.as_ref(), URL_SAFE);
        let avatar = user.avatar.map(|a| encode_config(&a, URL_SAFE));
        let user = User {
            display_name: user.display_name,
            real_name: user.real_name,
            bio: user.bio,
            services: Some(user.services),
            avatar
        };
        ResourceObject::new(id, Some(user))
    }

    pub fn into_service_user(user: ResourceObject<User>) -> Result<ApiUser, ConversionError> {
        let raw_id = decode_config(&user.id, URL_SAFE)?;
        if raw_id.len() != ID_LEN {
            return Err(ConversionError::BadIdLength(raw_id.len()));
        }
        let id = Identity::truncate(&raw_id);

        Ok(match user.attributes {
            Some(user) => {
                let avatar = if let Some(a) = user.avatar {
                    Some(decode_config(&a, URL_SAFE)?)
                } else { None };
                let services = user.services.unwrap_or_default();
                ApiUser {
                    id,
                    display_name: user.display_name,
                    real_name: user.real_name,
                    bio: user.bio,
                    services,
                    avatar,
                }
            },
            None => {
                ApiUser {
                    id,
                    display_name: None,
                    real_name: None,
                    bio: BTreeMap::new(),
                    services: Vec::new(),
                    avatar: None,
                }
            },
        })
    }
}
