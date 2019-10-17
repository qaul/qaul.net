use crate::{
    error::{AuthError, QaulError, GenericError},
    Authenticator,
    CurrentUser,
    QaulCore,
    JSONAPI_MIME,
};
use iron::{
    status::Status,
    prelude::*,
};
use japi::{ResourceObject, OptionalVec, Document};
use serde_json;
use router::Router;
use libqaul::UserAuth;

pub fn grant_delete(req: &mut Request) -> IronResult<Response> {
    let id = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone().identity();

    let grant = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    
    { req.extensions.get::<QaulCore>().unwrap()
        .user_logout(UserAuth::Trusted(id, grant.into())).map_err(|e| QaulError::from(e))?; }

    { req.extensions.get::<Authenticator>().unwrap().tokens.lock().unwrap().remove(grant); }

    Ok(Response::with((
        Status::NoContent,
        JSONAPI_MIME.clone()
    )))
}
