use super::{id_string_to_bin, uuid_string_to_bin};
use crate::{cli::GroupSubcmd, commands::RpcCommand, proto::Modules};
use uuid::Uuid;

use qaul_proto::qaul_rpc_chat as proto_chat;
/// protobuf RPC definition
use qaul_proto::qaul_rpc_group as proto;

use prost::Message;
use proto::{
    group, Group, GroupCreateRequest, GroupInfoRequest, GroupInviteMemberRequest,
    GroupInvitedRequest, GroupListRequest, GroupMemberRole, GroupMemberState,
    GroupRemoveMemberRequest, GroupRenameRequest, GroupReplyInviteRequest, GroupStatus,
};
use proto_chat::{chat_content_message, ChatContentMessage, GroupEventType};

impl GroupSubcmd {
    /// reply invite
    fn reply_to_invite(
        &self,
        group_id: Vec<u8>,
        accept: bool,
    ) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        // group invite send message
        let proto_message = Group {
            message: Some(group::Message::GroupReplyInviteRequest(
                GroupReplyInviteRequest {
                    group_id: group_id.clone(),
                    accept,
                },
            )),
        };

        Ok((proto_message.encode_to_vec(), Modules::Group))
    }

    /// Process the last message & print it's content
    fn print_last_message(&self, data: Vec<u8>) {
        if let Ok(content_message) = ChatContentMessage::decode(&data[..]) {
            match content_message.message {
                Some(chat_content_message::Message::ChatContent(chat_content)) => {
                    println!("\t\t{}", chat_content.text);
                }
                Some(chat_content_message::Message::FileContent(file_content)) => {
                    println!(
                        "\t\tfile {} [{}] ID {}",
                        file_content.file_name,
                        file_content.file_size.to_string(),
                        file_content.file_id.to_string()
                    );
                    println!("\t\t{}", file_content.file_description);
                }
                Some(chat_content_message::Message::GroupEvent(group_event)) => {
                    match GroupEventType::try_from(group_event.event_type) {
                        Ok(GroupEventType::Joined) => {
                            println!(
                                "\t\tNew user joined group, user id: {}",
                                bs58::encode(group_event.user_id).into_string()
                            );
                        }
                        Ok(GroupEventType::Left) => {
                            println!(
                                "\t\tUser left group, user id: {}",
                                bs58::encode(group_event.user_id).into_string()
                            );
                        }
                        Ok(GroupEventType::Removed) => {
                            println!("\t\tYou have been removed from this group.");
                        }
                        Ok(GroupEventType::Created) => {
                            println!("\t\tYou created this group");
                        }
                        Ok(GroupEventType::InviteAccepted) => {
                            println!("\t\tYou accepted the invitation");
                        }
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                None => {}
            }
        }
    }
}

impl RpcCommand for GroupSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        match &self {
            GroupSubcmd::Create { name } => {
                // create group send message
                let proto_message = Group {
                    message: Some(group::Message::GroupCreateRequest(GroupCreateRequest {
                        group_name: name.clone(),
                    })),
                };

                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
            GroupSubcmd::Rename { group_id, name } => {
                let group_id = uuid_string_to_bin(group_id.to_string())?;
                let proto_message = Group {
                    message: Some(group::Message::GroupRenameRequest(GroupRenameRequest {
                        group_name: name.clone(),
                        group_id: group_id.clone(),
                    })),
                };

                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
            GroupSubcmd::Info { id } => {
                let group_id = uuid_string_to_bin(id.to_string())?;

                let proto_message = Group {
                    message: Some(group::Message::GroupInfoRequest(GroupInfoRequest {
                        group_id: group_id.clone(),
                    })),
                };

                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
            GroupSubcmd::List => {
                let proto_message = Group {
                    message: Some(group::Message::GroupListRequest(GroupListRequest {
                        offset: 0,
                        limit: 0,
                    })),
                };

                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Group))
            }
            GroupSubcmd::Invited => {
                let proto_message = Group {
                    message: Some(group::Message::GroupInvitedRequest(GroupInvitedRequest {
                        offset: 0,
                        limit: 0,
                    })),
                };

                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
            GroupSubcmd::Invite { group_id, user_id } => {
                let group_id = uuid_string_to_bin(group_id.to_string())?;
                let user_id = id_string_to_bin(user_id.to_string())?;

                let proto_message = Group {
                    message: Some(group::Message::GroupInviteMemberRequest(
                        GroupInviteMemberRequest {
                            group_id: group_id.clone(),
                            user_id: user_id.clone(),
                        },
                    )),
                };

                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
            GroupSubcmd::Accept { group_id } => {
                let group_id = uuid_string_to_bin(group_id.to_string())?;
                self.reply_to_invite(group_id, true)
            }
            GroupSubcmd::Decline { group_id } => {
                let group_id = uuid_string_to_bin(group_id.to_string())?;
                self.reply_to_invite(group_id, false)
            }
            GroupSubcmd::Remove { group_id, user_id } => {
                let group_id = uuid_string_to_bin(group_id.to_string())?;
                let user_id = id_string_to_bin(user_id.to_string())?;

                let proto_message = Group {
                    message: Some(group::Message::GroupRemoveMemberRequest(
                        GroupRemoveMemberRequest {
                            group_id: group_id.clone(),
                            user_id: user_id.clone(),
                        },
                    )),
                };
                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
        }
    }
    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        let group = Group::decode(data)?;
        match group.message {
            Some(group::Message::GroupCreateResponse(r)) => {
                let group_id_bytes: [u8; 16] = match r.group_id.try_into() {
                    Ok(b) => b,
                    Err(e) => {
                        log::error!("invalid group id bytes: {:?}", e);
                        return Ok(());
                    }
                };
                let group_id = Uuid::from_bytes(group_id_bytes);
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "group_id": group_id.to_string(),
                        }))?
                    );
                } else {
                    println!("====================================");
                    println!("Group was created or updated");
                    println!("\tid: {}", group_id.to_string());
                }
            }
            Some(group::Message::GroupRenameResponse(r)) => {
                let result = match r.result {
                    Some(r) => r,
                    None => {
                        log::error!("missing result in rename response");
                        return Ok(());
                    }
                };
                let group_id_bytes: [u8; 16] = match r.group_id.try_into() {
                    Ok(b) => b,
                    Err(e) => {
                        log::error!("invalid group id bytes: {:?}", e);
                        return Ok(());
                    }
                };
                let group_id = Uuid::from_bytes(group_id_bytes);
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "group_id": group_id.to_string(),
                            "success": result.status,
                            "error": result.message,
                        }))?
                    );
                } else {
                    println!("====================================");
                    println!("Group Rename status: {}", result.status);
                    println!("\tid: {}", group_id.to_string());
                    if !result.status {
                        println!("\terror: {}", result.message);
                    }
                }
            }
            Some(group::Message::GroupInviteMemberResponse(r)) => {
                let result = match r.result {
                    Some(r) => r,
                    None => {
                        log::error!("missing result in invite response");
                        return Ok(());
                    }
                };
                let group_id_bytes: [u8; 16] = match r.group_id.try_into() {
                    Ok(b) => b,
                    Err(e) => {
                        log::error!("invalid group id bytes: {:?}", e);
                        return Ok(());
                    }
                };
                let group_id = Uuid::from_bytes(group_id_bytes);
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "group_id": group_id.to_string(),
                            "success": result.status,
                            "error": result.message,
                        }))?
                    );
                } else {
                    println!("====================================");
                    println!("Group Invite status: {}", result.status);
                    println!("\tid: {}", group_id.to_string());
                    if !result.status {
                        println!("\terror: {}", result.message);
                    }
                }
            }
            Some(group::Message::GroupReplyInviteResponse(r)) => {
                let result = match r.result {
                    Some(r) => r,
                    None => {
                        log::error!("missing result in reply invite response");
                        return Ok(());
                    }
                };
                let group_id_bytes: [u8; 16] = match r.group_id.try_into() {
                    Ok(b) => b,
                    Err(e) => {
                        log::error!("invalid group id bytes: {:?}", e);
                        return Ok(());
                    }
                };
                let group_id = Uuid::from_bytes(group_id_bytes);
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "group_id": group_id.to_string(),
                            "success": result.status,
                            "error": result.message,
                        }))?
                    );
                } else {
                    println!("====================================");
                    println!("Reply Group Invite status: {}", result.status);
                    println!("\tid: {}", group_id.to_string());
                    if !result.status {
                        println!("\terror: {}", result.message);
                    }
                }
            }
            Some(group::Message::GroupRemoveMemberResponse(r)) => {
                let result = match r.result {
                    Some(r) => r,
                    None => {
                        log::error!("missing result in remove member response");
                        return Ok(());
                    }
                };
                let group_id_bytes: [u8; 16] = match r.group_id.try_into() {
                    Ok(b) => b,
                    Err(e) => {
                        log::error!("invalid group id bytes: {:?}", e);
                        return Ok(());
                    }
                };
                let group_id = Uuid::from_bytes(group_id_bytes);
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "group_id": group_id.to_string(),
                            "success": result.status,
                            "error": result.message,
                        }))?
                    );
                } else {
                    println!("====================================");
                    println!("Group Remove Member status: {}", result.status);
                    println!("\tid: {}", group_id.to_string());
                    if !result.status {
                        println!("\terror: {}", result.message);
                    }
                }
            }
            Some(group::Message::GroupInfoResponse(r)) => {
                let group_id_bytes: [u8; 16] = match r.group_id.try_into() {
                    Ok(b) => b,
                    Err(e) => {
                        log::error!("invalid group id bytes: {:?}", e);
                        return Ok(());
                    }
                };
                let group_id = Uuid::from_bytes(group_id_bytes);
                if json {
                    let members: Vec<serde_json::Value> = r.members.iter().map(|m| {
                        serde_json::json!({
                            "user_id": bs58::encode(&m.user_id).into_string(),
                            "state": GroupMemberState::try_from(m.state).map(|s| format!("{:?}", s)).unwrap_or_default(),
                            "role": GroupMemberRole::try_from(m.role).map(|r| format!("{:?}", r)).unwrap_or_default(),
                            "last_message_index": m.last_message_index,
                        })
                    }).collect();
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "group_id": group_id.to_string(),
                            "name": r.group_name,
                            "created_at": r.created_at,
                            "members": members,
                        }))?
                    );
                } else {
                    println!("====================================");
                    println!("Group Information");
                    println!("\tid: {}", group_id.to_string());
                    println!("\tname: {}", r.group_name);
                    println!("\tcreated_at: {}", r.created_at);
                    println!("\tmembers: {}", r.members.len());
                }
            }
            Some(group::Message::GroupListResponse(r)) => {
                if json {
                    let groups: Vec<serde_json::Value> = r.groups.iter().map(|group| {
                        let group_id = uuid::Uuid::from_bytes(group.group_id.clone().try_into().unwrap());
                        let status = GroupStatus::try_from(group.status)
                            .map(|s| format!("{:?}", s))
                            .unwrap_or_default();
                        let members: Vec<serde_json::Value> = group.members.iter().map(|m| {
                            serde_json::json!({
                                "user_id": bs58::encode(&m.user_id).into_string(),
                                "state": GroupMemberState::try_from(m.state).map(|s| format!("{:?}", s)).unwrap_or_default(),
                                "role": GroupMemberRole::try_from(m.role).map(|r| format!("{:?}", r)).unwrap_or_default(),
                                "last_message_index": m.last_message_index,
                            })
                        }).collect();
                        serde_json::json!({
                            "group_id": group_id.to_string(),
                            "name": group.group_name,
                            "is_direct_chat": group.is_direct_chat,
                            "status": status,
                            "created_at": group.created_at,
                            "revision": group.revision,
                            "unread_messages": group.unread_messages,
                            "last_message_at": group.last_message_at,
                            "last_message_sender_id": bs58::encode(&group.last_message_sender_id).into_string(),
                            "members": members,
                        })
                    }).collect();
                    println!("{}", serde_json::to_string_pretty(&groups)?);
                } else {
                    println!("=============List Of Groups=================");
                    for group in r.groups {
                        let group_id = uuid::Uuid::from_bytes(group.group_id.try_into().unwrap());
                        let group_type = if group.is_direct_chat {
                            "Direct"
                        } else {
                            "Group"
                        };
                        println!(
                            "{} {} {}",
                            group_type,
                            group_id.to_string(),
                            group.group_name
                        );
                        print!("\tstatus: ");
                        match GroupStatus::try_from(group.status) {
                            Ok(GroupStatus::Active) => println!("Active"),
                            Ok(GroupStatus::InviteAccepted) => println!("Invite Accepted"),
                            Ok(GroupStatus::Deactivated) => println!("Deactivated"),
                            Err(_) => println!("NOT SET"),
                        }
                        println!(
                            "\tcreated_at: {}, members: {}",
                            group.created_at,
                            group.members.len()
                        );
                        for member in group.members {
                            print!(
                                "\t\t id: {} , state: ",
                                bs58::encode(member.user_id.clone()).into_string()
                            );
                            match GroupMemberState::try_from(member.state) {
                                Ok(GroupMemberState::Invited) => print!("invited , role: "),
                                Ok(GroupMemberState::Activated) => print!("activated , role: "),
                                Err(_) => {}
                            }
                            match GroupMemberRole::try_from(member.role) {
                                Ok(GroupMemberRole::User) => {
                                    println!("user , sent: {}", member.last_message_index)
                                }
                                Ok(GroupMemberRole::Admin) => {
                                    println!("admin , sent: {}", member.last_message_index)
                                }
                                Err(_) => {}
                            }
                        }
                        println!("\trevision: {}", group.revision);
                        println!("\tunread messages: {}", group.unread_messages);
                        println!("\tlast message:");
                        println!(
                            "\t\tsent_at: {} from: {}",
                            group.last_message_at,
                            bs58::encode(group.last_message_sender_id).into_string()
                        );
                        self.print_last_message(group.last_message);
                    }
                }
            }
            Some(group::Message::GroupInvitedResponse(r)) => {
                if json {
                    let invited: Vec<serde_json::Value> = r
                        .invited
                        .iter()
                        .filter_map(|invite| {
                            invite.group.as_ref().map(|group| {
                                let group_id = uuid::Uuid::from_bytes(
                                    group.group_id.clone().try_into().unwrap(),
                                );
                                serde_json::json!({
                                    "group_id": group_id.to_string(),
                                    "name": group.group_name,
                                    "sender_id": bs58::encode(&invite.sender_id).into_string(),
                                    "received_at": invite.received_at,
                                    "member_count": group.members.len(),
                                })
                            })
                        })
                        .collect();
                    println!("{}", serde_json::to_string_pretty(&invited)?);
                } else {
                    println!("=============List Of Invited=================");
                    for invite in r.invited {
                        if let Some(group) = invite.group {
                            let group_id =
                                uuid::Uuid::from_bytes(group.group_id.try_into().unwrap());
                            println!("id: {}", group_id.to_string());
                            println!("\tname: {}", group.group_name);
                            println!("\tsender: {}", bs58::encode(invite.sender_id).into_string());
                            println!("\treceived at: {}", invite.received_at);
                            println!(
                                "\tcreated_at: {}, members: {}",
                                invite.received_at,
                                group.members.len()
                            );
                        }
                    }
                }
            }
            _ => {
                log::error!("unprocessable RPC group chat message");
            }
        };
        Ok(())
    }
}
