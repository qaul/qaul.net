use super::from_identity;
use crate::error::ApiError;
use japi::{
    Attributes, Identifier, Link, Links, OptionalVec, Relationship, Relationships, ResourceObject,
};
use libqaul::users::UserAuth;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct Grant {
    pub secret: String,
}

impl Attributes for Grant {
    fn kind() -> String {
        "grant".into()
    }
}

impl Grant {
    pub fn from_user_auth(ua: UserAuth) -> Result<ResourceObject<Grant>, ApiError> {
        let UserAuth(id, grant) = ua;
        let mut g = ResourceObject::<Grant>::new(grant.clone(), None);

        let mut relationships = Relationships::new();
        relationships.insert(
            "user".into(),
            Relationship {
                data: OptionalVec::One(Some(Identifier::new(from_identity(&id), "user".into()))),
                ..Default::default()
            },
        );
        g.relationships = Some(relationships);

        let mut links = Links::new();
        links.insert("self".into(), Link::Url(format!("/api/grants/{}", grant)));
        g.links = Some(links);

        Ok(g)
    }
}
