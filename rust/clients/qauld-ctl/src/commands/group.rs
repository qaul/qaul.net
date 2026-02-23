use crate::{
    cli::GroupSubcmd,
    commands::RpcCommand,
    proto::{
        chat_content_message, group, ChatContentMessage, Group, GroupCreateRequest, GroupEventType,
        GroupInfoRequest, GroupInviteMemberRequest, GroupInvitedRequest, GroupListRequest,
        GroupMemberRole, GroupMemberState, GroupRemoveMemberRequest, GroupRenameRequest,
        GroupReplyInviteRequest, GroupStatus, Modules,
    },
};
use prost::Message;
use std::fmt;
use uuid::Uuid;

impl GroupSubcmd {
    /// Convert Group ID from String to Binary
    fn uuid_string_to_bin(&self, id_str: String) -> Result<Vec<u8>, String> {
        match Uuid::parse_str(id_str.as_str()) {
            Ok(id) => Ok(id.as_bytes().to_vec()),
            _ => Err("invalid group id".to_string()),
        }
    }

    /// Convert Group ID from String to Binary
    fn id_string_to_bin(&self, id: String) -> Result<Vec<u8>, String> {
        // check length
        if id.len() < 52 {
            return Err("Group ID not long enough".to_string());
        }

        // convert input
        match bs58::decode(id).into_vec() {
            Ok(id_bin) => Ok(id_bin),
            Err(e) => {
                let err = fmt::format(format_args!("{}", e));
                Err(err)
            }
        }
    }

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
                let group_id = self.uuid_string_to_bin(group_id.to_string())?;
                let proto_message = Group {
                    message: Some(group::Message::GroupRenameRequest(GroupRenameRequest {
                        group_name: name.clone(),
                        group_id: group_id.clone(),
                    })),
                };

                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
            GroupSubcmd::Info { id } => {
                let group_id = self.uuid_string_to_bin(id.to_string())?;

                let proto_message = Group {
                    message: Some(group::Message::GroupInfoRequest(GroupInfoRequest {
                        group_id: group_id.clone(),
                    })),
                };

                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
            GroupSubcmd::List => {
                let proto_message = Group {
                    message: Some(group::Message::GroupListRequest(GroupListRequest {})),
                };

                // encode message
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Group))
            }
            GroupSubcmd::Invited => {
                let proto_message = Group {
                    message: Some(group::Message::GroupInvitedRequest(GroupInvitedRequest {})),
                };

                Ok((proto_message.encode_to_vec(), Modules::Group))
            }
            GroupSubcmd::Invite { group_id, user_id } => {
                let group_id = self.uuid_string_to_bin(group_id.to_string())?;
                let user_id = self.id_string_to_bin(user_id.to_string())?;

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
                let group_id = self.uuid_string_to_bin(group_id.to_string())?;
                self.reply_to_invite(group_id, true)
            }
            GroupSubcmd::Decline { group_id } => {
                let group_id = self.uuid_string_to_bin(group_id.to_string())?;
                self.reply_to_invite(group_id, false)
            }
            GroupSubcmd::Remove { group_id, user_id } => {
                let group_id = self.uuid_string_to_bin(group_id.to_string())?;
                let user_id = self.id_string_to_bin(user_id.to_string())?;

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
    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let group = Group::decode(data)?;
        match group.message {
            Some(group::Message::GroupCreateResponse(create_group_response)) => {
                println!("====================================");
                println!("Group was created or updated");
                let group_id = Uuid::from_bytes(create_group_response.group_id.try_into().unwrap());
                println!("\tid: {}", group_id.to_string());
            }
            Some(group::Message::GroupRenameResponse(rename_group_response)) => {
                let result = rename_group_response.result.unwrap();
                println!("====================================");
                println!("Group Rename status: {}", result.status);
                let group_id = Uuid::from_bytes(rename_group_response.group_id.try_into().unwrap());
                println!("\tid: {}", group_id.to_string());
                if !result.status {
                    println!("\terror: {}", result.message);
                }
            }
            Some(group::Message::GroupInviteMemberResponse(invite_group_response)) => {
                let result = invite_group_response.result.unwrap();
                println!("====================================");
                println!("Group Invite status: {}", result.status);
                let group_id = Uuid::from_bytes(invite_group_response.group_id.try_into().unwrap());
                println!("\tid: {}", group_id.to_string());
                if !result.status {
                    println!("\terror: {}", result.message);
                }
            }
            Some(group::Message::GroupReplyInviteResponse(reply_group_response)) => {
                let result = reply_group_response.result.unwrap();
                println!("====================================");
                println!("Reply Group Invite status: {}", result.status);
                let group_id = Uuid::from_bytes(reply_group_response.group_id.try_into().unwrap());
                println!("\tid: {}", group_id.to_string());
                if !result.status {
                    println!("\terror: {}", result.message);
                }
            }
            Some(group::Message::GroupRemoveMemberResponse(remove_member_response)) => {
                let result = remove_member_response.result.unwrap();
                println!("====================================");
                println!("Group Remove Member status: {}", result.status);
                let group_id =
                    Uuid::from_bytes(remove_member_response.group_id.try_into().unwrap());
                println!("\tid: {}", group_id.to_string());
                if !result.status {
                    println!("\terror: {}", result.message);
                }
            }
            Some(group::Message::GroupInfoResponse(group_info_response)) => {
                // group
                println!("====================================");
                println!("Group Information");
                let group_id = Uuid::from_bytes(group_info_response.group_id.try_into().unwrap());
                println!("\tid: {}", group_id.to_string());
                println!("\tname: {}", group_info_response.group_name.clone());
                println!("\tcreated_at: {}", group_info_response.created_at);
                println!("\tmembers: {}", group_info_response.members.len());
            }
            Some(group::Message::GroupListResponse(group_list_response)) => {
                // List groups
                println!("=============List Of Groups=================");
                for group in group_list_response.groups {
                    let group_id = uuid::Uuid::from_bytes(group.group_id.try_into().unwrap());
                    let group_type: String;
                    match group.is_direct_chat {
                        true => group_type = "Direct".to_string(),
                        false => group_type = "Group".to_string(),
                    }
                    println!(
                        "{} {} {}",
                        group_type,
                        group_id.to_string(),
                        group.group_name.clone()
                    );
                    print!("\tstatus: ");
                    match GroupStatus::try_from(group.status) {
                        Ok(GroupStatus::Active) => println!("Active"),
                        Ok(GroupStatus::InviteAccepted) => {
                            println!("Invite Accepted")
                        }
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
                            Ok(GroupMemberState::Invited) => {
                                print!("invited , role: ");
                            }
                            Ok(GroupMemberState::Activated) => {
                                print!("activated , role: ");
                            }
                            Err(_) => {}
                        }

                        match GroupMemberRole::try_from(member.role) {
                            Ok(GroupMemberRole::User) => {
                                println!("user , sent: {}", member.last_message_index);
                            }
                            Ok(GroupMemberRole::Admin) => {
                                println!("admin , sent: {}", member.last_message_index);
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
            Some(group::Message::GroupInvitedResponse(group_invited_response)) => {
                // List of pending invites
                println!("=============List Of Invited=================");
                for invite in group_invited_response.invited {
                    if let Some(group) = invite.group {
                        let group_id = uuid::Uuid::from_bytes(group.group_id.try_into().unwrap());
                        println!("id: {}", group_id.to_string());
                        println!("\tname: {}", group.group_name.clone());
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
            _ => {
                log::error!("unprocessable RPC group chat message");
            }
        };
        Ok(())
    }
}
