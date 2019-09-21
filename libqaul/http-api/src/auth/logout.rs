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

    fn setup() -> (Vec<Box<dyn BeforeMiddleware>>, Authenticator, CookieManager, Identity, String) {
        let qaul = Qaul::start();
        let user_auth = qaul.user_create("a".into()).unwrap();
        let (ident, key) = qaul.user_authenticate(user_auth).unwrap();

        let core = QaulCore::new(&qaul);
        let (before_manager, after_manager) = CookieManager::new();
        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(key.clone(), ident.clone()); } 

        (
            vec![Box::new(core), Box::new(before_manager), Box::new(JsonApi), Box::new(auth.clone())], 
            auth,
            after_manager,
            ident,
            key
        )
    }

    #[test]
    fn logout_cookie() {
        let (chain, auth, cookie_manager, ident, key) = setup();

        let mut jar = CookieJar::new();
        jar.add(Cookie::new("bearer", key.clone()));
        RequestBuilder::new(Method::Get, "https://127.0.0.1/")
            .unwrap()
            .set_cookies(&jar)
            .request(|mut req| {
                for m in chain { m.before(&mut req).unwrap() }
                let response = logout(&mut req).unwrap();

                let mut body = Vec::new();
                response.body.unwrap().write_body(&mut body).unwrap();
                let body = String::from_utf8(body).unwrap();
                let doc : Document = serde_json::from_str(&body).unwrap();
                let go = match doc.data {
                    OptionalVec::One(Some(go)) => go,
                    o => panic!("Exepected single generic object, got {:?}", o),
                };
                let ro : ResourceObject<Success> = go.try_into().unwrap();

                assert_eq!(auth.tokens.lock().unwrap().get(&key), None);
                assert_eq!(req.extensions.get::<Cookies>().unwrap().get("bearer"), None);
            });
    }

    #[test]
    fn logout_token() {
        let (chain, auth, cookie_manager, ident, key) = setup();

        RequestBuilder::new(Method::Get, "https://127.0.0.1/")
            .unwrap()
            .set_header(Authorization(Bearer { token: key.clone() }))
            .request(|mut req| {
                for m in chain { m.before(&mut req).unwrap() }
                let response = logout(&mut req).unwrap();

                let mut body = Vec::new();
                response.body.unwrap().write_body(&mut body).unwrap();
                let body = String::from_utf8(body).unwrap();
                let doc : Document = serde_json::from_str(&body).unwrap();
                let go = match doc.data {
                    OptionalVec::One(Some(go)) => go,
                    o => panic!("Exepected single generic object, got {:?}", o),
                };
                let ro : ResourceObject<Success> = go.try_into().unwrap();

                assert_eq!(auth.tokens.lock().unwrap().get(&key), None);
            });
    }

    #[test]
    fn no_login() {
        let (chain, auth, cookie_manager, ident, key) = setup();
        RequestBuilder::new(Method::Get, "https://127.0.0.1/")
            .unwrap()
            .request(|mut req| {
                for m in chain { m.before(&mut req).unwrap() }
                assert!(logout(&mut req).is_err())
            });
    }
}
