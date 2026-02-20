use crate::{
    cli::UsersSubcmd,
    commands::RpcCommand,
    proto::{
        users, ConnectionModule, Connectivity, GetUserByIdRequest, Modules, SecurityNumberRequest,
        UserEntry, UserOnlineRequest, UserRequest, Users,
    },
};
use prost::Message;
use uuid::Uuid;

fn send_user_update(
    user_id_base58: &str,
    verified: bool,
    blocked: bool,
) -> Result<Users, Box<dyn std::error::Error>> {
    let user_id = bs58::decode(user_id_base58).into_vec().unwrap();

    // create request message
    let proto_message = Users {
        message: Some(users::Message::UserUpdate(UserEntry {
            name: String::from(""),
            id: user_id,
            key_base58: String::from(""),
            group_id: Vec::new(),
            connectivity: 0,
            verified,
            blocked,
            connections: vec![],
        })),
    };

    Ok(proto_message)
}

impl RpcCommand for UsersSubcmd {
    fn expects_response(&self) -> bool {
        match &self {
            UsersSubcmd::Verify { .. } | UsersSubcmd::Block { .. } => false,
            _ => true,
        }
    }

    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        match &self {
            UsersSubcmd::List => {
                let proto_message = Users {
                    message: Some(users::Message::UserRequest(UserRequest {
                        offset: 0,
                        limit: 0,
                    })),
                };

                // encode message
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Users))
            }
            UsersSubcmd::Online => {
                let proto_message = Users {
                    message: Some(users::Message::UserOnlineRequest(UserOnlineRequest {
                        offset: 0,
                        limit: 0,
                    })),
                };

                // encode message
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Users))
            }
            UsersSubcmd::Verify { user_id } => {
                let proto_message = send_user_update(&user_id, true, false)?;
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Users))
            }
            UsersSubcmd::Block { user_id } => {
                let proto_message = send_user_update(&user_id, false, true)?;
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Users))
            }
            UsersSubcmd::Secure { .. } => {
                return Err(
                    "secure requires an authenticated session, which is not yet supported".into(),
                );
                // let id = bs58::decode(&user_id).into_vec()?;

                // let proto_message = Users {
                //     message: Some(users::Message::SecurityNumberRequest(
                //         SecurityNumberRequest { user_id: id },
                //     )),
                // };

                // // encode message
                // let mut buf = Vec::with_capacity(proto_message.encoded_len());
                // proto_message
                //     .encode(&mut buf)
                //     .expect("Vec<u8> provides capacity as needed");

                // Ok((buf, Modules::Users))
            }
            UsersSubcmd::Get { user_id } => {
                let id = bs58::decode(&user_id).into_vec()?;

                let proto_message = Users {
                    message: Some(users::Message::GetUserByIdRequest(GetUserByIdRequest {
                        user_id: id,
                    })),
                };

                // encode message
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Users))
            }
        }
    }

    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let users = Users::decode(data)?;
        match users.message {
            Some(users::Message::UserList(user_list)) => {
                let mut line = 1;
                println!("");
                println!("All known Users");
                println!("No. | User Name | User Id | Verified | Blocked | Connectivity");
                println!("    | Group ID | Public Key");

                for user in user_list.user {
                    let mut verified = "N";
                    let mut blocked = "N";
                    let onlined = Connectivity::try_from(user.connectivity)
                        .unwrap_or(Connectivity::Offline)
                        .as_str_name();

                    if user.verified {
                        verified = "Y";
                    }
                    if user.blocked {
                        blocked = "Y";
                    }
                    println!(
                        "{} | {} | {:?} | {} | {} | {}",
                        line,
                        user.name,
                        bs58::encode(user.id).into_string(),
                        verified,
                        blocked,
                        onlined
                    );
                    let group_uuid;
                    match Uuid::from_slice(&user.group_id) {
                        Ok(uuid) => {
                            group_uuid = uuid;
                            println!(
                                "   | {} | {}",
                                group_uuid.hyphenated().to_string(),
                                user.key_base58
                            );
                        }
                        Err(e) => log::error!("{}", e),
                    }
                    if user.connections.len() > 0 {
                        println!("  Connections: module | hc | rtt | via");
                        for cnn in user.connections {
                            let module = ConnectionModule::try_from(cnn.module)
                                .unwrap()
                                .as_str_name();
                            println!(
                                "      {} | {} | {} | {}",
                                module,
                                cnn.hop_count,
                                cnn.rtt,
                                bs58::encode(cnn.via.clone()).into_string()
                            );
                        }
                    }
                    line += 1;
                }

                println!("");
            }
            Some(users::Message::GetUserByIdResponse(resp)) => match resp.user {
                Some(user) => {
                    let mut verified = "N";
                    let mut blocked = "N";
                    let onlined = Connectivity::try_from(user.connectivity)
                        .unwrap_or(Connectivity::Offline)
                        .as_str_name();

                    if user.verified {
                        verified = "Y";
                    }
                    if user.blocked {
                        blocked = "Y";
                    }

                    println!("");
                    println!("User Info");
                    println!("Name: {}", user.name);
                    println!("ID: {}", bs58::encode(&user.id).into_string());
                    println!(
                        "Verified: {} | Blocked: {} | Connectivity: {}",
                        verified, blocked, onlined
                    );

                    match Uuid::from_slice(&user.group_id) {
                        Ok(uuid) => {
                            println!("Group ID: {}", uuid.hyphenated().to_string());
                        }
                        Err(e) => log::error!("{}", e),
                    }

                    println!("Public Key: {}", user.key_base58);

                    if user.connections.len() > 0 {
                        println!("Connections: module | hc | rtt | via");
                        for cnn in user.connections {
                            let module = ConnectionModule::try_from(cnn.module)
                                .unwrap()
                                .as_str_name();
                            println!(
                                "  {} | {} | {} | {}",
                                module,
                                cnn.hop_count,
                                cnn.rtt,
                                bs58::encode(cnn.via.clone()).into_string()
                            );
                        }
                    }
                    println!("");
                }
                None => {
                    println!("User not found.");
                }
            },
            Some(users::Message::SecurityNumberResponse(resp)) => {
                println!("Security Number:");
                let mut counter = 0;
                for number in resp.security_number_blocks {
                    print!("{:#05} ", number);
                    if counter == 3 {
                        println!("");
                    }
                    counter = counter + 1;
                }
                println!("");
            }
            _ => {
                log::error!("unprocessable RPC users message");
            }
        };
        Ok(())
    }
}
