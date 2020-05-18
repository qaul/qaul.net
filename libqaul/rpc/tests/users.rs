//! RPC users tests

#[cfg(test)]
mod harness;
use harness::rpc_harness::RPC;

// users create
#[async_std::test]
async fn rpc_users_create() {
    // create RPC environment
    let rpc = RPC::init().await;

    // RPC JSON input
    let json_str = r#"{
        "id": "/users/create",
        "kind": "users",
        "method": "create",
        "data": {
            "pw": "123456"
        }
    }"#;

    // send JSON
    let resp = rpc.send_a(json_str).await;

    // control return values
    dbg!(resp.clone());
    assert!(resp.data.contains_key("auth"));
}

// users list
#[async_std::test]
async fn rpc_users_list() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user
    let user_a = rpc.network.a().users().create("123456").await.unwrap();

    // RPC JSON input
    let json_str = r#"{
        "id": "/users/list",
        "kind": "users",
        "method": "list"
    }"#;

    // send JSON
    let resp = rpc.send_a(json_str).await;

    // control return values
    dbg!(resp.clone());
    assert!(resp.data.contains_key("user"));
    assert_eq!(
        resp.data.get("user").unwrap().get(0).unwrap().get("id").unwrap().to_string(), 
        serde_json::to_string(&user_a.0).unwrap()
    );
}

// users modify
#[async_std::test]
async fn rpc_users_modify() {
    // create RPC environment
    let rpc = RPC::init().await;
    let network_a = rpc.network.a().clone();

    // create a user
    let user_a = rpc.network.a().users().create("123456").await.unwrap();    

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/users/modify",
        "kind": "users",
        "method": "modify",
        "data": {{
            "display_name": {{
                "set": "my_username"
            }},
            "real_name": {{
                "set": "My Username"
            }}
        }},
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

    dbg!(resp.clone());
    assert!(resp.data.contains_key("type"));
    assert_eq!(resp.data.get("type").unwrap(), "success");
    assert_eq!(network_a.users().get(user_a.0).await.unwrap().display_name.unwrap(), "my_username");
}

// users delete
#[async_std::test]
async fn rpc_users_delete() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user
    let user_a = rpc.network.a().users().create("123456").await.unwrap();    

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/users/delete",
        "kind": "users",
        "method": "delete",
        "data": {{
            "purge": true
        }},
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

    dbg!(resp.clone());
    assert!(resp.data.contains_key("type"));
    assert_eq!(resp.data.get("type").unwrap(), "success");
}
