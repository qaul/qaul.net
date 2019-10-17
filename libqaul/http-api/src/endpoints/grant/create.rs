use crate::{
    error::{ApiError, DocumentError, QaulError},
    models::{Grant, User, into_identity},
    Authenticator,
    JsonApi,
    QaulCore,
    JSONAPI_MIME,
};
use iron::{
    status::Status,
    prelude::*,
};
use japi::{ResourceObject, OptionalVec, Document};
use std::convert::TryFrom;
use serde_json;

pub fn grant_create(req: &mut Request) -> IronResult<Response> {
    let ro = req.extensions.get::<JsonApi>().ok_or(DocumentError::NoDocument)
        .and_then(|d| match &d.data {
            OptionalVec::One(Some(go)) => Ok(go),
            OptionalVec::Many(_) => Err(DocumentError::MultipleData),
            _ => Err(DocumentError::NoData) })
        .and_then(|go| ResourceObject::<Grant>::try_from(go).map_err(|e| DocumentError::from(e)))?;

    let id = ro.relationships.as_ref().ok_or(
            DocumentError::no_relationships("/data/relationships".into()))
        .and_then(|rels| rels.get("user").ok_or(
            DocumentError::no_relationship("user".into(), "/data/relationships/user".into())))
        .and_then(|rel| match &rel.data {
            OptionalVec::One(Some(go)) => Ok(go),
            OptionalVec::Many(_) => Err(DocumentError::MultipleData),
            _ => Err(DocumentError::NoData)})
        .map_err(|e| ApiError::from(e)) 
        .and_then(|go| ResourceObject::<User>::try_from(go)
            .map_err(|e| DocumentError::ConversionError {
                err: e, pointer: Some("/data/relationships/user".into()) }.into()))
        .and_then(|ro| ro.id.ok_or(DocumentError::NoId { 
            pointer: Some("/data/relationships/user/id".into()) }.into()))
        .and_then(|id| into_identity(&id).map_err(|e| ApiError::from(e)))?;

    let attr = ro.attributes.ok_or(DocumentError::no_attributes("/data/attributes".into()))?;

    let ua = {
        req.extensions.get::<QaulCore>().unwrap().user_login(id, &attr.secret)
            .map_err(|e| QaulError::from(e))?
    };

    { 
        let (id, grant) = ua.clone().trusted().unwrap();
        req.extensions.get::<Authenticator>().unwrap().tokens.lock().unwrap().insert(grant, id);
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
