use crate::{
    CurrentUser,
    error::{QaulError, DocumentError, AuthError, GenericError, ApiError, Error as JsonError},
    models::{into_identity, User},
    QaulCore,
    JsonApi,
    JSONAPI_MIME,
};
use iron::{
    prelude::*,
    status::Status,
};
use japi::{Error, ResourceObject, Document, OptionalVec, Optional};
use libqaul::users::UserUpdate;
use router::Router;
use serde_json;
use std::convert::TryFrom;

pub fn user_update(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();
    let id = auth.clone().0;

    let ro = req.extensions.get::<JsonApi>().ok_or(DocumentError::NoDocument)
        // TODO: This shouldn't be a clone
        .and_then(|d| d.data.clone().one_or(DocumentError::MultipleData))
        .and_then(|d| d.ok_or(DocumentError::NoData))
        .and_then(|go| ResourceObject::<User>::try_from(go).map_err(|e| DocumentError::from(e)))?;

    let obj_id = ro.id.ok_or(DocumentError::NoId { pointer: Some("/data/id".into()) }.into())
        .and_then(|id| into_identity(&id).map_err(|e| ApiError::from(e)))?;
    if obj_id != id {
        return Err(AuthError::NotAuthorised.into());
    }

    let req_id = into_identity(req.extensions.get::<Router>().unwrap().find("id").unwrap())?;
    if req_id != id {
        return Err(AuthError::NotAuthorised.into());
    }

    let attr = ro.attributes.ok_or(DocumentError::no_attributes("/data/attributes".into()))?;

    let avatar = match attr.avatar.as_ref().map(|s| User::into_avatar(&s, "/data")) {
        Optional::Present(Ok(a)) => Optional::Present(a),
        Optional::Present(Err(e)) => { return Err(e.into()); },
        Optional::Null => Optional::Null,
        Optional::NotPresent => Optional::NotPresent,
    };

    let qaul = req.extensions.get::<QaulCore>().unwrap().users();
    qaul.update(auth.clone(), |profile| {
        match &attr.display_name {
            Optional::Present(v) => { profile.display_name = Some(v.to_string()); },
            Optional::Null => { profile.display_name = None; },
            Optional::NotPresent => {},
        }

        match &attr.real_name {
            Optional::Present(v) => { profile.real_name = Some(v.to_string()); },
            Optional::Null => { profile.real_name = None; },
            Optional::NotPresent => {},
        }

        if let Some(bio) = &attr.bio {
            for (k, v) in bio.iter() {
                if let Some(v) = v {
                    profile.bio.insert(k.to_string(), v.to_string());
                } else {
                    profile.bio.remove(k);
                }
            }
        }

        match &avatar {
            Optional::Present(a) => { profile.avatar = Some(a.to_vec()); },
            Optional::Null => { profile.avatar = None; },
            _ => {},
        }
    }).map_err(|e| QaulError::from(e))?;

    let user = qaul
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
        models::{from_identity, User},
    };
    use anneal::RequestBuilder;
    use iron::{
        headers::{Authorization, Bearer},
        middleware::Handler,
    };
    use japi::{Document, OptionalVec, ResourceObject};
    use libqaul::{Qaul, users::UserAuth};
    use std::{convert::TryFrom, collections::BTreeMap, sync::Arc};

    #[test]
    fn set() {
        let qaul = Arc::new(Qaul::dummy());
        let UserAuth(id, grant) = qaul.users().create("test").unwrap();

        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(grant.clone(), id.clone()); }

        let mut bio = BTreeMap::new();
        bio.insert("gender".to_string(), Some("yes please".to_string()));

        let s_id = from_identity(&id);
        let go = RequestBuilder::patch(
                &format!("http://127.0.0.1:8000/api/users/{}", &s_id))
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant.clone()
            }))
            .set_primary_data(
                ResourceObject {
                    attributes: Some(User {
                        display_name: Optional::Present("display".into()),
                        real_name: Optional::Present("real".into()),
                        bio: Some(bio.clone()),
                        services: None,
                        avatar: Optional::Present("ThisIsTotallyB64".into()),
                    }),
                    id: Some(s_id),
                    relationships: None,
                    links: None,
                    meta: None,
                }.into()
            )
            .add_middleware(QaulCore::new(qaul.clone()))
            .add_middleware(JsonApi)
            .add_middleware(auth)
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_primary_data()
            .unwrap();
        let ro = ResourceObject::<User>::try_from(go).unwrap();
        let attr = ro.attributes.unwrap();
        assert_eq!(attr.display_name, Optional::Present("display".to_string()));
        assert_eq!(attr.real_name, Optional::Present("real".to_string()));
        assert_eq!(attr.bio, Some(bio));
        assert_eq!(attr.avatar, Optional::Present("ThisIsTotallyB64".to_string()));
    }

    #[test]
    fn update() {
        let qaul = Arc::new(Qaul::dummy());
        let UserAuth(id, grant) = { 
            let users = qaul.users();
            let ua = users.create("test").unwrap();
            users.update(ua.clone(), |profile| {
                profile.display_name = Some("display".to_string());
                profile.real_name = Some("real".to_string());

                let mut bio = BTreeMap::new();
                bio.insert("gender".to_string(), "yes please".to_string());
                profile.bio = bio;
                profile.avatar = Some(vec![1,3,1,2]);
            });
            ua
        };

        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(grant.clone(), id.clone()); }

        let mut bio = BTreeMap::new();
        bio.insert("gender".to_string(), Some("no thanks".to_string()));

        let s_id = from_identity(&id);
        let go = RequestBuilder::patch(
                &format!("http://127.0.0.1:8000/api/users/{}", &s_id))
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant.clone()
            }))
            .set_primary_data(
                ResourceObject {
                    attributes: Some(User {
                        display_name: Optional::Present("yalpsid".into()),
                        real_name: Optional::Present("lear".into()),
                        bio: Some(bio.clone()),
                        services: None,
                        avatar: Optional::Present("ThisIsTotallyNotB64".into()),
                    }),
                    id: Some(s_id),
                    relationships: None,
                    links: None,
                    meta: None,
                }.into()
            )
            .add_middleware(QaulCore::new(qaul.clone()))
            .add_middleware(JsonApi)
            .add_middleware(auth)
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_primary_data()
            .unwrap();
        let ro = ResourceObject::<User>::try_from(go).unwrap();
        let attr = ro.attributes.unwrap();
        assert_eq!(attr.display_name, Optional::Present("yalpsid".to_string()));
        assert_eq!(attr.real_name, Optional::Present("lear".to_string()));
        assert_eq!(attr.bio, Some(bio));
        assert_eq!(attr.avatar, Optional::Present("ThisIsTotallyNotB64=".to_string()));
    }

    #[test]
    fn clear() {
        let qaul = Arc::new(Qaul::dummy());
        let UserAuth(id, grant) = { 
            let users = qaul.users();
            let ua = users.create("test").unwrap();
            users.update(ua.clone(), |profile| {
                profile.display_name = Some("display".to_string());
                profile.real_name = Some("real".to_string());

                let mut bio = BTreeMap::new();
                bio.insert("gender".to_string(), "yes please".to_string());
                profile.bio = bio;
                profile.avatar = Some(vec![1,3,1,2]);
            });
            ua
        };

        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(grant.clone(), id.clone()); }

        let mut bio = BTreeMap::new();
        bio.insert("gender".to_string(), None);

        let s_id = from_identity(&id);
        let go = RequestBuilder::patch(
                &format!("http://127.0.0.1:8000/api/users/{}", &s_id))
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant.clone()
            }))
            .set_primary_data(
                ResourceObject {
                    attributes: Some(User {
                        display_name: Optional::Null,
                        real_name: Optional::Null,
                        bio: Some(bio),
                        services: None,
                        avatar: Optional::Null,
                    }),
                    id: Some(s_id),
                    relationships: None,
                    links: None,
                    meta: None,
                }.into()
            )
            .add_middleware(QaulCore::new(qaul.clone()))
            .add_middleware(JsonApi)
            .add_middleware(auth)
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_primary_data()
            .unwrap();
        let ro = ResourceObject::<User>::try_from(go).unwrap();
        let attr = ro.attributes.unwrap();
        assert_eq!(attr.display_name, Optional::NotPresent);
        assert_eq!(attr.real_name, Optional::NotPresent);
        assert_eq!(attr.bio, Some(BTreeMap::new()));
        assert_eq!(attr.avatar, Optional::NotPresent);
    }
}
