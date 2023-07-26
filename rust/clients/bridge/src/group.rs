// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group module functions

use core::panic;
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    sync::{RwLock, RwLockWriteGuard},
};

use super::rpc::Rpc;
use crate::{
    configuration::{MatrixConfiguration, MatrixRoom},
    relay_bot::{MATRIX_CLIENT, MATRIX_CONFIG},
    users,
};
use libp2p::PeerId;
use matrix_sdk::ruma::{
    api::client::r0::room::create_room::Request as CreateRoomRequest,
    identifiers::{RoomName, RoomNameBox},
    serde::urlencoded::from_bytes,
    RoomId, UserId,
};
use prost::Message;
use tokio::runtime::Runtime;
use uuid::Uuid;

// pub static QAUL_GROUPS: state::Storage<RwLock<BTreeMap<Uuid, String>>> = state::Storage::new();

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.group.rs");
}

/// include chat protobuf RPC file
mod proto_chat {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.chat.rs");
}

/// Group module function handling
pub struct Group {}

impl Group {
    /// CLI command interpretation
    ///
    /// The CLI commands of group module are processed here
    pub fn cli(command: &str) {
        match command {
            // create group
            cmd if cmd.starts_with("create ") => {
                let command_string = cmd.strip_prefix("create ").unwrap().to_string();
                let group_name = command_string.trim().to_string();

                if group_name.len() > 0 {
                    // We pass empty room_id so that normal qaul groups do not get disturbed.
                    Self::create_group(group_name.clone(), "".to_string());
                } else {
                    log::error!("group create command incorrectly formatted");
                }
            }
            // rename group
            cmd if cmd.starts_with("rename ") => {
                let command_string = cmd.strip_prefix("rename ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::uuid_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            let group_name = command_string
                                .strip_prefix(group_id_str.clone())
                                .unwrap()
                                .to_string()
                                .trim()
                                .to_string();

                            if group_name.len() > 0 {
                                Self::rename_group(group_id, group_name.to_string());
                            } else {
                                log::error!("group name is missing");
                            }
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("group create command incorrectly formatted");
                }
            }
            // group info
            cmd if cmd.starts_with("info ") => {
                let command_string = cmd.strip_prefix("info ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::uuid_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            Self::group_info(group_id);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("group create command incorrectly formatted");
                }
            }
            // group list
            cmd if cmd.starts_with("list") => {
                //let command_string = cmd.strip_prefix("list ").unwrap().to_string();
                Self::group_list();
            }
            // group list
            cmd if cmd.starts_with("invited") => {
                Self::group_invited();
            }
            // group invite
            cmd if cmd.starts_with("invite ") => {
                let command_string = cmd.strip_prefix("invite ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::uuid_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            if let Some(user_id_str) = iter.next() {
                                match Self::id_string_to_bin(user_id_str.to_string()) {
                                    Ok(user_id) => {
                                        Self::invite(group_id, user_id);
                                    }
                                    Err(e) => {
                                        log::error!("{}", e);
                                        return;
                                    }
                                }
                            } else {
                                log::error!("user id is not given");
                            }
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("group create command incorrectly formatted");
                }
            }
            // accept invite
            cmd if cmd.starts_with("accept ") => {
                let command_string = cmd.strip_prefix("accept ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::uuid_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            Self::reply_invite(group_id, true);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("group accept command incorrectly formatted");
                }
            }
            // decline invite
            cmd if cmd.starts_with("decline ") => {
                let command_string = cmd.strip_prefix("decline ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::uuid_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            Self::reply_invite(group_id, false);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("group accept command incorrectly formatted");
                }
            }
            // remove member
            cmd if cmd.starts_with("remove ") => {
                let command_string = cmd.strip_prefix("remove ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::uuid_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            if let Some(user_id_str) = iter.next() {
                                match Self::id_string_to_bin(user_id_str.to_string()) {
                                    Ok(user_id) => {
                                        Self::remove_member(group_id, user_id);
                                    }
                                    Err(e) => {
                                        log::error!("{}", e);
                                        return;
                                    }
                                }
                            } else {
                                log::error!("user id is not given");
                            }
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("group remove command incorrectly formatted");
                }
            }
            // unknown command
            _ => log::error!("unknown group command"),
        }
    }

    /// Convert Group ID from String to Binary
    fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
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

    /// Convert Group ID from String to Binary
    fn uuid_string_to_bin(id_str: String) -> Result<Vec<u8>, String> {
        match uuid::Uuid::parse_str(id_str.as_str()) {
            Ok(id) => Ok(id.as_bytes().to_vec()),
            _ => Err("invalid group id".to_string()),
        }
    }

    /// create group
    pub fn create_group(group_name: String, request_id: String) {
        // create group send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupCreateRequest(
                proto::GroupCreateRequest {
                    group_name: group_name.clone(),
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Group.into(), request_id);
    }

    /// rename group
    fn rename_group(group_id: Vec<u8>, group_name: String) {
        // rename group send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupRenameRequest(
                proto::GroupRenameRequest {
                    group_name: group_name.clone(),
                    group_id: group_id.clone(),
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Group.into(),
            "".to_string(),
        );
    }

    /// group info
    fn group_info(group_id: Vec<u8>) {
        // group info send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupInfoRequest(
                proto::GroupInfoRequest {
                    group_id: group_id.clone(),
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Group.into(),
            "".to_string(),
        );
    }

    /// group list
    pub fn group_list() {
        // group list send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupListRequest(
                proto::GroupListRequest {},
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Group.into(),
            "".to_string(),
        );
    }

    /// group invited
    fn group_invited() {
        // group list send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupInvitedRequest(
                proto::GroupInvitedRequest {},
            )),
        };

        // send message
        Rpc::send_message(
            proto_message.encode_to_vec(),
            super::rpc::proto::Modules::Group.into(),
            "".to_string(),
        );
    }

    /// group invite
    fn invite(group_id: Vec<u8>, user_id: Vec<u8>) {
        // group invite send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupInviteMemberRequest(
                proto::GroupInviteMemberRequest {
                    group_id: group_id.clone(),
                    user_id: user_id.clone(),
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Group.into(),
            "".to_string(),
        );
    }

    /// reply invite
    fn reply_invite(group_id: Vec<u8>, accept: bool) {
        // group invite send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupReplyInviteRequest(
                proto::GroupReplyInviteRequest {
                    group_id: group_id.clone(),
                    accept,
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Group.into(),
            "".to_string(),
        );
    }

    /// remove member
    fn remove_member(group_id: Vec<u8>, user_id: Vec<u8>) {
        // group invite send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupRemoveMemberRequest(
                proto::GroupRemoveMemberRequest {
                    group_id: group_id.clone(),
                    user_id: user_id.clone(),
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Group.into(),
            "".to_string(),
        );
    }

    /// Process the last message & print it's content
    fn print_last_message(data: Vec<u8>) {
        if let Ok(content_message) = proto_chat::ChatContentMessage::decode(&data[..]) {
            match content_message.message {
                Some(proto_chat::chat_content_message::Message::ChatContent(chat_content)) => {
                    println!("\t\t{}", chat_content.text);
                }
                Some(proto_chat::chat_content_message::Message::FileContent(file_content)) => {
                    println!(
                        "\t\tfile {} [{}] ID {}",
                        file_content.file_name,
                        file_content.file_size.to_string(),
                        file_content.file_id.to_string()
                    );
                    println!("\t\t{}", file_content.file_description);
                }
                Some(proto_chat::chat_content_message::Message::GroupEvent(group_event)) => {
                    match proto_chat::GroupEventType::from_i32(group_event.event_type).unwrap() {
                        proto_chat::GroupEventType::Joined => {
                            println!(
                                "\t\tNew user joined group, user id: {}",
                                bs58::encode(group_event.user_id).into_string()
                            );
                        }
                        proto_chat::GroupEventType::Left => {
                            println!(
                                "\t\tUser left group, user id: {}",
                                bs58::encode(group_event.user_id).into_string()
                            );
                        }
                        proto_chat::GroupEventType::Removed => {
                            println!("\t\tYou have been removed from this group.");
                        }
                        proto_chat::GroupEventType::Created => {
                            println!("\t\tYou created this group");
                        }
                        proto_chat::GroupEventType::InviteAccepted => {
                            println!("\t\tYou accepted the invitation");
                        }
                        _ => {}
                    }
                }
                None => {}
            }
        }
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the group chat module.
    pub fn rpc(data: Vec<u8>, request_id: String) {
        match proto::Group::decode(&data[..]) {
            Ok(group_chat) => {
                match group_chat.message {
                    Some(proto::group::Message::GroupCreateResponse(create_group_response)) => {
                        println!("====================================");
                        println!("Group was created or updated");
                        let group_id = uuid::Uuid::from_bytes(
                            create_group_response.group_id.try_into().unwrap(),
                        );
                        println!("\tid: {}", group_id.to_string());
                        if request_id.contains('#') {
                            let mut iter = request_id.split('#');
                            let room_id = iter.next().unwrap();
                            let sender_id = iter.next().unwrap();
                            let qaul_user_id = iter.next().unwrap();
                            if let Ok(room_id) = RoomId::try_from(room_id) {
                                let mut config = MATRIX_CONFIG.get().write().unwrap();
                                let room_info = MatrixRoom {
                                    matrix_room_id: room_id,
                                    qaul_group_name: sender_id.to_owned(),
                                    last_index: 0,
                                };
                                config.room_map.insert(group_id, room_info);
                                Self::invite(
                                    Self::uuid_string_to_bin(group_id.to_string()).unwrap(),
                                    Self::id_string_to_bin(qaul_user_id.to_string()).unwrap(),
                                );
                                MatrixConfiguration::save(config.clone());
                            }
                        }
                    }
                    Some(proto::group::Message::GroupRenameResponse(rename_group_response)) => {
                        let result = rename_group_response.result.unwrap();
                        println!("====================================");
                        println!("Group Rename status: {}", result.status);
                        let group_id = uuid::Uuid::from_bytes(
                            rename_group_response.group_id.try_into().unwrap(),
                        );
                        println!("\tid: {}", group_id.to_string());
                        if !result.status {
                            println!("\terror: {}", result.message);
                        }
                    }
                    Some(proto::group::Message::GroupInviteMemberResponse(
                        invite_group_response,
                    )) => {
                        let result = invite_group_response.result.unwrap();
                        println!("====================================");
                        println!("Group Invite status: {}", result.status);
                        let group_id = uuid::Uuid::from_bytes(
                            invite_group_response.group_id.try_into().unwrap(),
                        );
                        println!("\tid: {}", group_id.to_string());
                        if !result.status {
                            println!("\terror: {}", result.message);
                        }
                    }
                    Some(proto::group::Message::GroupReplyInviteResponse(reply_group_response)) => {
                        let result = reply_group_response.result.unwrap();
                        println!("====================================");
                        println!("Reply Group Invite status: {}", result.status);
                        let group_id = uuid::Uuid::from_bytes(
                            reply_group_response.group_id.try_into().unwrap(),
                        );
                        println!("\tid: {}", group_id.to_string());
                        if !result.status {
                            println!("\terror: {}", result.message);
                        }
                    }
                    Some(proto::group::Message::GroupRemoveMemberResponse(
                        remove_member_response,
                    )) => {
                        let result = remove_member_response.result.unwrap();
                        println!("====================================");
                        println!("Group Remove Member status: {}", result.status);
                        let group_id = uuid::Uuid::from_bytes(
                            remove_member_response.group_id.try_into().unwrap(),
                        );
                        println!("\tid: {}", group_id.to_string());
                        if !result.status {
                            println!("\terror: {}", result.message);
                        }
                    }
                    Some(proto::group::Message::GroupInfoResponse(group_info_response)) => {
                        // group
                        println!("====================================");
                        println!("Group Information");
                        let group_id = uuid::Uuid::from_bytes(
                            group_info_response.group_id.try_into().unwrap(),
                        );
                        println!("\tid: {}", group_id.to_string());
                        println!("\tname: {}", group_info_response.group_name.clone());
                        println!("\tcreated_at: {}", group_info_response.created_at);
                        println!("\tmembers: {}", group_info_response.members.len());
                    }
                    Some(proto::group::Message::GroupListResponse(group_list_response)) => {
                        // List groups
                        // println!("=============List Of Groups=================");
                        let all_groups = group_list_response.groups.clone();

                        let mut config = MATRIX_CONFIG.get().write().unwrap();
                        // Create a HashMap for mapping. OR Deserialize from existing config.
                        // let mut qaul_groups = QAUL_GROUPS.get().write().unwrap();
                        for group in all_groups {
                            // If Mapping exist let it be. Else create new room.
                            let group_id =
                                uuid::Uuid::from_bytes(group.group_id.try_into().unwrap());
                            // qaul_groups.insert(group_id, group.group_name.clone());
                            // Currently we have user_id and not user_name. Shall we do this ? Or If yes then how to get
                            let mut qaul_room_admin = format!("@qaul://{}", "[username]");
                            for member in group.members {
                                if member.role == 255 {
                                    let user_id = PeerId::from_bytes(&member.user_id).unwrap();
                                    qaul_room_admin.push_str(&user_id.to_string());
                                    users::Users::peer_id_to_user_name();
                                }
                            }

                            // If group mapping does not exist
                            // If group_name contains matrix user name then only do this.
                            if let Ok(user) = UserId::try_from(group.group_name.clone()) {
                                if !config.room_map.contains_key(&group_id) {
                                    let matrix_client = MATRIX_CLIENT.get();
                                    let rt = Runtime::new().unwrap();
                                    rt.block_on(async {
                                        println!("{:#?}", group_id);
                                        // Check if user exist on matrix
                                        // Create a group on matrix with qaul user name.
                                        let mut request = CreateRoomRequest::new();
                                        let room_name =
                                            RoomNameBox::try_from(qaul_room_admin).unwrap();
                                        request.name = Some(&room_name);
                                        let room_id = matrix_client
                                            .create_room(request)
                                            .await
                                            .expect("Room creation failed")
                                            .room_id;

                                        // Check if the room is joined
                                        if let Some(joined_room) =
                                            matrix_client.get_joined_room(&room_id)
                                        {
                                            joined_room.invite_user_by_id(&user).await.unwrap();
                                        } else {
                                            println!("Wait till the bot joins the room");
                                        }

                                        // Save things to Config file
                                        let room_info = MatrixRoom {
                                            matrix_room_id: room_id,
                                            qaul_group_name: group.group_name,
                                            last_index: 0,
                                        };
                                        config.room_map.insert(group_id, room_info);
                                        MatrixConfiguration::save(config.clone());
                                    });
                                }
                            }
                        }
                        // for group in group_list_response.groups {
                        //     let group_id =
                        //         uuid::Uuid::from_bytes(group.group_id.try_into().unwrap());
                        //     // Mapping of groups with qaul Room;
                        //     let group_type: String;
                        //     match group.is_direct_chat {
                        //         true => group_type = "Direct".to_string(),
                        //         false => group_type = "Group".to_string(),
                        //     }
                        //     println!(
                        //         "{} {} {}",
                        //         group_type,
                        //         group_id.to_string(),
                        //         group.group_name.clone()
                        //     );
                        //     print!("\tstatus: ");
                        //     match proto::GroupStatus::from_i32(group.status) {
                        //         Some(proto::GroupStatus::Active) => println!("Active"),
                        //         Some(proto::GroupStatus::InviteAccepted) => {
                        //             println!("Invite Accepted")
                        //         }
                        //         Some(proto::GroupStatus::Deactivated) => println!("Deactivated"),
                        //         None => println!("NOT SET"),
                        //     }

                        //     println!(
                        //         "\tcreated_at: {}, members: {}",
                        //         group.created_at,
                        //         group.members.len()
                        //     );
                        //     for member in group.members {
                        //         print!(
                        //             "\t\t id: {} , state: ",
                        //             bs58::encode(member.user_id.clone()).into_string()
                        //         );
                        //         match proto::GroupMemberState::from_i32(member.state).unwrap() {
                        //             proto::GroupMemberState::Invited => {
                        //                 print!("invited , role: ");
                        //             }
                        //             proto::GroupMemberState::Activated => {
                        //                 print!("activated , role: ");
                        //             }
                        //         }

                        //         match proto::GroupMemberRole::from_i32(member.role).unwrap() {
                        //             proto::GroupMemberRole::User => {
                        //                 println!("user , sent: {}", member.last_message_index);
                        //             }
                        //             proto::GroupMemberRole::Admin => {
                        //                 println!("admin , sent: {}", member.last_message_index);
                        //             }
                        //         }
                        //     }
                        //     println!("\trevision: {}", group.revision);
                        //     println!("\tunread messages: {}", group.unread_messages);
                        //     println!("\tlast message:");
                        //     println!(
                        //         "\t\tsent_at: {} from: {}",
                        //         group.last_message_at,
                        //         bs58::encode(group.last_message_sender_id).into_string()
                        //     );
                        //     Self::print_last_message(group.last_message);
                        // }
                    }
                    Some(proto::group::Message::GroupInvitedResponse(group_invited_response)) => {
                        // List of pending invites
                        println!("=============List Of Invited=================");
                        for invite in group_invited_response.invited {
                            if let Some(group) = invite.group {
                                let group_id = uuid::Uuid::from_bytes(
                                    group.group_id.clone().try_into().unwrap(),
                                );
                                println!("id: {}", group_id.to_string());
                                println!("\tname: {}", group.group_name.clone());
                                println!(
                                    "\tsender: {}",
                                    bs58::encode(invite.sender_id).into_string()
                                );
                                println!("\treceived at: {}", invite.received_at);
                                println!(
                                    "\tcreated_at: {}, members: {}",
                                    invite.received_at,
                                    group.members.len()
                                );

                                // Accept the group invite automatically
                                Self::reply_invite(group.group_id, true);
                            }
                        }
                    }
                    _ => {
                        log::error!("unprocessable RPC group chat message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
