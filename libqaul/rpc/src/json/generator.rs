//! Json message generator

use crate::{
    json::{RequestEnv, ResponseEnv},
    Envelope, EnvelopeType,
};
use serde_json::{self, Map, Value as JsonValue};

impl From<(Envelope, RequestEnv)> for ResponseEnv {
    fn from(env: (Envelope, RequestEnv)) -> ResponseEnv {
        let Envelope { id, data } = env.0;
        let RequestEnv {
            auth, method, kind, ..
        } = env.1;

        // Turn the response into a map object
        let mut data: Map<String, JsonValue> = match data {
            EnvelopeType::Response(response) => match serde_json::to_value(response).unwrap() {
                JsonValue::Object(obj) => obj,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        data.remove("type");

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
    use libqaul::{users::UserAuth, Identity};
    use crate::api::{Envelope, Response};

    let ua = UserAuth(Identity::random(), "my-token-is-great".into());
    let data = EnvelopeType::Response(Response::Auth(ua));

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
    use libqaul::{users::UserProfile, Identity};
    use crate::api::Response;

    let users = vec![
        UserProfile::new(Identity::random()),
        UserProfile::new(Identity::random()),
        UserProfile::new(Identity::random()),
    ];

    let resp = Response::User(users);

    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
