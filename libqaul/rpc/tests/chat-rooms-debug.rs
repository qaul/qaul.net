//! RPC Chat-Rooms Debugging tests
//! 
//! This file tests the RPC communication with data
//! that leads to crashes or unexpected behaviour.
//! The tests shall make it easier to track and fix these bugs.

#[cfg(test)]
mod harness;
use harness::rpc_harness::RPC;

// unauthorised and unexisting user tries to create a 
// chat-room.
#[async_std::test]
async fn rpc_chatrooms_list_unexisting_user() {
    // create RPC environment
    let rpc = RPC::init().await;

    // RPC JSON input
    let json_string = format!(
        r#"{{
            "id": "/chat-rooms/create",
            "kind": "chat-rooms",
            "method": "create",
            "data": {{
                "users": ["DD3C-2F99-09C3-3386-738A-B8B7-4575-2DAB-5F9A-F148-0056-FE61-A982-B8A2-0578-0E53"],
                "name": "Test Name"
            }},
                    "auth": {{
                "id": "6714-458D-B597-D05A-EA91-6D27-3BE1-FC90-912B-8F6C-DC33-2BED-E465-2410-FF92-A921",
                "token": "R4q3Vg12m12Ui1HchsM2e72tRPN5b3Dzm3vOUkcM8qRHG3OvjDdlnpHyXZKV3RrS-gRibrfjbHcLGLrgvnowRvS-jJiMEj_9ReIABojVb1GuR592EWlddnWGgQM76xs06SKxItFlHkMa4o1EmNF9q65rdTx1y02Pxb054Hp4JNux8Dz2lnoKWpXSt2PzPDl7EFaExh0Tozr6dhGfP0SC9RHu7U_90MlQLPS0Ub4cgZOdWPoD503yzE5S2pso2CxFqGPln38D487FFLYSPJjdNOVsajz9rCNtbzZvyY6GMcPbuGd0_yS9SIsXNI3vyaeaK3sn-IB2YdIjiDX8YOW6cg=="
            }}
        }}"#
    );

    // send JSON
    let resp = rpc.send_a(json_string.as_str()).await;

    dbg!(resp.clone());
}
