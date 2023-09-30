// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group module functions

use std::fmt;

use super::rpc::Rpc;
use crate::{
    chat,
    configuration::{MatrixConfiguration, MatrixRoom},
    relay_bot::{MATRIX_CLIENT, MATRIX_CONFIG},
    users::QAUL_USERS,
};
use libp2p::PeerId;
use matrix_sdk::{
    room::Room,
    ruma::{
        api::client::r0::room::create_room::Request as CreateRoomRequest,
        events::{room::message::MessageEventContent, AnyMessageEventContent},
        identifiers::RoomNameBox,
        RoomId, UserId,
    },
};
use prost::Message;
use tokio::runtime::Runtime;

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
    pub fn uuid_string_to_bin(id_str: String) -> Result<Vec<u8>, String> {
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

    /// group info
    pub fn group_info(group_id: Vec<u8>, request_id: String) {
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
        Rpc::send_message(buf, super::rpc::proto::Modules::Group.into(), request_id);
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
    pub fn group_invited() {
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

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the group chat module.
    pub fn rpc(data: Vec<u8>, request_id: String) {
        match proto::Group::decode(&data[..]) {
            Ok(group_chat) => {
                match group_chat.message {
                    Some(proto::group::Message::GroupCreateResponse(create_group_response)) => {
                        log::info!("====================================");
                        log::info!("Group was created or updated");
                        let group_id = uuid::Uuid::from_bytes(
                            create_group_response.group_id.try_into().unwrap(),
                        );
                        log::info!("\tid: {}", group_id.to_string());
                        if request_id.contains('#') {
                            let mut iter = request_id.split('#');
                            let _cmd = iter.next().unwrap();
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
                        log::info!("====================================");
                        log::info!("Group Rename status: {}", result.status);
                        let group_id = uuid::Uuid::from_bytes(
                            rename_group_response.group_id.try_into().unwrap(),
                        );
                        log::info!("\tid: {}", group_id.to_string());
                        if !result.status {
                            log::info!("\terror: {}", result.message);
                        }
                    }
                    Some(proto::group::Message::GroupInviteMemberResponse(
                        invite_group_response,
                    )) => {
                        let result = invite_group_response.result.unwrap();
                        log::info!("====================================");
                        log::info!("Group Invite status: {}", result.status);
                        let group_id = uuid::Uuid::from_bytes(
                            invite_group_response.group_id.try_into().unwrap(),
                        );
                        log::info!("\tid: {}", group_id.to_string());
                        if !result.status {
                            log::info!("\terror: {}", result.message);
                        }
                    }
                    Some(proto::group::Message::GroupReplyInviteResponse(reply_group_response)) => {
                        let result = reply_group_response.result.unwrap();
                        log::info!("====================================");
                        log::info!("Reply Group Invite status: {}", result.status);
                        let group_id = uuid::Uuid::from_bytes(
                            reply_group_response.group_id.try_into().unwrap(),
                        );
                        log::info!("\tid: {}", group_id.to_string());
                        if !result.status {
                            log::info!("\terror: {}", result.message);
                        }
                    }
                    Some(proto::group::Message::GroupRemoveMemberResponse(
                        remove_member_response,
                    )) => {
                        let result = remove_member_response.result.unwrap();
                        log::info!("====================================");
                        log::info!("Group Remove Member status: {}", result.status);
                        let group_id = uuid::Uuid::from_bytes(
                            remove_member_response.group_id.try_into().unwrap(),
                        );
                        log::info!("\tid: {}", group_id.to_string());
                        if !result.status {
                            log::info!("\terror: {}", result.message);
                        }
                    }

                    Some(proto::group::Message::GroupInfoResponse(group_info_response)) => {
                        let group_id = uuid::Uuid::from_bytes(
                            group_info_response.group_id.try_into().unwrap(),
                        );

                        if request_id != "" {
                            // reqeust_id = qaul_user_id#room_id
                            let mut iter = request_id.split('#');
                            let cmd = iter.next().unwrap();
                            log::info!("cmd : {}", cmd);
                            let room_id = iter.next().unwrap();
                            log::info!("room : {}", room_id);
                            let _sender = iter.next().unwrap();
                            log::info!("sender : {}", _sender);
                            let qaul_user_id = iter.next().unwrap();
                            log::info!("qaul user : {}", qaul_user_id);

                            if cmd == "invite" {
                                let grp_members = group_info_response.members.clone();
                                let user_id =
                                    chat::Chat::id_string_to_bin(qaul_user_id.to_owned()).unwrap();
                                let mut all_members = Vec::new();
                                for member in grp_members {
                                    all_members.push(member.user_id);
                                }
                                if all_members.contains(&user_id) {
                                    matrix_rpc(
                                        "User already exist in the qaul group".to_owned(),
                                        RoomId::try_from(room_id).unwrap(),
                                    );
                                } else {
                                    // Invite user into this group.
                                    let users = QAUL_USERS.get().read().unwrap();
                                    let user_name = chat::Chat::find_user_for_given_id(
                                        users.clone(),
                                        qaul_user_id.to_owned(),
                                    )
                                    .unwrap();
                                    matrix_rpc(
                                        format!("User {} has been invited. Please wait until user accepts the invitation.", 
                                        user_name
                                    ).to_owned(), RoomId::try_from(room_id).unwrap());
                                    matrix_rpc("User has been invited. Please wait until user accepts the invitation.".to_owned(), RoomId::try_from(room_id).unwrap());
                                    Self::invite(
                                        chat::Chat::uuid_string_to_bin(group_id.to_string())
                                            .unwrap(),
                                        user_id,
                                    );
                                }
                            }

                            if cmd == "remove" {
                                let grp_members = group_info_response.members.clone();
                                let user_id =
                                    chat::Chat::id_string_to_bin(qaul_user_id.to_owned()).unwrap();
                                let mut all_members = Vec::new();
                                for member in grp_members {
                                    all_members.push(member.user_id);
                                }
                                if all_members.contains(&user_id) {
                                    // Remove
                                    Self::remove_member(
                                        chat::Chat::uuid_string_to_bin(group_id.to_string())
                                            .unwrap(),
                                        user_id,
                                    );
                                    matrix_rpc(
                                        "User has been removed".to_owned(),
                                        RoomId::try_from(room_id).unwrap(),
                                    );
                                } else {
                                    // Member is not in grp.
                                    matrix_rpc(
                                        "User is not a member of this grp.".to_owned(),
                                        RoomId::try_from(room_id).unwrap(),
                                    );
                                }
                            }

                            if cmd == "info" {
                                let group_id = group_id.to_string();
                                let creation_time = group_info_response.created_at;
                                let members = group_info_response.members;
                                let mut member_string = String::new();
                                let users = QAUL_USERS.get().read().unwrap();
                                let mut i = 1;
                                for member in members {
                                    let user_name = chat::Chat::find_user_for_given_id(
                                        users.clone(),
                                        bs58::encode(member.clone().user_id).into_string(),
                                    )
                                    .unwrap();
                                    let mut is_admin = String::new();
                                    if member.role == 255 {
                                        is_admin.push_str("Admin");
                                    } else {
                                        is_admin.push_str("Member");
                                    }
                                    member_string.push_str(&format!(
                                        "{} : {}({}) | Peer ID : {}\n",
                                        i,
                                        user_name,
                                        is_admin,
                                        bs58::encode(member.user_id).into_string()
                                    ));
                                    i += 1;
                                }
                                let message_format = format!("# Group Information \n\nGroup ID : {}\nCreated at : {}\nList of Members : \n{}",group_id,creation_time,member_string);
                                matrix_rpc(message_format, RoomId::try_from(room_id).unwrap());
                            }
                        }
                    }
                    Some(proto::group::Message::GroupListResponse(group_list_response)) => {
                        let all_groups = group_list_response.groups.clone();

                        let mut config = MATRIX_CONFIG.get().write().unwrap();
                        for group in all_groups {
                            // If Mapping exist let it be. Else create new room.
                            let group_id =
                                uuid::Uuid::from_bytes(group.group_id.try_into().unwrap());
                            // qaul_groups.insert(group_id, group.group_name.clone());
                            let mut qaul_room_admin = format!("@qaul://{}", "[username]");
                            for member in group.members {
                                if member.role == 255 {
                                    let user_id = PeerId::from_bytes(&member.user_id).unwrap();
                                    qaul_room_admin.push_str(&user_id.to_string());
                                }
                            }

                            // If group mapping does not exist
                            // If group_name contains matrix user name then only do this.
                            if let Ok(user) = UserId::try_from(group.group_name.clone()) {
                                if !config.room_map.contains_key(&group_id) {
                                    let matrix_client = MATRIX_CLIENT.get();
                                    let rt = Runtime::new().unwrap();
                                    rt.block_on(async {
                                        log::info!("{:#?}", group_id);
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
                                            log::info!("Wait till the bot joins the room");
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
                    }
                    Some(proto::group::Message::GroupInvitedResponse(group_invited_response)) => {
                        // List of pending invites
                        for invite in group_invited_response.invited {
                            if let Some(group) = invite.group {
                                let group_id = uuid::Uuid::from_bytes(
                                    group.group_id.clone().try_into().unwrap(),
                                );
                                log::info!("id: {}", group_id.to_string());
                                log::info!("\tname: {}", group.group_name.clone());
                                log::info!(
                                    "\tsender: {}",
                                    bs58::encode(invite.sender_id).into_string()
                                );
                                log::info!("\treceived at: {}", invite.received_at);
                                log::info!(
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

/// Connect RPC function call with the Matrix Room and send message
fn matrix_rpc(msg: String, room_id: RoomId) {
    // Get the Room based on RoomID from the client information
    let matrix_client = MATRIX_CLIENT.get();
    let room = matrix_client.get_room(&room_id).unwrap();
    if let Room::Joined(room) = room {
        // Build the message content to send to matrix
        let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(msg));

        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // Sends messages into the matrix room
            room.send(content, None).await.unwrap();
        });
    };
}
