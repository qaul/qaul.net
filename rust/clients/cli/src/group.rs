// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group module functions

use super::rpc::Rpc;
use prost::Message;
use std::fmt;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.group.rs");
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
                let mut iter = command_string.split_whitespace();

                if let Some(group_name) = iter.next() {
                    Self::create_group(group_name.to_string());
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
                            if let Some(group_name) = iter.next() {
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
                            if let Some(user_id_str) = iter.next() {
                                match Self::id_string_to_bin(user_id_str.to_string()) {
                                    Ok(user_id) => {
                                        Self::reply_invite(group_id, user_id, true);
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
                            if let Some(user_id_str) = iter.next() {
                                match Self::id_string_to_bin(user_id_str.to_string()) {
                                    Ok(user_id) => {
                                        Self::reply_invite(group_id, user_id, false);
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

    /// Convert Conversation ID from String to Binary
    fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
        // check length
        if id.len() < 52 {
            return Err("Conversation ID not long enough".to_string());
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

    /// Convert Conversation ID from String to Binary
    fn uuid_string_to_bin(id_str: String) -> Result<Vec<u8>, String> {
        match uuid::Uuid::parse_str(id_str.as_str()) {
            Ok(id) => Ok(id.as_bytes().to_vec()),
            _ => Err("invalid group id".to_string()),
        }
    }

    /// create group
    fn create_group(group_name: String) {
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
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Group.into(),
            "".to_string(),
        );
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
    fn group_list() {
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
    fn reply_invite(group_id: Vec<u8>, user_id: Vec<u8>, accept: bool) {
        // group invite send message
        let proto_message = proto::Group {
            message: Some(proto::group::Message::GroupReplyInviteRequest(
                proto::GroupReplyInviteRequest {
                    group_id: group_id.clone(),
                    user_id: user_id.clone(),
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

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the group chat module.
    pub fn rpc(data: Vec<u8>) {
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
                    }
                    Some(proto::group::Message::GroupRenameResponse(rename_group_response)) => {
                        let result = rename_group_response.result.unwrap();
                        println!("====================================");
                        println!("Group Reanme status: {}", result.status);
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
                        println!("=============List Of Groups=================");
                        for group in group_list_response.groups {
                            let group_id =
                                uuid::Uuid::from_bytes(group.group_id.try_into().unwrap());
                            println!("id: {}", group_id.to_string());
                            println!("\tname: {}", group.group_name.clone());
                            println!("\tis_direct_chat: {}", group.is_direct_chat);
                            println!(
                                "\tcreated_at: {}, members: {}",
                                group.created_at,
                                group.members.len()
                            );
                            for member in group.members {
                                println!(
                                    "\t\t id: {}, sent: {}",
                                    bs58::encode(member.user_id.clone()).into_string(),
                                    member.last_message_index
                                );
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
