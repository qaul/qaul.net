use crate::{
    models::{into_identity, TextMessage},
    error::{QaulError, AuthError, GenericError, ServiceError},
    CurrentUser, QaulMessaging, JSONAPI_MIME,
};
use japi::{GenericObject, Document, OptionalVec};
use iron::{status::Status, prelude::*};
use libqaul::{Identity, messages::{MessageQuery, Recipient}};

pub fn text_message_query(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();

    let (key, value) = req.url.as_ref().query_pairs().next().unwrap();
    let query = match key.as_ref() {
        "sender" => MessageQuery::Sender(into_identity(&value)?),
        "recipient" if value.len() == 0 => MessageQuery::Recipient(Recipient::Flood),
        "recipient" => {
            let mut recipients = value
                .split(",")
                .map(|id| into_identity(&id))
                .collect::<Result<Vec<Identity>, GenericError>>()?;
            MessageQuery::Recipient(
                if recipients.len() == 1 { Recipient::User(recipients.pop().unwrap()) }
                else { Recipient::Group(recipients) }
            )
        },
        key => { 
            Err(GenericError::new("Unknown Query".into())
                .detail(format!("The application does not understand query parameter {}", key))
                .status(Status::BadRequest)
                .parameter(key.to_string()))?
        },
    };

    let objs = req.extensions
        .get::<QaulMessaging>()
        .ok_or(ServiceError::not_mounted("net.qaul.messaging".into()))?
        .query(auth, query)
        .map_err(QaulError::from)?
        .into_iter()
        .map(|msg| TextMessage::from_message(msg).into())
        .collect::<Vec<GenericObject>>();

    let doc = Document {
        data: OptionalVec::Many(objs),
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
    use router::Router;

     #[test]
    fn works() {
        let network = TestNetwork::new();

        let ua_a = network.add_user_a("abc");
        let ua_b = network.add_user_b("abc");
        let ua_c = network.add_user_a("abc");
        let UserAuth(id_a, grant_a) = ua_a.clone();
        let UserAuth(id_b, grant_b) = ua_b.clone();
        let UserAuth(id_c, grant_c) = ua_c.clone();

        let auth = Authenticator::new();
        { auth.tokens.lock().unwrap().insert(grant_a.clone(), id_a.clone()); }

        let messaging_a = Messaging::new(network.a.clone());
        let messaging_b = Messaging::new(network.b.clone());

        // TODO: test group and flood modes
        let single_id = messaging_b
            .send(
                ua_b.clone(),
                Recipient::User(id_a.clone()),
                TextPayload { text: "hewwo".into() },
            )
            .unwrap();
        //let group_id = messaging_b
        //    .send(
        //        ua_b.clone(),
        //        Recipient::Group(vec![id_a.clone(), id_c.clone()]),
        //        TextPayload { text: "hewwo".into() },
        //    )
        //    .unwrap();
        //let flood_id = messaging_b
        //    .send(
        //        ua_b.clone(),
        //        Recipient::Flood,
        //        TextPayload { text: "hewwo".into() },
        //    )
        //    .unwrap();

        #[allow(deprecated)]
        std::thread::sleep_ms(500);

        let mut rb = RequestBuilder::default();
        rb.set_header(Authorization(Bearer {
                token: grant_a
            }))
            .add_middleware(QaulCore::new(network.a.clone()))
            .add_middleware(auth)
            .add_middleware(QaulMessaging::new(&messaging_a));

        let doc = rb
            .set_url(&format!(
                "http://127.0.0.1:8000/api/text_messages?sender={}", 
                from_identity(&id_b),
            ))
            .unwrap()
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_document()
            .unwrap();
        assert_eq!(doc.data.many_or("").unwrap().len(), 1);

        let doc = rb
            .set_url(&format!(
                "http://127.0.0.1:8000/api/text_messages?recipient={}", 
                from_identity(&id_a),
            ))
            .unwrap()
            .request_response(|mut req| {
                let mut router = Router::new();
                route(&mut router);
                router.handle(&mut req)
            })
            .unwrap()
            .get_document()
            .unwrap();
        let ro = ResourceObject::<TextMessage>::try_from(&doc.data.many_or("").unwrap()[0]).unwrap();
        assert_eq!(ro.id.unwrap(), from_message_id(&single_id));

        //let doc = rb
        //    .set_url(&format!(
        //        "http://127.0.0.1:8000/api/text_messages?recipient={},{}", 
        //        from_identity(&id_a),
        //        from_identity(&id_c),
        //    ))
        //    .unwrap()
        //    .request_response(|mut req| {
        //        let mut router = Router::new();
        //        route(&mut router);
        //        router.handle(&mut req)
        //    })
        //    .unwrap()
        //    .get_document()
        //    .unwrap();
        //let ro = ResourceObject::<TextMessage>::try_from(&doc.data.many_or("").unwrap()[0]).unwrap();
        //assert_eq!(ro.id.unwrap(), from_message_id(&group_id));
    }
}
