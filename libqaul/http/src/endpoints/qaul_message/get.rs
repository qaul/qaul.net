use crate::{
    error::{QaulError, AuthError, GenericError},
    models::{into_message_id, QaulMessage},
    CurrentUser, QaulCore, JSONAPI_MIME,
};
use iron::{prelude::*, status::Status};
use japi::{Document, OptionalVec};
use router::Router;
use libqaul::messages::MessageQuery;
use serde_json;

pub fn qaul_message_get(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();
    let router = req.extensions.get::<Router>().unwrap();
    let service = router.find("service").unwrap();
    let raw_id = router.find("id").unwrap();
    let id = into_message_id(raw_id.clone())?;

    let mut messages = req.extensions
        .get::<QaulCore>()
        .unwrap()
        .messages()
        .query(auth, service, MessageQuery::Id(id))
        .map_err(QaulError::from)?;

    if messages.len() == 0 {
        Err(GenericError::new("No Message With Id".into())
            .detail(format!("No message found with id {}", raw_id))
            .status(Status::NotFound))?
    } else if messages.len() > 1 {
        Err(GenericError::new("Multiple Messages Found".into())
            .detail(format!("Found multiple messages with id {} which probably means something has gone horribly wrong", raw_id))
            .status(Status::InternalServerError))?
    }

    let obj = QaulMessage::from_message(messages.pop().unwrap().as_ref().clone());

    let doc = Document {
        data: OptionalVec::One(Some(obj.into())),
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
        endpoints::qaul_message::route,
        models::{from_identity, from_message_id},
        Authenticator, QaulMessaging, QaulCore,
        test_utils::TestNetwork,
    };
    use anneal::RequestBuilder;
    use iron::{
        headers::{Authorization, Bearer},
        middleware::Handler,
    };
    use japi::ResourceObject;
    use libqaul::{users::UserAuth, messages::Recipient};
    use std::{sync::Arc, convert::TryFrom};

     #[test]
    fn works() {
        let network = TestNetwork::new();

        let ua_a = network.add_user_a("abc");
        let ua_b = network.add_user_b("abc");
        let UserAuth(id_a, grant_a) = ua_a.clone();
        let UserAuth(id_b, grant_b) = ua_b.clone();

        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(grant_a.clone(), id_a.clone()); }

        network.a.services().register("test");
        network.b.services().register("test");

        let msg_id = network.b
            .messages()
            .send(
                ua_b.clone(),
                Recipient::User(id_a.clone()),
                "test",
                b"beep".to_vec(),
            )
            .unwrap();

        #[allow(deprecated)]
        std::thread::sleep_ms(500);

        let go = RequestBuilder::get(
                &format!("http://127.0.0.1:8000/api/qaul_messages/test/{}", from_message_id(&msg_id)))
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant_a
            }))
            .add_middleware(QaulCore::new(network.a.clone()))
            .add_middleware(auth)
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_primary_data()
            .unwrap();

        let ro = ResourceObject::<QaulMessage>::try_from(go).unwrap();

        assert_eq!(QaulMessage::sender(&ro, "".into()).unwrap(), id_b);
        assert_eq!(QaulMessage::recipient(&ro, "".into()).unwrap(), Recipient::User(id_a));
        assert_eq!(QaulMessage::payload(&ro, "".into()).unwrap(), b"beep");
        assert_eq!(QaulMessage::message_id(&ro, "".into()).unwrap(), msg_id);
    }
}
