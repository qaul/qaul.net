use super::{from_identity, into_identity};
use crate::error::{ApiError, DocumentError, GenericError};
use base64::{decode_config, encode_config, URL_SAFE};
use japi::{
    Attributes, Identifier, Link, Links, OptionalVec, Relationship, Relationships, ResourceObject,
};
use libqaul::{users::UserProfile, Identity};
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    convert::TryFrom,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<BTreeSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Attributes for User {
    fn kind() -> String {
        "user".into()
    }
}

impl User {
    pub fn from_service_user(user: UserProfile) -> ResourceObject<User> {
        let id = from_identity(&user.id);
        let mut ro = ResourceObject::new(id.clone(), None);

        let mut relationships = Relationships::new();
        relationships.insert(
            "secret".into(),
            Relationship {
                data: OptionalVec::One(Some(Identifier::new(id.clone(), "secret".into()))),
                ..Default::default()
            },
        );
        ro.relationships = Some(relationships);

        let mut links = Links::new();
        links.insert("self".into(), Link::Url(format!("/api/users/{}", id)));
        ro.links = Some(links);

        ro
    }

    pub fn from_service_user_with_data(service_user: UserProfile) -> ResourceObject<User> {
        let avatar = service_user
            .avatar
            .as_ref()
            .map(|a| encode_config(a, URL_SAFE));
        let user = User {
            display_name: service_user.display_name.clone(),
            real_name: service_user.real_name.clone(),
            bio: Some(service_user.bio.clone()),
            services: Some(service_user.services.clone()),
            avatar,
        };
        let mut ro = Self::from_service_user(service_user);
        ro.attributes = Some(user);
        ro
    }

    pub fn identity(ro: &ResourceObject<User>, pointer: &str) -> Result<Identity, ApiError> {
        ro.id
            .as_ref()
            .ok_or(
                DocumentError::NoId {
                    pointer: Some(format!("{}/id", pointer)),
                }
                .into(),
            )
            .and_then(|id| Ok(into_identity(&id)?))
    }

    pub fn avatar(
        ro: &ResourceObject<User>,
        pointer: &str,
    ) -> Result<Option<Vec<u8>>, GenericError> {
        ro.attributes
            .as_ref()
            .and_then(|d| d.avatar.as_ref())
            .map(|a| {
                decode_config(a, URL_SAFE).map_err(|e| {
                    GenericError::new("Invalid Avatar".into())
                        .detail(format!("{}", e))
                        .pointer(format!("{}/attributes/avatar", pointer))
                })
            })
            .transpose()
    }

    pub fn secret_relationship(
        ro: &ResourceObject<User>,
        pointer: &str,
    ) -> Result<Identity, ApiError> {
        ro.relationships
            .as_ref()
            .ok_or(
                DocumentError::NoRelationships {
                    pointer: Some(format!("{}/relationships", pointer)),
                }
                .into(),
            )
            .and_then(|r| {
                r.get("secret").ok_or(
                    DocumentError::NoRelationship {
                        rel: "secret".into(),
                        pointer: Some(format!("{}/relationships/secret", pointer)),
                    }
                    .into(),
                )
            })
            .and_then(|r| match &r.data {
                OptionalVec::One(Some(r)) => Ok(r),
                OptionalVec::Many(_) => Err(DocumentError::MultipleData.into()),
                _ => Err(DocumentError::NoData.into()),
            })
            .and_then(|id| {
                ResourceObject::<User>::try_from(id).map_err(|e| {
                    DocumentError::ConversionError {
                        err: e,
                        pointer: Some(format!("{}/relationships/secret", pointer)),
                    }
                    .into()
                })
            })
            .and_then(|id| {
                id.id.ok_or(DocumentError::NoId {
                    pointer: Some(format!("{}/relationships/secret/id", pointer)),
                })
            })
            .map_err(|e| e.into())
            .and_then(|id| Ok(into_identity(&id)?))
    }
}
