use crate::{
    error::{DocumentError, AuthError, QaulError, GenericError},
    models::QaulMessage,
    CurrentUser, JsonApi, QaulCore, JSONAPI_MIME,
};
use japi::{ResourceObject, Document, OptionalVec};
use iron::{prelude::*, status::Status};
use std::convert::TryFrom;
use libqaul::messages::MessageQuery;
use serde_json;
use router::Router;

pub fn qaul_message_create(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();
    let service = req.extensions.get::<Router>().unwrap().find("service").unwrap();

    let ro = ResourceObject::<QaulMessage>::try_from(req
            .extensions
            .get::<JsonApi>()
            .ok_or(DocumentError::NoDocument)?
            .data
            .clone()
            .one_or(DocumentError::MultipleData)?
            .ok_or(DocumentError::NoData)?
        )
        .map_err(|e| DocumentError::from(e))?;

    let payload = QaulMessage::payload(&ro, "/data".into())?;
    let recipient = QaulMessage::recipient(&ro, "/data".into())?;

    let mut msg = {
        let messaging = req
            .extensions
            .get::<QaulCore>()
            .unwrap()
            .messages();
        messaging
            .send(auth.clone(), recipient, service, payload)
            .and_then(|id| messaging.query(auth, service, MessageQuery::Id(id)))
            .map_err(QaulError::from)?
    };

    if msg.len() == 0 {
        Err(GenericError::new("Failed to find message".into())
            .detail("The message sent successfully but the id returned does not belong to a known message".into())
            .status(Status::InternalServerError))?;
    }

    let obj = QaulMessage::from_message(msg.pop().unwrap().as_ref().clone());

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
        endpoints::qaul_message::route,
        Authenticator,
        test_utils::TestNetwork,
    };
    use anneal::RequestBuilder;
    use iron::{
        headers::{Authorization, Bearer},
        middleware::Handler,
    };
    use japi::{ResourceObject, Relationships, Relationship, Identifier, OptionalVec};
    use libqaul::{Qaul, users::UserAuth, messages::Recipient};
    use std::sync::Arc;
    use base64::{encode_config, URL_SAFE};

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

        let mut relationships = Relationships::new();
        relationships.insert(
            "recipient".into(), 
            Relationship {
                data: OptionalVec::One(Some(Identifier::new(from_identity(&id_b), "user".into()))),
                ..Default::default()
            },
        );

        let go = RequestBuilder::post("http://127.0.0.1:8000/api/qaul_messages/test")
            .unwrap()
            .set_header(Authorization(Bearer {
                token: grant_a
            }))
            .set_primary_data(
                ResourceObject {
                    attributes: Some(QaulMessage {
                        payload: encode_config(b"hewwo", URL_SAFE),
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
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_primary_data()
            .unwrap();

        let ro = ResourceObject::<QaulMessage>::try_from(go).unwrap();

        assert_eq!(QaulMessage::sender(&ro, "".into()).unwrap(), id_a);
        assert_eq!(QaulMessage::recipient(&ro, "".into()).unwrap(), Recipient::User(id_b));
        assert_eq!(QaulMessage::payload(&ro, "".into()).unwrap(), b"hewwo");

        #[allow(deprecated)]
        std::thread::sleep_ms(500);

        let msg = network.b.messages().poll(ua_b, "test").unwrap();
        assert_eq!(msg.sender, id_a);
        assert_eq!(msg.recipient, Recipient::User(id_b));
        assert_eq!(msg.payload, b"hewwo");
        assert_eq!(msg.id, QaulMessage::message_id(&ro, "".to_string()).unwrap());
    }
}
