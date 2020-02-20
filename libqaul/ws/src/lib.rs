//! libqaul websocket RPC
//!
//! The native interface for libqaul in async Rust.  But a few other
//! RPC interfaces are exposed via the libqaul-rpc collection.  One of
//! them is the websocket interface, which is primarily used by the
//! qaul.net webui.
//!
//! The structures are encoded in JSON, as described by the
//! libqaul-rpc structures.  Every request has an envelope, which
//! contains in ID and some data.  the data can either be a request or
//! a response, with appropriate data or error values inside.
//!
//! Because web devs are a bunch of pussies this crate also wraps the
//! envelope in a way that web devs will like, such as making the data
//! generic (a string) and pulling out the method names; things that
//! other rpc layers would hit you for but hey, it's 2020.

use libqaul::{users::UserAuth, Identity, Qaul};
use libqaul_rpc::{Envelope, EnvelopeType, QaulExt, QaulRpc, Request, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{self, json, Value as JsonValue};
use std::collections::BTreeMap;

type JsonMap = BTreeMap<String, JsonValue>;

#[derive(Debug, Serialize, Deserialize)]
struct JsonEnvelope {
    /// The request ID
    pub id: String,
    /// Auth data for the request
    pub auth: Option<JsonAuth>,
    /// Operation method
    pub method: String,
    /// Request scope
    pub kind: String,
    /// The rest of the nested data
    ///
    /// We keep this as a map because we need to inject the auth
    /// information into it later on, because the API expects it to be
    /// in eath RPC struct, while the json interface likes it in the
    /// envelope
    #[serde(default)]
    pub data: JsonMap,
}

/// A struct wrapper for UserAuth
#[derive(Debug, Serialize, Deserialize)]
struct JsonAuth {
    id: Identity,
    token: String,
}

impl From<JsonAuth> for UserAuth {
    fn from(ja: JsonAuth) -> Self {
        Self(ja.id, ja.token)
    }
}

#[derive(Debug)]
struct AuthInject<'env> {
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
        ("user", "list")
        | ("user", "login")
        | ("user", "create")
        | ("user", "get")
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

impl From<JsonEnvelope> for Envelope {
    fn from(je: JsonEnvelope) -> Self {
        let JsonEnvelope {
            id,
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
                ("chat-message", "poll") => req(Request::ChatMsgNext(de_json(data, auth))),
                ("chat-message", "subscribe") => req(Request::ChatMsgSub(de_json(data, auth))),
                ("chat-message", "send") => req(Request::ChatMsgSend(de_json(data, auth))),
                // ("chat-message", "query") => req(Request::ChatMsgQuery(de_json(data, auth))),

                // chat service room management
                ("chat-room", "list") => req(Request::ChatRoomList(de_json(data, auth))),
                ("chat-room", "get") => req(Request::ChatRoomGet(de_json(data, auth))),
                ("chat-room", "create") => req(Request::ChatRoomCreate(de_json(data, auth))),
                ("chat-room", "modify") => req(Request::ChatRoomModify(de_json(data, auth))),
                ("chat-room", "delete") => req(Request::ChatRoomDelete(de_json(data, auth))),

                // libqaul contact functions
                ("contact", "list") => req(Request::ContactAll(de_json(data, auth))),
                ("contact", "get") => req(Request::ContactGet(de_json(data, auth))),
                ("contact", "query") => req(Request::ContactQuery(de_json(data, auth))),
                ("contact", "modify") => req(Request::ContactQuery(de_json(data, auth))),

                // libqaul user functions
                ("user", "list") => req(Request::UserList(de_json(data, auth))),
                ("user", "create") => req(Request::UserCreate(de_json(data, auth))),
                ("user", "delete") => req(Request::UserDelete(de_json(data, auth))),
                ("user", "repass") => req(Request::UserChangePw(de_json(data, auth))),
                ("user", "login") => req(Request::UserLogin(de_json(data, auth))),
                ("user", "logout") => req(Request::UserLogout(de_json(data, auth))),
                ("user", "get") => req(Request::UserGet(de_json(data, auth))),
                ("user", "modify") => req(Request::UserUpdate(de_json(data, auth))),
                (_, _) => unreachable!(), // Replace with transmit error
            },
        }
    }
}

#[test]
fn envelope_chat_message_next() {
    let json = r#"{ "id": "1", 
                    "auth": { "id": "1C56 105D 52C3 D617  2603 D69F 9E0F 93AE", "token": "token" }, 
                    "kind": "chat-message", 
                    "method": "poll", 
                    "data": { "room": "1C56 105D 52C3 D617  2603 D69F 9E0F 93AE" } }"#;

    let je: JsonEnvelope = serde_json::from_str(&json).expect("JsonEnvelope failed");
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
                    "kind": "user", 
                    "method": "list" }"#;

    let je: JsonEnvelope = serde_json::from_str(&json).expect("JsonEnvelope failed");
    let env: Envelope = je.into();
}
