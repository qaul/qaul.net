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

#[cfg(test)]
mod test {
    use super::*;
    use anneal::RequestBuilder;
    use libqaul::{Qaul, UserAuth};
    use crate::{
        models::from_identity,
        endpoints::grant::route,
    };
    use iron::{
        middleware::Handler,
        headers::{Authorization, Bearer},
    };

    #[test]
    fn works() {
        let qaul = Qaul::start();
        let (id, grant) = qaul.user_create("test").unwrap().trusted().unwrap();

        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(grant.clone(), id.clone()); }

        assert_eq!(RequestBuilder::delete(&format!("http://127.0.0.1:8000/api/grant/{}", grant))
            .unwrap()
            .set_header(Authorization(Bearer { token: grant.clone() }))
            .add_middleware(QaulCore::new(&qaul))
            .add_middleware(auth.clone())
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            }).unwrap().get_status().unwrap(), &Status::NoContent);
        assert!(auth.tokens.lock().unwrap().get(&grant).is_none());
        assert!(qaul.user_delete(UserAuth::Trusted(id, grant)).is_err());

    }
}
