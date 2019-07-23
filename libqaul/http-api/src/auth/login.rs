use crate::{
    JsonApi,
    models::UserAuth,
};
use iron::{
    prelude::*,
};
use json_api::{
    Document,
    OptionalVec,
    ResourceObject,
};
use std::convert::TryInto;
use super::AuthError;

pub fn login(req: &mut Request) -> IronResult<Response> {
    // data should contain exactly one object
    let data = match req.extensions.get::<JsonApi>().unwrap().data {
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

    // is there a secret (there has to be a secret!)
    let secret = match ua.attributes.and_then(|a| a.secret.take()) {
        Some(s) => s,
        None => {
            return Err(AuthError::NoSecret.into());
        },
    };

    // is the identity valid
    let identity = match UserAuth::into_identity(ua) {
        Ok(id) => id,
        Err(e) => {
            return Err(AuthError::InvalidIdentity(e).into());
        },
    };
}
