//! RPC Users Debugging tests
//! 
//! This file tests the RPC users communication with data
//! that leads to crashes or unexpected behaviour.
//! The tests shall make it easier to track and fix these bugs.

#[cfg(test)]
mod harness;
use harness::rpc_harness::RPC;

// modify a user and unset a value which hasn't been set
#[async_std::test]
async fn rpc_users_modify_with_unset_value() {
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
            "real_name": "unset"
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
    assert_eq!(
        network_a
            .users()
            .get(user_a.0)
            .await
            .unwrap()
            .display_name
            .unwrap(),
        "my_username"
    );
}

