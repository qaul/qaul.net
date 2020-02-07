use crate::{
    error::{ApiError, DocumentError, QaulError},
    models::{into_identity, Grant, User},
    Authenticator, JsonApi, QaulCore, JSONAPI_MIME,
};
use iron::{prelude::*, status::Status};
use japi::{Document, OptionalVec, ResourceObject};
use libqaul::users::UserAuth;
use serde_json;
use std::convert::TryFrom;

pub fn grant_create(req: &mut Request) -> IronResult<Response> {
    let ro = req
        .extensions
        .get::<JsonApi>()
        .ok_or(DocumentError::NoDocument)
        .and_then(|d| match &d.data {
            OptionalVec::One(Some(go)) => Ok(go),
            OptionalVec::Many(_) => Err(DocumentError::MultipleData),
            _ => Err(DocumentError::NoData),
        })
        .and_then(|go| ResourceObject::<Grant>::try_from(go).map_err(|e| DocumentError::from(e)))?;

    let id = ro
        .relationships
        .as_ref()
        .ok_or(DocumentError::no_relationships(
            "/data/relationships".into(),
        ))
        .and_then(|rels| {
            rels.get("user").ok_or(DocumentError::no_relationship(
                "user".into(),
                "/data/relationships/user".into(),
            ))
        })
        .and_then(|rel| match &rel.data {
            OptionalVec::One(Some(go)) => Ok(go),
            OptionalVec::Many(_) => Err(DocumentError::MultipleData),
            _ => Err(DocumentError::NoData),
        })
        .map_err(|e| ApiError::from(e))
        .and_then(|go| {
            ResourceObject::<User>::try_from(go).map_err(|e| {
                DocumentError::ConversionError {
                    err: e,
                    pointer: Some("/data/relationships/user".into()),
                }
                .into()
            })
        })
        .and_then(|ro| {
            ro.id.ok_or(
                DocumentError::NoId {
                    pointer: Some("/data/relationships/user/id".into()),
                }
                .into(),
            )
        })
        .and_then(|id| into_identity(&id).map_err(|e| ApiError::from(e)))?;

    let attr = ro
        .attributes
        .ok_or(DocumentError::no_attributes("/data/attributes".into()))?;

    let ua = req
        .extensions
        .get::<QaulCore>()
        .unwrap()
        .users()
        .login(id, &attr.secret)
        .map_err(|e| QaulError::from(e))?;

    {
        let UserAuth(id, grant) = ua.clone();
        req.extensions
            .get::<Authenticator>()
            .unwrap()
            .tokens
            .lock()
            .unwrap()
            .insert(grant, id);
    }

    let grant = Grant::from_user_auth(ua)?;

    let doc = Document {
        data: OptionalVec::One(Some(grant.into())),
        ..Default::default()
    };

    Ok(Response::with((
        Status::Created,
        JSONAPI_MIME.clone(),
        serde_json::to_string(&doc).unwrap(),
    )))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::from_identity;
    use anneal::RequestBuilder;
    use japi::{Identifier, Relationship, Relationships};
    use libqaul::{Qaul};
    use std::sync::Arc;

    #[test]
    fn works() {
        let qaul = Arc::new(Qaul::dummy());
        let id = qaul.users().create("test").unwrap().0;

        let mut relationships = Relationships::new();
        relationships.insert(
            "user".into(),
            Relationship {
                data: OptionalVec::One(Some(Identifier::new(from_identity(&id), "user".into()))),
                ..Default::default()
            },
        );
        let ro = ResourceObject {
            id: None,
            attributes: Some(Grant {
                secret: "test".into(),
            }),
            relationships: Some(relationships.clone()),
            links: None,
            meta: None,
        };

        let auth = Authenticator::new();

        let go = RequestBuilder::default_post()
            .add_middleware(QaulCore::new(qaul.clone()))
            .add_middleware(JsonApi)
            .add_middleware(auth.clone())
            .set_primary_data(ro.into())
            .request_response(|mut req| grant_create(&mut req))
            .unwrap()
            .get_primary_data()
            .unwrap();

        let ro = ResourceObject::<Grant>::try_from(go).unwrap();
        assert_eq!(ro.relationships.unwrap(), relationships);
        let grant = ro.id.unwrap();

        assert_eq!(auth.tokens.lock().unwrap().get(&grant), Some(&id));

        qaul.users().change_pw(UserAuth(id, grant), "test2").unwrap();
    }
}
