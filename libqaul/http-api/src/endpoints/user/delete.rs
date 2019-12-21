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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        endpoints::user::route,
        Authenticator,
        models::from_identity,
    };
    use anneal::RequestBuilder;
    use iron::{
        headers::{Authorization, Bearer},
        middleware::Handler,
    };
    use libqaul::{Qaul, users::UserAuth};
    use std::sync::Arc;

    #[test]
    fn works() {
        let qaul = Arc::new(Qaul::dummy());
        let ua = qaul.users().create("test").unwrap();
        let UserAuth(id, grant) = ua;

        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(grant.clone(), id.clone()); }

        let s_id = from_identity(&id);
        assert!(RequestBuilder::delete(&format!("http://127.0.0.1:8000/api/users/{}", s_id))
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant.clone()
            }))
            .add_middleware(QaulCore::new(qaul.clone()))
            .add_middleware(auth)
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            }).unwrap().is_success());

        assert!(qaul.users().get(id).is_err());
    }
}
