//! RPC chat tests

#[cfg(test)]
mod harness;
use harness::rpc_harness::RPC;

// chat-rooms create
#[async_std::test]
async fn rpc_chatrooms_create() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat-rooms/create",
        "kind": "chat-rooms",
        "method": "create",
        "data": {{
            "users": ["{friend}"]
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
    assert!(resp.data.contains_key("room_id"));
}

// chat-rooms get
#[async_std::test]
async fn rpc_chatrooms_get() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // create a chat room
    let room = rpc.responder_a.chat.start_chat(user_a.clone(), vec![user_b.0]).await.unwrap();

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat-rooms/list",
        "kind": "chat-rooms",
        "method": "get",
        "data": {{
            "id": "{room_id}"
        }},
        "auth": {{
            "id": "{a_id}",
            "token": "{a_token}"
        }}
    }}"#,
        room_id = room,
        a_id = user_a.0,
        a_token = user_a.1
    );

    // send JSON
    let resp = rpc.send_a(json_string.as_str()).await;

    // check result
    dbg!(resp.clone());
    assert!(resp.data.contains_key("room"));
    assert_eq!(
        String::from(resp.data.get("room").unwrap().get("id").unwrap().as_str().unwrap()),
        room.to_string()
    );
}

// chat-rooms modify
#[async_std::test]
async fn rpc_chatrooms_modify() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // create a chat room
    let room = rpc.responder_a.chat.start_chat(user_a.clone(), vec![user_b.0]).await.unwrap();

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat-rooms/list",
        "kind": "chat-rooms",
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
        room_id = room,
        a_id = user_a.0,
        a_token = user_a.1
    );

    // send JSON
    let resp = rpc.send_a(json_string.as_str()).await;

    // check result
    dbg!(resp.clone());
}

// chat-rooms list
#[async_std::test]
async fn rpc_chatrooms_list() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // create a chat room
    let room = rpc.responder_a.chat.start_chat(user_a.clone(), vec![user_b.0]).await.unwrap();

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat-rooms/list",
        "kind": "chat-rooms",
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
    assert!(resp.data.contains_key("rooms"));
    //assert!(resp.data["rooms"].as_array().unwrap().len() > 0);
}

// TODO: There is no way at the moment ot delete a chat room
//       or to leave a group chat.
