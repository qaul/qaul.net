use super::{from_identity, into_identity};
use crate::error::{ApiError, DocumentError};
use japi::{
    Attributes, Identifier, Link, Links, OptionalVec, Relationship, Relationships, ResourceObject,
};
use libqaul::Identity;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct Secret {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
}

impl Attributes for Secret {
    fn kind() -> String {
        "secret".into()
    }
}

impl Secret {
    pub fn from_identity(id: &Identity) -> ResourceObject<Secret> {
        let id = from_identity(id);
        let mut ro = ResourceObject::<Secret>::new(id.clone(), None);

        let mut links = Links::new();
        links.insert("self".into(), Link::Url(format!("/api/secrets/{}", id)));
        ro.links = Some(links);

        let mut relationships = Relationships::new();
        relationships.insert(
            "user".into(),
            Relationship {
                data: OptionalVec::One(Some(Identifier::new(id, "user".into()))),
                ..Default::default()
            },
        );
        ro.relationships = Some(relationships);

        ro
    }

    pub fn into_identity(
        ro: &ResourceObject<Secret>,
        pointer: String,
    ) -> Result<Identity, ApiError> {
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
}
