use crate::{
    error::QaulError,
    models::{into_identity, User},
    CurrentUser, QaulCore, JSONAPI_MIME,
};
use iron::{prelude::*, status::Status};
use japi::{Document, OptionalVec};
use router::Router;
use serde_json;

pub fn user_get(req: &mut Request) -> IronResult<Response> {
    let id = into_identity(req.extensions.get::<Router>().unwrap().find("id").unwrap())?;

    // if we are authenticated as the same user we're getting we'll make a trusted request
    let id = req
        .extensions
        .get::<CurrentUser>()
        .and_then(|user| {
            if user.0 == id {
                Some(user.0.clone())
            } else {
                None
            }
        })
        .unwrap_or(id);

    let user = req
        .extensions
        .get::<QaulCore>()
        .unwrap()
        .users()
        .get(id)
        .map_err(|e| QaulError::from(e))?;

    let doc = Document {
        data: OptionalVec::One(Some(User::from_service_user_with_data(user).into())),
        ..Default::default()
    };

    Ok(Response::with((
        Status::Ok,
        JSONAPI_MIME.clone(),
        serde_json::to_string(&doc).unwrap(),
    )))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        endpoints::user::route,
        Authenticator,
        models::{from_identity, Secret},
    };
    use anneal::RequestBuilder;
    use iron::middleware::Handler;
    use japi::ResourceObject;
    use libqaul::Qaul;
    use std::{
        convert::TryFrom,
        sync::Arc,
    };

    #[test]
    fn works() {
        let qaul = Arc::new(Qaul::dummy());
        let ua = qaul.users().create("test").unwrap();
        qaul.users()
            .update(ua.clone(), |user| {
                user.display_name = Some("boop".into());
            })
            .unwrap();

        let id = from_identity(&ua.0);
        let go = RequestBuilder::get(&format!("http://127.0.0.1:8000/api/users/{}", id))
            .unwrap()
            .add_middleware(QaulCore::new(qaul.clone()))
            .add_middleware(Authenticator::new())
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_primary_data()
            .unwrap();

        let ro = ResourceObject::<User>::try_from(go).unwrap();
        assert_eq!(ro.id.unwrap(), id);
        assert_eq!(ro.attributes.unwrap().display_name.unwrap(), "boop");

        let rels = ro.relationships.unwrap();
        let rel = rels.get("secret").unwrap();
        let secret = match &rel.data {
            OptionalVec::One(Some(d)) => d,
            _ => panic!("No or multiple data"),
        };
        assert_eq!(secret.id, id);
    }
}
