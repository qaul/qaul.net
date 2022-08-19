//use bs58::decode;
use libp2p::PeerId;
use prost::Message;

use crate::services::messaging;
use crate::utilities::timestamp;
use std::collections::BTreeMap;

use super::chat;
use super::Group;

pub struct Manage {}
impl Manage {
    pub fn create_new_direct_chat_group(user_id: &PeerId, peer_id: &PeerId) -> Vec<u8> {
        let conversation_id =
            super::messaging::ConversationId::from_peers(user_id, peer_id).unwrap();
        let group_id = &conversation_id.to_bytes();

        let groups = Group::get_groups_of_user(user_id);
        let group_idx = groups.group_id_to_index(group_id);
        if group_idx > 0 {
            return group_id.clone();
        }

        let mut members = BTreeMap::new();
        members.insert(
            user_id.to_bytes(),
            super::GroupMember {
                user_id: user_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: timestamp::Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            },
        );
        members.insert(
            peer_id.to_bytes(),
            super::GroupMember {
                user_id: peer_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: timestamp::Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            },
        );

        let new_group = super::Group {
            id: group_id.clone(),
            name: "".to_string(),
            is_direct_chat: true,
            created_at: timestamp::Timestamp::get_timestamp(),
            creator_id: user_id.to_bytes(),
            members,
        };
        let mut groups = Group::get_groups_of_user(user_id);

        let key = groups.last_group + 1;
        //db save
        if let Err(error) = groups.db_ref.insert(&key.to_be_bytes(), new_group) {
            log::error!("group db updating error {}", error.to_string());
        }
        if let Err(error) = groups.db_ref.flush() {
            log::error!("group db flush error {}", error.to_string());
        }

        //add new id
        groups.last_group = key;
        groups.group_ids.insert(group_id.clone(), key);
        Group::update_groups_of_user(user_id, groups);

        return group_id.clone();
    }

    /// create new group from rpc command
    pub fn create_new_group(user_id: &PeerId, name: String) -> Vec<u8> {
        let id = uuid::Uuid::new_v4().as_bytes().to_vec();

        let mut members = BTreeMap::new();
        members.insert(
            user_id.to_bytes(),
            super::GroupMember {
                user_id: user_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: timestamp::Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            },
        );

        let new_group = super::Group {
            id: id.clone(),
            name,
            is_direct_chat: false,
            created_at: timestamp::Timestamp::get_timestamp(),
            creator_id: user_id.to_bytes(),
            members,
        };

        let mut groups = Group::get_groups_of_user(user_id);

        let key = groups.last_group + 1;
        //db save
        if let Err(error) = groups.db_ref.insert(&key.to_be_bytes(), new_group) {
            log::error!("group db updating error {}", error.to_string());
        }
        if let Err(error) = groups.db_ref.flush() {
            log::error!("group db flush error {}", error.to_string());
        }

        //add new id
        groups.last_group = key;
        groups.group_ids.insert(id.clone(), key);
        Group::update_groups_of_user(user_id, groups);

        return id.clone();
    }

    /// remove group from rpc command
    pub fn rename_group(
        user_id: &PeerId,
        group_id: &Vec<u8>,
        name: String,
    ) -> Result<Vec<u8>, String> {
        let groups = Group::get_groups_of_user(user_id);
        let group_idx = groups.group_id_to_index(group_id);
        if group_idx == 0 {
            return Err("can not find group".to_string());
        }

        let mut group = groups
            .db_ref
            .get(&group_idx.to_be_bytes())
            .unwrap()
            .unwrap();
        if let Some(member) = group.get_member(&user_id.to_bytes()) {
            //check permission
            if member.role != 255 {
                return Err("you have not permission for rename this group".to_string());
            }
        } else {
            return Err("you are not member for this group".to_string());
        }

        //rename and save db
        group.name = name.clone();
        if let Err(error) = groups.db_ref.insert(&group_idx.to_be_bytes(), group) {
            log::error!("group db updating error {}", error.to_string());
        }
        Ok(group_id.clone())
    }

    /// get group information from rpc command
    pub fn group_info(
        user_id: &PeerId,
        group_id: &Vec<u8>,
    ) -> Result<super::proto_rpc::GroupInfoResponse, String> {
        let groups = Group::get_groups_of_user(user_id);

        let group_idx = groups.group_id_to_index(group_id);
        if group_idx == 0 {
            return Err("can not find group".to_string());
        }

        let group = groups
            .db_ref
            .get(&group_idx.to_be_bytes())
            .unwrap()
            .unwrap();
        let mut members: Vec<super::proto_rpc::GroupMember> = vec![];
        for m in group.members.values() {
            let member = super::proto_rpc::GroupMember {
                user_id: m.user_id.clone(),
                role: m.role,
                joined_at: m.joined_at,
                state: m.state,
                last_message_index: m.last_message_index,
            };
            members.push(member);
        }

        let res = super::proto_rpc::GroupInfoResponse {
            group_id: group.id,
            group_name: group.name,
            created_at: group.created_at,
            is_direct_chat: group.is_direct_chat,
            members,
        };
        Ok(res)
    }

