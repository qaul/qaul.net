use crate::{
    error::{AuthError, DocumentError, QaulError},
    models::{into_identity, Secret},
    CurrentUser, JsonApi, QaulCore,
};
use iron::{prelude::*, status::Status};
use japi::{Document, OptionalVec, ResourceObject};
use router::Router;
use serde_json;
use std::convert::TryFrom;

pub fn secret_update(req: &mut Request) -> IronResult<Response> {
    let auth_id = req
        .extensions
        .get::<CurrentUser>()
        .ok_or(AuthError::NotLoggedIn)?
        .clone()
        .0;

    let ro = req
        .extensions
        .get::<JsonApi>()
        .ok_or(DocumentError::NoDocument)
        .and_then(|d| match &d.data {
            OptionalVec::One(Some(go)) => Ok(go),
            OptionalVec::Many(_) => Err(DocumentError::MultipleData),
            _ => Err(DocumentError::NoData),
        })
        .and_then(|go| {
            ResourceObject::<Secret>::try_from(go).map_err(|e| DocumentError::from(e))
        })?;

    // check that the authenticated user is the same as the secret's id is for
    if Secret::into_identity(&ro, "/data/id".into())? != auth_id {
        return Err(AuthError::NotAuthorised.into());
    }

    // check that the authenticated user is the same as the secret's user relationship
    let rel_id = ro
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
            OptionalVec::One(Some(d)) => Ok(d),
            // TODO: this error needs a pointer
            OptionalVec::Many(_) => Err(DocumentError::MultipleData),
            _ => Err(DocumentError::NoData),
        })?;
    if into_identity(&rel_id.id)? != auth_id {
        return Err(AuthError::NotAuthorised.into());
    }

    // check that the authenticated user is the same as the path id
    if into_identity(&req.extensions.get::<Router>().unwrap().find("id").unwrap())? != auth_id {
        return Err(AuthError::NotAuthorised.into());
    }

    let attr = ro
        .attributes
        .ok_or(DocumentError::no_attributes("/data/attributes".into()))?;

    let old_val = attr.old_value.ok_or(DocumentError::no_attribute(
        "old_value".into(),
        "/data/attributes/old_value".into(),
    ))?;

    let qaul = req.extensions.get::<QaulCore>().unwrap();

    // check that the old password is correct
    let ua = qaul
        .users()
        .login(auth_id.clone(), &old_val)
        .map_err(|e| QaulError::from(e))?;
    qaul.users()
        .change_pw(ua.clone(), &attr.value)
        .map_err(|e| QaulError::from(e))?;
    qaul.users().logout(ua).map_err(|e| QaulError::from(e))?;

    Ok(Response::with(Status::NoContent))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{endpoints::secret::route, models::from_identity, Authenticator};
    use anneal::RequestBuilder;
    use iron::{
        headers::{Authorization, Bearer},
        middleware::Handler,
    };
    use japi::{Identifier, Relationship, Relationships};
    use libqaul::{Qaul, users::UserAuth};
    use std::sync::Arc;

    #[test]
    fn works() {
        let qaul = Arc::new(Qaul::dummy());
        let UserAuth(id, grant) = qaul.users().create("test").unwrap();

        let mut relationships = Relationships::new();
        relationships.insert(
            "user".into(),
            Relationship {
                data: OptionalVec::One(Some(Identifier::new(from_identity(&id), "user".into()))),
                ..Default::default()
            },
        );
        let ro = ResourceObject {
            id: Some(from_identity(&id)),
            attributes: Some(Secret {
                value: "test2".into(),
                old_value: Some("test".into()),
            }),
            relationships: Some(relationships),
            links: None,
            meta: None,
        };

        let mut auth = Authenticator::new();
        {
            auth.tokens
                .lock()
                .unwrap()
                .insert(grant.clone(), id.clone());
        }

        assert_eq!(
            RequestBuilder::patch(&format!(
                "http://127.0.0.1:8000/api/secrets/{}",
                from_identity(&id)
            ))
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant.clone()
            }))
            .set_primary_data(ro.into())
            .add_middleware(QaulCore::new(qaul.clone()))
            .add_middleware(JsonApi)
            .add_middleware(auth)
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_status()
            .unwrap(),
            &Status::NoContent
        );

        assert!(qaul.users().login(id, "test2").is_ok())
    }
}
