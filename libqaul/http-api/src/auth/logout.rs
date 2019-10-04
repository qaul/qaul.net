use crate::{
    Cookies,
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

    // if an auth cookie had been set mark it for removal
    let mut cookies = req.extensions.get_mut::<Cookies>().unwrap();
    if let Some(cookie) = cookies.get("bearer") {
        cookies.remove(cookie.clone());
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

#[cfg(test)]
mod test {
    use anneal::RequestBuilder;
    use crate::{cookie::CookieManager, JsonApi};
    use cookie::{CookieJar, Cookie};
    use iron::{
        method::Method,
        middleware::BeforeMiddleware,
        headers::{Authorization, Bearer},
    };
    use std::io::Read;
    use super::*;
    use libqaul::{
        Qaul,
        UserUpdate,
        Identity,
    };
    use json_api::ResourceObject;

    fn setup() -> (RequestBuilder, Authenticator, String) {
        let qaul = Qaul::start();
        let user_auth = qaul.user_create("a".into()).unwrap();
        let (ident, key) = qaul.user_authenticate(user_auth).unwrap();

        let (before_manager, _) = CookieManager::new();
        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(key.clone(), ident); } 

        let mut rb = RequestBuilder::default();
        rb.add_middleware(QaulCore::new(&qaul));
        rb.add_middleware(before_manager);
        rb.add_middleware(JsonApi);
        rb.add_middleware(auth.clone());

        (
            rb,
            auth,
            key
        )
    }

    #[test]
    fn logout_cookie() {
        let (mut rb, auth, key) = setup();
    
        let mut jar = CookieJar::new();
        jar.add(Cookie::new("bearer", key.clone()));
        let go = rb
            .set_cookies(&jar)
            .request_response(|mut req| {
                let response = logout(&mut req).unwrap();
                assert_eq!(auth.tokens.lock().unwrap().get(&key), None);
                assert_eq!(req.extensions.get::<Cookies>().unwrap().get("bearer"), None);
                Ok(response)
            }).unwrap()
            .get_primary_data().unwrap();
        let ro : ResourceObject<Success> = go.try_into().unwrap();
    }

    #[test]
    fn logout_token() {
        let (mut rb, auth, key) = setup();
    
        let go = rb
            .set_header(Authorization(Bearer { token: key.clone() }))
            .request_response(|mut req| {
                let response = logout(&mut req).unwrap();
                assert_eq!(auth.tokens.lock().unwrap().get(&key), None);
                Ok(response)
            }).unwrap()
            .get_primary_data().unwrap();
        let ro : ResourceObject<Success> = go.try_into().unwrap();
    }
    
    #[test]
    fn no_login() {
        let (rb, _, _) = setup();
        rb.request(|mut req| {
                assert!(logout(&mut req).is_err())
            });
    }
}
