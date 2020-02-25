use crate::{JsonAuth, JsonMap, RequestEnv};
use libqaul::users::UserAuth;
use libqaul_rpc::{Envelope, EnvelopeType, Request};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub(crate) struct AuthInject<'env> {
    auth: Option<JsonAuth>,
    method: &'env str,
    kind: &'env str,
}

/// Inject auth info, and turn the data map into a concrete object
///
/// The way we do this here should hopefully be optimisable enough,
/// because we don't otherwise touch anything and rustc is a smart
/// cookie, but this function might still break down with larger
/// payloads and need more optimisations.
fn de_json<'env, T: DeserializeOwned>(mut data: JsonMap, auth: AuthInject<'env>) -> T {
    dbg!(&auth);
    match (auth.kind, auth.method) {
        // We don't want to inject the auth info for a few cases
        ("users", "list")
        | ("users", "login")
        | ("users", "create")
        | ("users", "get")
        | ("files", "list") => {}
        (_, _) => {
            data.insert(
                "auth".into(),
                serde_json::to_value(UserAuth::from(auth.auth.unwrap()))
                    .expect("Failed to inject auth"),
            );
        }
    };

    serde_json::from_value(serde_json::to_value(&data).unwrap())
        .expect(&format!("Failed to parse websocket payload `{:#?}`", &data))
}

/// Wrap a request type
#[inline(always)]
fn req(inner: Request) -> EnvelopeType {
    EnvelopeType::Request(inner)
}

impl From<RequestEnv> for Envelope {
    fn from(je: RequestEnv) -> Self {
        let RequestEnv {
            id,
            page: _,
            method,
            kind,
            auth,
            data,
        } = je; // m'apelle janette

        let auth = AuthInject {
            auth,
            kind: &kind,
            method: &method,
        };

        Envelope {
            id,
            data: match (kind.as_str(), method.as_str()) {
                // chat service message functions
                #[cfg(features = "chat")]
                ("chat-message", "poll") => req(Request::ChatMsgNext(de_json(data, auth))),
                #[cfg(features = "chat")]
                ("chat-message", "subscribe") => req(Request::ChatMsgSub(de_json(data, auth))),
                #[cfg(features = "chat")]
                ("chat-message", "send") => req(Request::ChatMsgSend(de_json(data, auth))),
                // ("chat-message", "query") => req(Request::ChatMsgQuery(de_json(data, auth))),

                // chat service room management
                #[cfg(features = "chat")]
                ("chat-room", "list") => req(Request::ChatRoomList(de_json(data, auth))),
                #[cfg(features = "chat")]
                ("chat-room", "get") => req(Request::ChatRoomGet(de_json(data, auth))),
                #[cfg(features = "chat")]
                ("chat-room", "create") => req(Request::ChatRoomCreate(de_json(data, auth))),
                #[cfg(features = "chat")]
                ("chat-room", "modify") => req(Request::ChatRoomModify(de_json(data, auth))),
                #[cfg(features = "chat")]
                ("chat-room", "delete") => req(Request::ChatRoomDelete(de_json(data, auth))),

                // libqaul contact functions
                ("contact", "list") => req(Request::ContactAll(de_json(data, auth))),
                ("contact", "get") => req(Request::ContactGet(de_json(data, auth))),
                ("contact", "query") => req(Request::ContactQuery(de_json(data, auth))),
                ("contact", "modify") => req(Request::ContactQuery(de_json(data, auth))),

                // libqaul user functions
                ("users", "list") => req(Request::UserList(de_json(data, auth))),
                ("users", "create") => req(Request::UserCreate(de_json(data, auth))),
                ("users", "delete") => req(Request::UserDelete(de_json(data, auth))),
                ("users", "repass") => req(Request::UserChangePw(de_json(data, auth))),
                ("users", "login") => req(Request::UserLogin(de_json(data, auth))),
                ("users", "logout") => req(Request::UserLogout(de_json(data, auth))),
                ("users", "get") => req(Request::UserGet(de_json(data, auth))),
                ("users", "modify") => req(Request::UserUpdate(de_json(data, auth))),
                (_, _) => unreachable!(), // Replace with transmit error
            },
        }
    }
}

#[test]
#[cfg(features = "chat")]
fn envelope_chat_message_next() {
    // This re-uses the same ID for auth and room data not because
    // it's in any way significant but rather because it's 3am and I'm
    // being lazy
    let json = r#"{ "id": "1", 
                    "auth": { "id": "1C56 105D 52C3 D617  2603 D69F 9E0F 93AE", "token": "token" }, 
                    "kind": "chat-message", 
                    "method": "poll", 
                    "data": { "room": "1C56 105D 52C3 D617  2603 D69F 9E0F 93AE" } }"#;

    let je: RequestEnv = serde_json::from_str(&json).expect("JsonEnvelope failed");
    let env: Envelope = je.into();

    if let EnvelopeType::Request(Request::ChatMsgNext(msg)) = env.data {
        assert_eq!(
            msg.auth.0.to_string(),
            "1C56 105D 52C3 D617  2603 D69F 9E0F 93AE"
        );
    } else {
        panic!("Failed to deserialise correctly")
    }
}

/// This test checks if an empty body request can be handled
///
/// This pins the layout of User::List, which can't ever be a
/// zero-sized struct, but must have an empty body because serde_json
/// get's sad otherwise.
#[test]
fn envelope_chat_user_list() {
    let json = r#"{ "id": "1", 
                    "kind": "users", 
                    "method": "list" }"#;

    let je: RequestEnv = serde_json::from_str(&json).expect("JsonEnvelope failed");
    let _env: Envelope = je.into();
}
