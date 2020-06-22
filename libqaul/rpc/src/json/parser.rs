use crate::{
    json::{JsonAuth, JsonMap, RequestEnv},
    Envelope, Request,
};
use libqaul::{helpers::ItemDiff, users::UserAuth};
use serde::de::DeserializeOwned;
use serde_json::json;

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
        ("user", "list")
        | ("user", "login")
        | ("user", "create")
        | ("user", "get")
        | ("file", "list") => {}
        (_, _) => {
            data.insert(
                "auth".into(),
                serde_json::to_value(UserAuth::from(auth.auth.unwrap()))
                    .expect("Failed to inject auth"),
            );
        }
    };

    serde_json::from_value(dbg!(serde_json::to_value(&data).unwrap())).map_err(|e| {
        format!(
            "Failed to parse json data payload `{:#?}`. Error: {}",
            &data, e
        )
    })
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
                ("chat_message", "subscribe") => Request::ChatMsgSub(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat_message", "create") => Request::ChatMsgCreate(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat_message", "query") => Request::ChatLoadRoom(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat_room", "list") => Request::ChatRoomList(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat_room", "get") => Request::ChatRoomGet(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat_room", "create") => Request::ChatRoomCreate(de_json(data, auth)?),
                #[cfg(feature = "chat")]
                ("chat_room", "modify") => Request::ChatRoomModify(de_json(data, auth)?),

                // libqaul contact functions
                ("contact", "list") => Request::UserListRemote(de_json(data, auth)?),
                // ("contact", "get") => Request::ContactGet(de_json(data, auth)?),
                // ("contact", "query") => Request::ContactQuery(de_json(data, auth)?),
                // ("contact", "modify") => Request::ContactQuery(de_json(data, auth)?),

                // libqaul user functions
                ("user", "list") => Request::UserList(de_json(data, auth)?),
                ("user", "create") => Request::UserCreate(de_json(data, auth)?),
                ("user", "delete") => Request::UserDelete(de_json(data, auth)?),
                ("user", "repass") => Request::UserChangePw(de_json(data, auth)?),
                ("user", "login") => Request::UserLogin(de_json(data, auth)?),
                ("user", "logout") => Request::UserLogout(de_json(data, auth)?),
                ("user", "validate") => Request::UserIsAuthenticated(de_json(data, auth)?),
                ("user", "get") => Request::UserGet(de_json(data, auth)?),
                ("user", "modify") => Request::UserUpdate(de_json(data, auth)?),
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
                                0 => format!(r#""{}": {}"#, k.to_owned(), v),
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
    let json: JsonMap = serde_json::from_str(&json_builder("user", "list", None, None)).unwrap();
    assert_eq!(json.get("kind").unwrap(), "user");
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
        serde_json::from_str(&json_builder("user", "list", Some(auth.clone()), None)).unwrap();
    assert_eq!(json.get("kind").unwrap(), "user");
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
    let json = json_builder("user", "list", None, None);

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
        "chat_room",
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
            name: None,
        })
    );
}

#[test]
fn envelope_chat_room_list() {
    use libqaul::Identity;

    let friend = Identity::random();
    let auth = UserAuth::test();
    let json = json_builder("chat_room", "list", Some(auth.clone()), None);

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
        "chat_room",
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

#[test]
fn envelope_chat_room_modify() {
    use libqaul::Identity;

    let room = Identity::random();
    let auth = UserAuth::test();
    let json = json_builder(
        "chat_room",
        "modify",
        Some(auth.clone()),
        Some(vec![
            ("id", Value::String(room.to_string())),
            ("name", json!({ "set": "Cool Name" })),
        ]),
    );

    println!("{}", json);

    let je: RequestEnv = serde_json::from_str(&json).expect("JsonEnvelope failed");
    let env = je.generate_envelope().unwrap();

    assert_eq!(
        env.data,
        Request::ChatRoomModify(crate::api::chat::rooms::Modify {
            auth,
            id: room,
            users: vec![],
            name: ItemDiff::Set("Cool Name".into())
        })
    );
}
