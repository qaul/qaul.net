//! RPC users tests

#[cfg(test)]
mod tests;
use tests::tests::RPC;

// users create
#[async_std::test]
async fn rpc_users_create() {
    // RPC JSON input
    let json_str = r#"{
        "id": "/users/create",
        "kind": "users",
        "method": "create",
        "data": {
            "pw": "123456"
        }
    }"#;

    // create RPC environment
    let rpc = RPC::init().await;
    let resp = rpc.send(json_str).await;

    assert!(resp.data.contains_key("auth"));
}

// users list
#[async_std::test]
async fn rpc_users_list() {
    // RPC JSON input
    let json_str = r#"{
        "id": "/users/list",
        "kind": "users",
        "method": "list"
    }"#;

    // create RPC environment
    let rpc = RPC::init().await;
    let resp = rpc.send(json_str).await;

    assert!(resp.data.contains_key("user"));
}

// users modify
#[async_std::test]
async fn rpc_users_modify() {
    // create RPC environment
    let rpc = RPC::init().await;

    // create a user
    

    // RPC JSON input
    let json_str = r#"{
        "id": "/users/list",
        "kind": "users",
        "method": "list"
    }"#;

    // send JSON
    let resp = rpc.send(json_str).await;

    assert!(resp.data.contains_key("user"));
}
