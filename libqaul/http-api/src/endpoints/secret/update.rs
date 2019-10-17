use crate::{
    error::{AuthError, DocumentError, QaulError},
    models::{Secret, into_identity},
    CurrentUser,
    JsonApi,
    QaulCore,
};
use iron::{
    status::Status,
    prelude::*,
};
use japi::{ResourceObject, OptionalVec, Document};
use router::Router;
use std::convert::TryFrom;
use serde_json;

pub fn secret_update(req: &mut Request) -> IronResult<Response> {
    let auth_id = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone().identity();

    let ro = req.extensions.get::<JsonApi>().ok_or(DocumentError::NoDocument)
        .and_then(|d| match &d.data {
            OptionalVec::One(Some(go)) => Ok(go),
            OptionalVec::Many(_) => Err(DocumentError::MultipleData),
            _ => Err(DocumentError::NoData)})
        .and_then(|go| ResourceObject::<Secret>::try_from(go).map_err(|e| DocumentError::from(e)))?;

    // check that the authenticated user is the same as the secret's id is for
    if Secret::into_identity(&ro, "/data/id".into())? != auth_id {
        return Err(AuthError::NotAuthorised.into());
    }

    // check that the authenticated user is the same as the path id
    if into_identity(&req.extensions.get::<Router>().unwrap().find("id").unwrap())? != auth_id {
        return Err(AuthError::NotAuthorised.into());
    }

    let attr = ro.attributes.ok_or(DocumentError::no_attributes("/data/attributes".into()))?;

    let old_val = attr.old_value.ok_or(
        DocumentError::no_attribute("old_value".into(), "/data/attributes/old_value".into()))?;

    {
        let qaul = req.extensions.get::<QaulCore>().unwrap();

        // check that the old password is correct
        let ua = qaul.user_login(auth_id.clone(), &old_val).map_err(|e| QaulError::from(e))?;
        qaul.user_change_pw(ua.clone(), &attr.value).map_err(|e| QaulError::from(e))?;
        qaul.user_logout(ua).map_err(|e| QaulError::from(e))?;
    }

    Ok(Response::with(Status::NoContent))
}
