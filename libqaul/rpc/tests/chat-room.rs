//! RPC chat room tests

#[cfg(test)]
mod harness;
use harness::rpc_harness::RPC;

// chat_room create
#[async_std::test]
async fn rpc_chatRoom_create() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat_room/create",
        "kind": "chat_room",
        "method": "create",
        "data": {{
            "users": ["{friend}"],
            "name": "Test Name"
        }},
        "auth": {{
            "id": "{a_id}",
            "token": "{a_token}"
        }}
    }}"#,
        friend = user_b.0,
        a_id = user_a.0,
        a_token = user_a.1
    );

    // create a chat room
    let resp = rpc.send_a(json_string.as_str()).await;

    // check result
    dbg!(resp.clone());
    assert!(resp.data.contains_key("chat_room"));
}

// chat_room get
#[async_std::test]
async fn rpc_chatRoom_get() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // create a chat room
    let room = rpc
        .responder_a
        .chat
        .start_chat(user_a.clone(), vec![user_b.0], None)
        .await
        .unwrap();

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "1",
        "kind": "chat_room",
        "method": "get",
        "data": {{
            "id": "{room_id}"
        }},
        "auth": {{
            "id": "{a_id}",
            "token": "{a_token}"
        }}
    }}"#,
        room_id = room.id,
        a_id = user_a.0,
        a_token = user_a.1
    );

    // send JSON
    let resp = rpc.send_a(json_string.as_str()).await;

    // check result
    dbg!(resp.clone());
    assert!(resp.data.contains_key("chat_room"));
    assert_eq!(
        String::from(
            resp.data
                .get("chat_room")
                .unwrap()
                .get("id")
                .unwrap()
                .as_str()
                .unwrap()
        ),
        room.id.to_string()
    );
}

// chat_room modify
#[async_std::test]
async fn rpc_chat_room_modify() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // create a chat room
    let room = rpc
        .responder_a
        .chat
        .start_chat(user_a.clone(), vec![user_b.0], None)
        .await
        .unwrap();

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat_room/modify",
        "kind": "chat_room",
        "method": "modify",
        "data": {{
            "id": "{room_id}",
            "name": {{
                "set": "Test Name"
            }}
        }},
        "auth": {{
            "id": "{a_id}",
            "token": "{a_token}"
        }}
    }}"#,
        room_id = room.id,
        a_id = user_a.0,
        a_token = user_a.1
    );

    // send JSON
    let resp = rpc.send_a(json_string.as_str()).await;

    // check result
    dbg!(resp.clone());
}

// chat_room list
#[async_std::test]
async fn rpc_chat_room_list() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // create a chat room
    let room = rpc
        .responder_a
        .chat
        .start_chat(user_a.clone(), vec![user_b.0], None)
        .await
        .unwrap();

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat_room/list",
        "kind": "chat_room",
        "method": "list",
        "auth": {{
            "id": "{a_id}",
            "token": "{a_token}"
        }}
    }}"#,
        a_id = user_a.0,
        a_token = user_a.1
    );

    // send JSON
    let resp = rpc.send_a(json_string.as_str()).await;

    // check result
    dbg!(resp.clone());
    assert!(resp.data.contains_key("chat_rooms"));
    assert!(resp.data["chat_rooms"].as_array().unwrap().len() > 0);
}

// // TODO: There is no way at the moment ot delete a chat room
// //       or to leave a group chat.
