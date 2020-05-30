//! Json message generator

use crate::{
    json::{RequestEnv, ResponseEnv},
    Envelope, Response,
};
use serde_json::{self, Map, Value as JsonValue};
use std::convert::TryFrom;

impl From<(Envelope<Response>, RequestEnv)> for ResponseEnv {
    fn from(env: (Envelope<Response>, RequestEnv)) -> ResponseEnv {
        let Envelope { id, data } = env.0;
        let RequestEnv {
            auth, method, kind, ..
        } = env.1;

        // Turn the response into a map object
        let mut data: Map<String, JsonValue> = match serde_json::to_value(data).unwrap() {
            JsonValue::Object(mut obj) => obj,
            JsonValue::String(s) => Some(("type".into(), "success".into()))
                .into_iter()
                .collect(),
            s => panic!("Unexpected value: {:?}", s),
        };

        // And build the final response envelope
        ResponseEnv {
            id,
            auth,
            method,
            kind,
            total: None,
            next: None,
            data: data.into_iter().collect(),
        }
    }
}

#[test]
fn get_auth() {
    use crate::api::{Envelope, Response};
    use crate::json::{RequestEnv, ResponseEnv};
    use libqaul::{users::UserAuth, Identity};

    let ua = UserAuth(Identity::random(), "my-token-is-great".into());
    let data = Response::Auth(ua);

    let env = Envelope {
        id: "request-id".into(),
        data,
    };

    let request_env = RequestEnv {
        id: "request-id".into(),
        auth: None,
        page: None,
        method: "create".into(),
        kind: "users".into(),
        data: vec![("pw".into(), "my-even-greater-password".into())]
            .into_iter()
            .collect(),
    };

    let response: ResponseEnv = (env.clone(), request_env.clone()).into();

    assert!(response.id == env.id && env.id == request_env.id);

    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}

#[test]
fn user_list() {
    use crate::api::Response;
    use libqaul::{users::UserProfile, Identity};

    let users = vec![
        UserProfile::new(Identity::random()),
        UserProfile::new(Identity::random()),
        UserProfile::new(Identity::random()),
    ];

    let resp = Response::User(users);

    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}

#[async_std::test]
async fn user_delete() {
    use crate::json::{JsonAuth, RequestEnv};
    use crate::{Envelope, Responder};
    use async_std::task::block_on;
    use libqaul::Qaul;
    use qaul_chat::Chat;
    use std::sync::Arc;

    let qaul = Qaul::dummy();
    let chat = Chat::new(Arc::clone(&qaul)).await.unwrap();
    let auth = block_on(qaul.users().create("blep")).unwrap();
    assert_eq!(qaul.users().list().await.len(), 1);

    struct FakeStream;

    #[async_trait::async_trait]
    impl crate::StreamResponder for FakeStream {
        async fn respond(self: Arc<Self>, _: Response) {}
    }

    let responder = Responder {
        streamer: crate::Streamer::new(FakeStream),
        qaul: Arc::clone(&qaul),
        chat: chat,
    };

    let req_env = RequestEnv {
        id: "bapples".into(),
        auth: Some(JsonAuth {
            id: auth.0.clone(),
            token: auth.1.clone(),
        }),
        page: None,
        method: "delete".into(),
        kind: "users".into(),
        data: vec![("purge".into(), true.into())].into_iter().collect(),
    };

    let Envelope { id, data } = req_env.clone().generate_envelope().unwrap();

    let resp = block_on(responder.respond(data));
    let env = Envelope { id, data: resp };

    let resp_env: ResponseEnv = (env, req_env).into();

    assert_eq!(qaul.users().list().await.len(), 0);
}
