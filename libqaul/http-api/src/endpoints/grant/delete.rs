use crate::{
    error::{AuthError, GenericError, QaulError},
    Authenticator, CurrentUser, QaulCore, JSONAPI_MIME,
};
use iron::{prelude::*, status::Status};
use japi::{Document, OptionalVec, ResourceObject};
use libqaul::users::UserAuth;
use router::Router;
use serde_json;

pub fn grant_delete(req: &mut Request) -> IronResult<Response> {
    let id = req
        .extensions
        .get::<CurrentUser>()
        .ok_or(AuthError::NotLoggedIn)?
        .clone()
        .0;

    let grant = req.extensions.get::<Router>().unwrap().find("id").unwrap();

    req.extensions
        .get::<QaulCore>()
        .unwrap()
        .users()
        .logout(UserAuth(id, grant.into()))
        .map_err(|e| QaulError::from(e))?;

    {
        req.extensions
            .get::<Authenticator>()
            .unwrap()
            .tokens
            .lock()
            .unwrap()
            .remove(grant);
    }

    Ok(Response::with((Status::NoContent, JSONAPI_MIME.clone())))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{endpoints::grant::route, models::from_identity};
    use anneal::RequestBuilder;
    use iron::{
        headers::{Authorization, Bearer},
        middleware::Handler,
    };
    use libqaul::{users::UserAuth, Qaul};
    use std::sync::Arc;

    #[test]
    fn works() {
        let qaul = Arc::new(Qaul::dummy());
        let user_auth = qaul.users().create("test").unwrap();
        let id = user_auth.0;
        let grant = user_auth.1;

        let auth = Authenticator::new();
        {
            auth.tokens
                .lock()
                .unwrap()
                .insert(grant.clone(), id.clone());
        }

        assert_eq!(
            RequestBuilder::delete(&format!("http://127.0.0.1:8000/api/grants/{}", grant))
                .unwrap()
                .set_header(Authorization(Bearer {
                    token: grant.clone()
                }))
                .add_middleware(QaulCore::new(qaul.clone()))
                .add_middleware(auth.clone())
                .request_response(|mut req| {
                    let mut router = Router::new();
                    route(&mut router);
                    router.handle(&mut req)
                })
                .unwrap()
                .get_status()
                .unwrap(),
            &Status::NoContent
        );
        assert!(auth.tokens.lock().unwrap().get(&grant).is_none());
        assert!(qaul.users().change_pw(UserAuth(id, grant), "test2").is_err());
    }
}