    /// get group list from rpc command
    pub fn group_list(user_id: &PeerId) -> super::proto_rpc::GroupListResponse {
        let groups = Group::get_groups_of_user(user_id);

        let mut res = super::proto_rpc::GroupListResponse { groups: vec![] };

        for entry in groups.db_ref.iter() {
            match entry {
                Ok((_, group)) => {
                    let mut members: Vec<super::proto_rpc::GroupMember> = vec![];
                    for m in group.members.values() {
                        let member = super::proto_rpc::GroupMember {
                            user_id: m.user_id.clone(),
                            role: m.role,
                            joined_at: m.joined_at,
                            state: m.state,
                            last_message_index: m.last_message_index,
                        };
                        members.push(member);
                    }

                    let grp = super::proto_rpc::GroupInfoResponse {
                        group_id: group.id,
                        group_name: group.name,
                        created_at: group.created_at,
                        is_direct_chat: group.is_direct_chat,
                        members,
                    };
                    res.groups.push(grp);
                }
                _ => {}
            }
        }
        res
    }

    /// get group list from rpc command
    pub fn invited_list(user_id: &PeerId) -> super::proto_rpc::GroupInvitedResponse {
        let groups = Group::get_groups_of_user(user_id);

        let mut res = super::proto_rpc::GroupInvitedResponse { invited: vec![] };

        for entry in groups.invited_ref.iter() {
            match entry {
                Ok((_, invite)) => {
                    let invited = super::proto_rpc::GroupInvited {
                        group_id: invite.id.clone(),
                        sender_id: invite.sender_id.clone(),
                        received_at: invite.received_at,
                        group_name: invite.name.clone(),
                        created_at: invite.created_at,
                        member_count: invite.member_count,
                    };
                    res.invited.push(invited);
                }
                _ => {}
            }
        }
        res
    }

    /// process group notify message from network
    pub fn on_group_notify(
        sender_id: &Vec<u8>,
        receiver_id: &Vec<u8>,
        notify: &super::proto_net::GroupNotify,
    ) {
        log::error!("on_group_notify!");
        let user_id = PeerId::from_bytes(receiver_id).unwrap();
        let mut groups = Group::get_groups_of_user(&user_id);

        let mut group_idx = groups.group_id_to_index(&notify.group_id);
        let mut first_join = false;
        let mut orign_members: BTreeMap<Vec<u8>, bool> = BTreeMap::new();
        let mut new_members: Vec<Vec<u8>> = vec![];

        if group_idx == 0 {
            group_idx = groups.last_group + 1;
            groups.last_group = group_idx;
            groups.group_ids.insert(notify.group_id.clone(), group_idx);
            first_join = true;
        } else {
            // get all origin members
            if let Ok(grp_opt) = groups.db_ref.get(&group_idx.to_be_bytes().to_vec()) {
                for (member_id, _member) in &grp_opt.unwrap().members {
                    orign_members.insert(member_id.clone(), true);
                }
            }
        }

        let mut members: BTreeMap<Vec<u8>, super::GroupMember> = BTreeMap::new();
        for m in &notify.members {
            if orign_members.contains_key(&m.user_id) {
                orign_members.remove(&m.user_id);
            } else {
                new_members.push(m.user_id.clone());
            }

            members.insert(
                m.user_id.clone(),
                super::GroupMember {
                    user_id: m.user_id.clone(),
                    role: m.role,
                    joined_at: m.joined_at,
                    state: m.state,
                    last_message_index: m.last_message_index,
                },
            );
        }

        let group = super::Group {
            id: notify.group_id.clone(),
            name: notify.group_name.clone(),
            is_direct_chat: false,
            created_at: notify.created_at,
            creator_id: notify.creator_id.clone(),
            members,
        };

        if let Err(error) = groups.db_ref.insert(&group_idx.to_be_bytes(), group) {
            log::error!("group db updating error {}", error.to_string());
        }
        Group::update_groups_of_user(&user_id, groups);

        // save events
        let user_id = PeerId::from_bytes(&receiver_id).unwrap();
        let snd_id = PeerId::from_bytes(&sender_id).unwrap();
        let converstion_id = messaging::ConversationId::from_bytes(&notify.group_id).unwrap();

        if first_join {
            let event = chat::rpc_proto::GroupEvent {
                event_type: chat::rpc_proto::GroupEventType::GroupJoined
                    .try_into()
                    .unwrap(),
                user_id: receiver_id.clone(),
            };
            chat::Chat::save_event(
                &user_id,
                &snd_id,
                chat::rpc_proto::ContentType::GroupEvent.try_into().unwrap(),
                &event.encode_to_vec(),
                &converstion_id,
            );
        } else {
            for new_member in &new_members {
                let event = chat::rpc_proto::GroupEvent {
                    event_type: chat::rpc_proto::GroupEventType::GroupJoined
                        .try_into()
                        .unwrap(),
                    user_id: new_member.clone(),
                };
                chat::Chat::save_event(
                    &user_id,
                    &snd_id,
                    chat::rpc_proto::ContentType::GroupEvent.try_into().unwrap(),
                    &event.encode_to_vec(),
                    &converstion_id,
                );
            }

            for left_member in orign_members.keys() {
                let event = chat::rpc_proto::GroupEvent {
                    event_type: chat::rpc_proto::GroupEventType::GroupLeft
                        .try_into()
                        .unwrap(),
                    user_id: left_member.clone(),
                };
                chat::Chat::save_event(
                    &user_id,
                    &snd_id,
                    chat::rpc_proto::ContentType::GroupEvent.try_into().unwrap(),
                    &event.encode_to_vec(),
                    &converstion_id,
                );
            }
        }
    }
}
