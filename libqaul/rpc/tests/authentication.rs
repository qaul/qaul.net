//! RPC authentication tests

#[cfg(test)]
mod harness;
use harness::rpc_harness::RPC;

// users create
#[async_std::test]
async fn rpc_authentication_login() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user
    let user_a = rpc.network.a().users().create("123456").await.unwrap();    

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/login",
        "kind": "users",
        "method": "login",
        "data": {{
            "id": "{a_id}",
            "pw": "123456"
        }}
        }}"#,
        a_id = user_a.0
    );

    // send JSON
    let resp = rpc.send_a(json_string.as_str()).await;

    // control return values
    dbg!(resp.clone());
    assert!(resp.data.contains_key("auth"));
}

// users modify
#[async_std::test]
async fn rpc_authentication_logout() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user
    let user_a = rpc.network.a().users().create("123456").await.unwrap();    

    // RPC JSON input
    let json_string = format!(
        r#"{{
        "id": "/logout",
        "kind": "users",
        "method": "logout",
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
