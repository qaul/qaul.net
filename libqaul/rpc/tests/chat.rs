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
    let json_string = format!("{{
        \"id\": \"/chat-rooms/create\",
        \"kind\": \"chat-rooms\",
        \"method\": \"create\",
        \"data\": {{
            \"users\": [\"{}\",\"{}\"]
        }},
        \"auth\": {{
            \"id\": \"{}\",
            \"token\": \"{}\"
        }}
    }}", user_a.0, user_b.0, user_a.0, user_b.1);

    // create a chat room
    let resp = rpc.send_a(json_string.as_str()).await;
}
