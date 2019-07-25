use crate::{
    JsonApi,
    models::{
        UserAuth,
        UserGrant,
        GrantType,
        Success,
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
    let identity = match UserAuth::identity(&ua) {
        Ok(id) => id,
        Err(e) => {
            return Err(AuthError::InvalidIdentity(e).into());
        },
    };

    // is there a secret (there has to be a secret!)
    let attr = match ua.attributes {
        Some(s) => s,
        None => { 
            return Err(AuthError::NoAttributes.into());
        },
    };

    let secret = attr.secret;
    let grant_type = attr.grant_type;

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
    let obj = match grant_type {
        GrantType::Token => ResourceObject::<UserGrant>::new(token, None).into(),
        GrantType::Cookie => { unimplemented!() },
    };

    let doc = Document {
        data: OptionalVec::One(Some(obj)),
        ..Default::default()
    };

    Ok(Response::with((Status::Ok, JSONAPI_MIME.clone(), serde_json::to_string(&doc).unwrap())))
}
