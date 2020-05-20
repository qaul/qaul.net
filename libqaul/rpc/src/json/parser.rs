use crate::{
    json::{JsonAuth, JsonMap, RequestEnv},
    Envelope, Request,
};
use libqaul::users::UserAuth;
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
fn de_json<'env, T: DeserializeOwned>(
    mut data: JsonMap,
    auth: AuthInject<'env>,
) -> Result<T, String> {
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
        .map_err(|_| format!("Failed to parse json data payload `{:#?}`", &data))
}

impl RequestEnv {
    pub fn generate_envelope(self) -> Result<Envelope<Request>, String> {
        let RequestEnv {
            id,
            page: _,
            method,
            kind,
            auth,
            data,
        } = self; // m'apelle janette

        let auth = AuthInject {
            auth,
            kind: &kind,
            method: &method,
        };

        Ok(Envelope {
            id,
            data: match (kind.as_str(), method.as_str()) {
                // chat service message functions
                #[cfg(feature = "chat")]
                ("chat-messages", "subscribe") => Request::ChatMsgSub(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat-messages", "create") => Request::ChatMsgCreate(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat-messages", "get") => Request::ChatLoadRoom(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat-rooms", "list") => Request::ChatRoomList(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat-rooms", "get") => Request::ChatRoomGet(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat-rooms", "create") => Request::ChatRoomCreate(de_json(data, auth)?),
                //#[cfg(feature = "chat")]
                //("chat-rooms", "modify") => Request::ChatRoomModify(de_json(data, auth)?),

                // libqaul contact functions
                ("contacts", "list") => Request::ContactAll(de_json(data, auth)?),
                ("contacts", "get") => Request::ContactGet(de_json(data, auth)?),
                ("contacts", "query") => Request::ContactQuery(de_json(data, auth)?),
                ("contacts", "modify") => Request::ContactQuery(de_json(data, auth)?),

                // libqaul user functions
                ("users", "list") => Request::UserList(de_json(data, auth)?),
                ("users", "create") => Request::UserCreate(de_json(data, auth)?),
                ("users", "delete") => Request::UserDelete(de_json(data, auth)?),
                ("users", "repass") => Request::UserChangePw(de_json(data, auth)?),
                ("users", "login") => Request::UserLogin(de_json(data, auth)?),
                ("users", "logout") => Request::UserLogout(de_json(data, auth)?),
                ("users", "get") => Request::UserGet(de_json(data, auth)?),
                ("users", "modify") => Request::UserUpdate(de_json(data, auth)?),
                (kind, method) => {
                    return Err(format!("Unknown parse tuple: ({}, {})", kind, method));
                }
            },
        })
    }
}

#[cfg(test)]
use serde_json::Value;

/// A utility to build json envelopes
///
/// This should maybe be rewritten to use the serde_json json! macro,
/// but it might not actually be any prettier, because most of the
/// ugliness of this comes from making sure we only insert data that
/// we want to have inserted.
///
/// I mean, okay look: this function is pretty well tested. IF it
/// starts acting up, maybe replace it.  Otherwise, probably don't
/// bother.  Don't judge me :P
///
/// In either case, it's probably a good idea never to compare the
/// actual strings though, always re-decode via serde_json.
#[cfg(test)]
fn json_builder(
    kind: &str,
    method: &str,
    auth: Option<UserAuth>,
    keys: Option<Vec<(&str, Value)>>,
) -> String {
    format!(
        r#"{{ "id": "1", "kind": "{kind}", "method": "{method}" {auth_block} {keys_block} }}"#,
        kind = kind,
        method = method,
        auth_block = match auth {
            Some(UserAuth(id, token)) => format!(
                r#", "auth": {}"#,
                serde_json::to_string(&JsonAuth { id, token }).unwrap()
            ),
            None => "".into(),
        },
        keys_block = match keys {
            Some(keys) => {
                let len = keys.len();
                format!(
                    r#" , "data": {{ {}  }} "#,
                    keys.into_iter()
                        .enumerate()
                        .fold(String::new(), |prev, (num, (k, v))| {
                            match num {
                                0 => format!(
                                    r#"{} "{}": {}"#,
                                    if len == 1 { "" } else { "," },
                                    k.to_owned(),
                                    v
                                ),
                                _ => format!(r#"{}, "{}": {}"#, prev, k, v),
                            }
                        })
                )
            }
            None => "".into(),
        }
    )
}

#[test]
fn test_builder() {
    let json: JsonMap = serde_json::from_str(&json_builder("users", "list", None, None)).unwrap();
    assert_eq!(json.get("kind").unwrap(), "users");
    assert_eq!(json.get("method").unwrap(), "list");
    assert_eq!(json.get("auth"), None);
}

#[test]
fn test_builder_with_auth() {
    use libqaul::Identity;
    use serde_json::map::Map;
    use std::collections::BTreeMap;

    let auth = UserAuth::test();
    let json: JsonMap =
        serde_json::from_str(&json_builder("users", "list", Some(auth.clone()), None)).unwrap();
    assert_eq!(json.get("kind").unwrap(), "users");
    assert_eq!(json.get("method").unwrap(), "list");
    assert_eq!(
        json.get("auth").unwrap(),
        &Value::Object({
            let mut map = Map::new();
            map.insert("id".into(), Value::String(auth.0.to_string()));
            map.insert("token".into(), Value::String(auth.1.clone()));
            map
        })
    );
}

/// This test checks if an empty body request can be handled
///
/// This pins the layout of User::List, which can't ever be a
/// zero-sized struct, but must have an empty body because serde_json
/// get's sad otherwise.
#[test]
fn envelope_chat_user_list() {
    let json = json_builder("users", "list", None, None);

    let je: RequestEnv = serde_json::from_str(&json).expect("JsonEnvelope failed");
    let env = je.generate_envelope().unwrap();
    assert_eq!(env.data, Request::UserList(crate::api::users::List {}));
}

#[test]
fn envelope_chat_room_create() {
    use libqaul::Identity;

    let friend = Identity::random();
    let auth = UserAuth::test();
    let json = json_builder(
        "chat-rooms",
        "create",
        Some(auth.clone()),
        Some(vec![(
            "users",
            Value::Array(vec![Value::String(friend.to_string())]),
        )]),
    );

    let je: RequestEnv = serde_json::from_str(&json).expect("JsonEnvelope failed");
    let env = je.generate_envelope().unwrap();

    assert_eq!(
        env.data,
        Request::ChatRoomCreate(crate::api::chat::rooms::Create {
            auth,
            users: vec![friend],
        })
    );
}

#[test]
fn envelope_chat_room_list() {
    use libqaul::Identity;

    let friend = Identity::random();
    let auth = UserAuth::test();
    let json = json_builder("chat-rooms", "list", Some(auth.clone()), None);

    let je: RequestEnv = serde_json::from_str(&json).expect("JsonEnvelope failed");
    let env = je.generate_envelope().unwrap();

    assert_eq!(
        env.data,
        Request::ChatRoomList(crate::api::chat::rooms::List { auth })
    );
}

#[test]
fn envelope_chat_room_get() {
    use libqaul::Identity;

    let room = Identity::random();
    let auth = UserAuth::test();
    let json = json_builder(
        "chat-rooms",
        "get",
        Some(auth.clone()),
        Some(vec![("id", Value::String(room.to_string()))]),
    );

    let je: RequestEnv = serde_json::from_str(&json).expect("JsonEnvelope failed");
    let env = je.generate_envelope().unwrap();

    assert_eq!(
        env.data,
        Request::ChatRoomGet(crate::api::chat::rooms::Get { auth, id: room })
    );
}
