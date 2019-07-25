use crate::{
    JsonApi,
    models::{
        UserAuth,
        UserGrant,
    },
    QaulCore,
    JSONAPI_MIME,
};
use chrono::{ DateTime, offset::Utc };
use libqaul::UserAuth as QaulUserAuth;
use iron::{
    prelude::*,
    status::Status,
};
use json_api::{
    Document,
    OptionalVec,
    ResourceObject,
};
use std::convert::TryInto;
use super::{
    AuthError,
    Authenticator,
    CurrentUser,
};

pub fn login(req: &mut Request) -> IronResult<Response> {
    // data should contain exactly one object
    let data = match &req.extensions.get::<JsonApi>().unwrap().data {
        OptionalVec::One(Some(d)) => d,
        OptionalVec::Many(_) => { 
            return Err(AuthError::MultipleData.into());
        },
        _ => {
            return Err(AuthError::NoData.into());
        },
    };

    // try to decode the payload
    let ua : ResourceObject<UserAuth> = match data.try_into() {
        Ok(ua) => ua,
        Err(e) => {
            return Err(AuthError::ConversionError(e).into());
        },
    };

    // is the identity valid
    let (identity, secret) = match UserAuth::into_identity(ua) {
        Ok(id) => id,
        Err(e) => {
            return Err(AuthError::InvalidIdentity(e).into());
        },
    };

    // is there a secret (there has to be a secret!)
    let secret = match secret {
        Some(s) => s,
        None => { 
            return Err(AuthError::NoSecret.into());
        },
    };

    let qaul = req.extensions.get::<QaulCore>().unwrap();

    // perform the login
    let (ident, token) = match qaul.user_login(identity.clone(), &secret) {
        Ok(QaulUserAuth::Trusted(ident, token)) => (ident, token),
        Ok(QaulUserAuth::Untrusted(_)) => { unreachable!(); },
        Err(e) => {
            return Err(AuthError::QaulError(e).into());
        },
    };

    // register the token with the authenticator
    {
        req.extensions.get::<Authenticator>().unwrap()
            .tokens.lock().unwrap()
            .insert(token.clone(), ident);
    }

    // return the grant
    // so you'd think the id here would be the user id right?
    // NO
    // that's illegal according to the JSON:API spec as it'd result in multiple
    // resources having the same (id, type) pair and ALL objects MUST have an id
    // so we have two options:
    // token
    // or something unique like time
    // we do the second one because if we set a cookie and then return the token
    // anyways that'd be a bit silly
    let obj = ResourceObject::new(
        format!("{}", Utc::now().timestamp_millis()),
        Some(UserGrant { token }));

    let doc = Document {
        data: OptionalVec::One(Some(obj.into())),
        ..Default::default()
    };

    Ok(Response::with((Status::Ok, JSONAPI_MIME.clone(), serde_json::to_string(&doc).unwrap())))
}
