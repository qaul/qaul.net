use crate::{
    error::{QaulError, AuthError, ServiceError, GenericError},
    models::{into_message_id, TextMessage},
    CurrentUser, QaulMessaging, JSONAPI_MIME,
};
use iron::{prelude::*, status::Status};
use japi::{Document, OptionalVec};
use router::Router;
use libqaul::messages::MessageQuery;
use serde_json;

pub fn text_message_get(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();
    let raw_id = req.extensions.get::<Router>().unwrap().find("id").unwrap(); 
    let id = into_message_id(raw_id.clone())?; 

    let mut messages = req.extensions
        .get::<QaulMessaging>()
        .ok_or(ServiceError::not_mounted("net.qaul.messaging".into()))?
        .query(auth, MessageQuery::Id(id))
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

    let obj = TextMessage::from_message(messages.pop().unwrap());

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
        endpoints::text_message::route,
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
    use text_messaging::{Messaging, TextPayload};
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

        let messaging_a = Messaging::new(network.a.clone());
        let messaging_b = Messaging::new(network.b.clone());

        let msg_id = messaging_b
            .send(
                ua_b.clone(),
                Recipient::User(id_a.clone()),
                TextPayload { text: "hewwo".into() },
            )
            .unwrap();

        #[allow(deprecated)]
        std::thread::sleep_ms(500);

        let go = RequestBuilder::get(
                &format!("http://127.0.0.1:8000/api/text_messages/{}", from_message_id(&msg_id)))
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant_a
            }))
            .add_middleware(QaulCore::new(network.a.clone()))
            .add_middleware(auth)
            .add_middleware(QaulMessaging::new(&messaging_a))
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_primary_data()
            .unwrap();

        let ro = ResourceObject::<TextMessage>::try_from(go).unwrap();

        assert_eq!(TextMessage::sender(&ro, "".into()).unwrap(), id_b);
        assert_eq!(TextMessage::recipient(&ro, "".into()).unwrap(), Recipient::User(id_a));
        assert_eq!(TextMessage::payload(&ro, "".into()).unwrap().text, "hewwo");
        assert_eq!(TextMessage::message_id(&ro, "".into()).unwrap(), msg_id);
    }
}
