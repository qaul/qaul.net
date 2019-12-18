use crate::{
    CurrentUser,
    error::{QaulError, DocumentError, AuthError, GenericError, ApiError, Error as JsonError},
    models::{into_identity, User},
    QaulCore,
    JsonApi,
    JSONAPI_MIME,
};
use iron::{
    prelude::*,
    status::Status,
};
use japi::{Error, ResourceObject, Document, OptionalVec, Optional};
use libqaul::users::UserUpdate;
use router::Router;
use serde_json;
use std::convert::TryFrom;

pub fn user_update(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();
    let id = auth.clone().0;

    let ro = req.extensions.get::<JsonApi>().ok_or(DocumentError::NoDocument)
        // TODO: This shouldn't be a clone
        .and_then(|d| d.data.clone().one_or(DocumentError::MultipleData))
        .and_then(|d| d.ok_or(DocumentError::NoData))
        .and_then(|go| ResourceObject::<User>::try_from(go).map_err(|e| DocumentError::from(e)))?;

    let obj_id = ro.id.ok_or(DocumentError::NoId { pointer: Some("/data/id".into()) }.into())
        .and_then(|id| into_identity(&id).map_err(|e| ApiError::from(e)))?;
    if obj_id != id {
        return Err(AuthError::NotAuthorised.into());
    }

    let req_id = into_identity(req.extensions.get::<Router>().unwrap().find("id").unwrap())?;
    if req_id != id {
        return Err(AuthError::NotAuthorised.into());
    }

    let attr = ro.attributes.ok_or(DocumentError::no_attributes("/data/attributes".into()))?;

    let avatar = match attr.avatar.as_ref().map(|s| User::into_avatar(&s, "/data")) {
        Optional::Present(Ok(a)) => Optional::Present(a),
        Optional::Present(Err(e)) => { return Err(e.into()); },
        Optional::Null => Optional::Null,
        Optional::NotPresent => Optional::NotPresent,
    };

    let qaul = req.extensions.get::<QaulCore>().unwrap().users();
    qaul.update(auth.clone(), |profile| {
        match &attr.display_name {
            Optional::Present(v) => { profile.display_name = Some(v.to_string()); },
            Optional::Null => { profile.display_name = None; },
            Optional::NotPresent => {},
        }

        match &attr.real_name {
            Optional::Present(v) => { profile.real_name = Some(v.to_string()); },
            Optional::Null => { profile.real_name = None; },
            Optional::NotPresent => {},
        }

        if let Some(bio) = &attr.bio {
            for (k, v) in bio.iter() {
                if let Some(v) = v {
                    profile.bio.insert(k.to_string(), v.to_string());
                } else {
                    profile.bio.remove(k);
                }
            }
        }

        match &avatar {
            Optional::Present(a) => { profile.avatar = Some(a.to_vec()); },
            Optional::Null => { profile.avatar = None; },
            _ => {},
        }
    }).map_err(|e| QaulError::from(e))?;

    let user = qaul
        .get(id)
        .map_err(|e| QaulError::from(e))?;

    let doc = Document {
        data: OptionalVec::One(Some(User::from_service_user_with_data(user).into())),
        ..Default::default()
    };

    Ok(Response::with((
        Status::Ok,
        JSONAPI_MIME.clone(),
        serde_json::to_string(&doc).unwrap(),
    )))
}
