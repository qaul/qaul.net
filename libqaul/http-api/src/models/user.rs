use super::ConversionError;
use base64::{decode_config, encode_config, URL_SAFE};
use identity::ID_LEN;
use json_api::{Attributes, ResourceObject};
use libqaul::{Identity, User, UserData};
use serde_derive::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// An entity dual for `libqaul::User`
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    real_name: Option<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    bio: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<BTreeSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
}

impl Attributes for UserEntity {
    fn kind() -> String {
        "user".into()
    }
}

impl UserEntity {
    pub fn from_service_user(user: User) -> ResourceObject<UserEntity> {
        let id = encode_config(user.id.as_ref(), URL_SAFE);
        let avatar = user.data.avatar.map(|a| encode_config(&a, URL_SAFE));
        let user = UserEntity {
            display_name: user.data.display_name,
            real_name: user.data.real_name,
            bio: user.data.bio,
            services: Some(user.data.services),
            avatar,
        };
        ResourceObject::new(id, Some(user))
    }

    pub fn into_service_user(user: ResourceObject<UserEntity>) -> Result<User, ConversionError> {
        let raw_id = decode_config(&user.id, URL_SAFE)?;
        if raw_id.len() != ID_LEN {
            return Err(ConversionError::BadIdLength(raw_id.len()));
        }
        let id = Identity::truncate(&raw_id);

        Ok(match user.attributes {
            Some(user) => {
                let avatar = if let Some(a) = user.avatar {
                    Some(decode_config(&a, URL_SAFE)?)
                } else {
                    None
                };
                let services = user.services.unwrap_or_default();
                User {
                    id,
                    data: UserData {
                        display_name: user.display_name,
                        real_name: user.real_name,
                        bio: user.bio,
                        services,
                        avatar,
                    },
                }
            }
            None => User {
                id,
                data: Default::default(),
            },
        })
    }
}
