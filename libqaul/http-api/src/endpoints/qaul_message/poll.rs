use crate::{
    error::{AuthError, QaulError},
    models::QaulMessage,
    CurrentUser, QaulCore, JSONAPI_MIME,
};
use japi::{Document, OptionalVec};
use iron::{prelude::*, status::Status};
use serde_json;
use libqaul::error::Error;
use router::Router;

pub fn qaul_message_poll(req: &mut Request) -> IronResult<Response> {
    let auth = req.extensions.get::<CurrentUser>().ok_or(AuthError::NotLoggedIn)?.clone();
    let service = req.extensions.get::<Router>().unwrap().find("service").unwrap();

    let obj = match req.extensions
        .get::<QaulCore>()
        .unwrap()
        .messages()
        .poll(auth, service) {
            Ok(msg) => QaulMessage::from_message(msg.as_ref().clone()),
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
        endpoints::qaul_message::route,
        models::from_identity,
        Authenticator,
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
                b"hewwo".to_vec(),
            )
            .unwrap();

        #[allow(deprecated)]
        std::thread::sleep_ms(500);

        let go = RequestBuilder::get("http://127.0.0.1:8000/api/qaul_messages/test")
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
        assert_eq!(QaulMessage::payload(&ro, "".into()).unwrap(), b"hewwo");
        assert_eq!(QaulMessage::message_id(&ro, "".into()).unwrap(), msg_id);
    }
}
