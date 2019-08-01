use crate::{
    QaulCore,
    JSONAPI_MIME,
    models::Success,
};
use libqaul::UserAuth;
use iron::{
    prelude::*,
    status::Status,
};
use json_api::{
    Document,
    OptionalVec,
};
use std::convert::TryInto;
use super::{
    AuthError,
    Authenticator,
    CurrentUser
};

pub fn logout(req: &mut Request) -> IronResult<Response> {
    // we can't log out until we know who we are
    let (identity, token) = match req.extensions.get::<CurrentUser>() {
        Some(UserAuth::Trusted(identity, token)) => (identity, token),
        _ => {
            return Err(AuthError::NotLoggedIn.into());
        },
    };

    // log us out
    let qaul = req.extensions.get::<QaulCore>().unwrap();
    if let Err(e) = qaul.user_logout(UserAuth::Trusted(identity.clone(), token.clone())) {
        return Err(AuthError::QaulError(e).into());
    }

    // tell the authenticator we've logged out
    {
        req.extensions.get::<Authenticator>().unwrap()
            .tokens.lock().unwrap()
            .remove(token);
    }

    // return a little success message
    // we're a JSON:API endpoint (well, probably) so we gotta return something
    let obj = Success::from_message("Successfully logged out".into());

    let doc = Document {
        data: OptionalVec::One(Some(obj.into())),
        ..Default::default()
    };

    Ok(Response::with((Status::Ok, serde_json::to_string(&doc).unwrap(), JSONAPI_MIME.clone())))
}
