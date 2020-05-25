//! RPC chat-messages tests

#[cfg(test)]
mod harness;
use harness::rpc_harness::RPC;
use std::time::Duration;


// chat-messages send and receive
#[async_std::test]
async fn rpc_chatmessages_create() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // create a chat room
    let room = rpc.responder_a.chat.start_chat(user_a.clone(), vec![user_b.0]).await.unwrap();

    // Send Message from user A
    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat-messages/create",
        "kind": "chat-messages",
        "method": "create",
        "data": {{
            "text": "hello world!",
            "room": "{room_id}"
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
    assert!(resp.data.contains_key("type"));
    assert_eq!(resp.data.get("type").unwrap(), "success");
}


// chat-messages send and receive
#[async_std::test]
async fn rpc_chatmessages_get() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user on each node
    let user_a = rpc.network.a().users().create("123456").await.unwrap();
    let user_b = rpc.network.b().users().create("123456").await.unwrap();

    // create a chat room
    let room = rpc.responder_a.chat.start_chat(user_a.clone(), vec![user_b.0]).await.unwrap();

    // Send Message from user A
    let msg = rpc.responder_a.chat
        .send_message(user_b.clone(), room.clone(), "hello world!".to_string())
        .await.unwrap();

    // wait until message is delivered
    async_std::task::sleep(Duration::from_secs(1)).await;

    // Receive Message at user B
    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/chat-messages/create",
        "kind": "chat-messages",
        "method": "create",
        "data": {{
            "text": "hello world!",
            "room": "{room_id}"
        }},
        "auth": {{
            "id": "{b_id}",
            "token": "{b_token}"
        }}
    }}"#,
        room_id = room,
        b_id = user_b.0,
        b_token = user_b.1
    );

    // send JSON
    let resp = rpc.send_b(json_string.as_str()).await;

    // check result
    dbg!(resp.clone());
}
