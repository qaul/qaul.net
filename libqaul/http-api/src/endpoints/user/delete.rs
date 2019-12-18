use crate::{
    CurrentUser,
    error::{QaulError, AuthError},
    models::{into_identity, User},
    QaulCore,
    JSONAPI_MIME,
};
use iron::{
    prelude::*,
    status::Status,
};
use router::Router;
use serde_json;

pub fn user_delete(req: &mut Request) -> IronResult<Response> {
    let ua = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();
    let identity = ua.clone().0;

    if into_identity(req.extensions.get::<Router>().unwrap().find("id").unwrap())? != identity {
        return Err(AuthError::NotAuthorised.into());
    }

    req.extensions.get::<QaulCore>().unwrap().users().delete(ua).map_err(|e| QaulError::from(e))?;

    Ok(Response::with((JSONAPI_MIME.clone(), Status::NoContent)))
}
