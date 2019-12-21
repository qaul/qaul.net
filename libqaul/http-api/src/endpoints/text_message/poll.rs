use crate::{
    error::{AuthError, ServiceError, QaulError},
    models::TextMessage,
    CurrentUser, QaulMessaging, JSONAPI_MIME,
};
use japi::{Document, OptionalVec};
use iron::{prelude::*, status::Status};
use serde_json;
use libqaul::error::Error;

pub fn text_message_poll(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();

    let obj = match req.extensions
        .get::<QaulMessaging>()
        .ok_or(ServiceError::not_mounted("net.qaul.messaging".into()))?
        .poll(auth) {
            Ok(msg) => TextMessage::from_message(msg),
            Err(Error::NoData) => { 
                return Ok(Response::with((JSONAPI_MIME.clone(), Status::NoContent))); 
            },
            Err(e) => { 
                Err(QaulError::from(e))? 
            },
        };

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
        models::from_identity,
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

        let go = RequestBuilder::get("http://127.0.0.1:8000/api/text_messages")
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant_a
            }))
            .add_middleware(QaulCore::new(network.a.clone()))
            .add_middleware(auth)
            .add_middleware(QaulMessaging::new(&messaging_a))
            .request_response(|mut req| {
                text_message_poll(&mut req)
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
