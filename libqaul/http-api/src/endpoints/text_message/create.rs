use crate::{
    error::{DocumentError, AuthError, ServiceError, QaulError, GenericError},
    models::TextMessage,
    CurrentUser, JsonApi, QaulMessaging, JSONAPI_MIME,
};
use japi::{ResourceObject, Document, OptionalVec};
use iron::{prelude::*, status::Status};
use std::convert::TryFrom;
use libqaul::messages::MessageQuery;
use serde_json;

pub fn text_message_create(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();

    let ro = ResourceObject::<TextMessage>::try_from(req
            .extensions
            .get::<JsonApi>()
            .ok_or(DocumentError::NoDocument)?
            .data
            .clone()
            .one_or(DocumentError::MultipleData)?
            .ok_or(DocumentError::NoData)?
        )
        .map_err(|e| DocumentError::from(e))?;

    let payload = TextMessage::payload(&ro, "/data".into())?;
    let recipient = TextMessage::recipient(&ro, "/data".into())?;

    let mut msg = {
        let messaging = req
            .extensions
            .get::<QaulMessaging>()
            .ok_or(ServiceError::not_mounted("net.qaul.messaging".into()))?;
        messaging
            .send(auth.clone(), recipient, payload)
            .and_then(|id| messaging.query(auth, MessageQuery::Id(id)))
            .map_err(QaulError::from)?
    };

    if msg.len() == 0 {
        Err(GenericError::new("Failed to find message".into())
            .detail("The message sent successfully but the id returned does not belong to a known message".into())
            .status(Status::InternalServerError))?;
    }

    let obj = TextMessage::from_message(msg.pop().unwrap());

    let doc = Document {
        data: OptionalVec::One(Some(obj.into())),
        ..Default::default()
    };

    Ok(Response::with((
        Status::Created, 
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
    use japi::{ResourceObject, Relationships, Relationship, Identifier, OptionalVec};
    use libqaul::{Qaul, users::UserAuth, messages::Recipient};
    use text_messaging::Messaging;
    use std::sync::Arc;

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

        let mut relationships = Relationships::new();
        relationships.insert(
            "recipient".into(), 
            Relationship {
                data: OptionalVec::One(Some(Identifier::new(from_identity(&id_b), "user".into()))),
                ..Default::default()
            },
        );

        let go = RequestBuilder::post("http://127.0.0.1:8000/api/text_messages")
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant_a
            }))
            .set_primary_data(
                ResourceObject {
                    attributes: Some(TextMessage {
                        payload: "hewwo".into(),
                        sign: None,
                    }),
                    id: None,
                    relationships: Some(relationships),
                    links: None,
                    meta: None,
                }.into()
            )
            .add_middleware(QaulCore::new(network.a.clone()))
            .add_middleware(JsonApi)
            .add_middleware(auth)
            .add_middleware(QaulMessaging::new(&messaging_a))
            .request_response(|mut req| {
                text_message_create(&mut req)
            })
            .unwrap()
            .get_primary_data()
            .unwrap();

        let ro = ResourceObject::<TextMessage>::try_from(go).unwrap();

        assert_eq!(TextMessage::sender(&ro, "".into()).unwrap(), id_a);
        assert_eq!(TextMessage::recipient(&ro, "".into()).unwrap(), Recipient::User(id_b));
        assert_eq!(TextMessage::payload(&ro, "".into()).unwrap().text, "hewwo");

        #[allow(deprecated)]
        std::thread::sleep_ms(500);

        let msg = messaging_b.poll(ua_b).unwrap();
        assert_eq!(msg.sender, id_a);
        assert_eq!(msg.recipient, Recipient::User(id_b));
        assert_eq!(msg.payload.text, "hewwo");
        assert_eq!(msg.id, TextMessage::message_id(&ro, "".to_string()).unwrap());
    }
}
