//use bs58::decode;
use libp2p::{
    identity::{ed25519, Keypair},
    PeerId,
};

use crate::utilities::timestamp;
use std::collections::BTreeMap;

use super::Group;

pub struct Manage{}
impl Manage{
    /// create new group from rpc command
    pub fn create_new_group(user_id: &PeerId, name: String) -> Vec<u8>{
        let keys_ed25519 = ed25519::Keypair::generate();
        let keys = Keypair::Ed25519(keys_ed25519);
        let id = PeerId::from(keys.public());                

        let mut members = BTreeMap::new();
        members.insert(user_id.to_bytes(), 
            super::GroupMember{
            user_id: user_id.to_bytes(),
            role: 255, //admin
            joined_at: timestamp::Timestamp::get_timestamp(),
            state: 1,
            }
        );

        let new_group = super::Group{
            id: id.to_bytes(),
            name,
            created_at: timestamp::Timestamp::get_timestamp(),
            creator_id: user_id.to_bytes(),
            members, 
        };

        let mut groups = Group::get_groups_of_user(user_id.clone());


        let key = groups.last_group + 1;
        //db save
        if let Err(error) = groups.db_ref.insert(&key.to_be_bytes(), new_group){
            log::error!("group db updating error {}", error.to_string());            
        }
        //add new id
        groups.last_group = key;
        groups.group_ids.insert(id.to_bytes(), key);
        Group::update_groups_of_user(user_id.clone(), groups);

        return id.to_bytes();
    }

    /// remove group from rpc command
    pub fn rename_group(user_id: &PeerId, group_id: &Vec<u8>, name: String) ->Result<Vec<u8>, String>{
        let groups = Group::get_groups_of_user(user_id.clone());
        let group_idx = groups.group_id_to_index(group_id);
        if group_idx == 0{
            return Err("can not find group".to_string());
        }

        let mut group = groups.db_ref.get(&group_idx.to_be_bytes()).unwrap().unwrap();
        if let Some(member) = group.get_member(&user_id.to_bytes()){
            //check permission
            if member.role != 255{
                return Err("you have not permission for rename this group".to_string());
            }
        }else{
            return Err("you are not member for this group".to_string());
        }

        //rename and save db
        group.name = name.clone();
        if let Err(error) = groups.db_ref.insert(&group_idx.to_be_bytes(), group){
            log::error!("group db updating error {}", error.to_string());            
        }
        Ok(group_id.clone())
    } 

    /// get group information from rpc command
    pub fn group_info(user_id: &PeerId, group_id: &Vec<u8>) ->Result<super::proto_rpc::GroupInfoResponse, String>{
        let groups = Group::get_groups_of_user(user_id.clone());

        let group_idx = groups.group_id_to_index(group_id);
        if group_idx == 0{
            return Err("can not find group".to_string());
        }
        
        let group = groups.db_ref.get(&group_idx.to_be_bytes()).unwrap().unwrap();
        let mut members :Vec<super::proto_rpc::GroupMember> = vec![];
        for m in group.members.values(){
            let member = super::proto_rpc::GroupMember{
                user_id: m.user_id.clone(),
                role: m.role as u32,
                joined_at: m.joined_at,
                state: m.state as u32,
            };
            members.push(member);
        }

        let res = super::proto_rpc::GroupInfoResponse{
            group_id: group.id,
            group_name: group.name,
            created_at: group.created_at,
            members,
        };
        Ok(res)
    } 
    
    /// get group list from rpc command
    pub fn group_list(user_id: &PeerId) ->super::proto_rpc::GroupListResponse{        
        let groups = Group::get_groups_of_user(user_id.clone());

        let mut res =  super::proto_rpc::GroupListResponse{
            groups: vec![],
        };

        for entry in groups.db_ref.iter(){
            match entry{
                Ok((_, group)) =>{
                    let mut members :Vec<super::proto_rpc::GroupMember> = vec![];
                    for m in group.members.values(){
                        let member = super::proto_rpc::GroupMember{
                            user_id: m.user_id.clone(),
                            role: m.role as u32,
                            joined_at: m.joined_at,
                            state: m.state as u32,
                        };
                        members.push(member);
                    }

                    let grp = super::proto_rpc::GroupInfoResponse{
                        group_id: group.id,
                        group_name: group.name,
                        created_at: group.created_at,
                        members,
                    };
                    res.groups.push(grp);
                }
                _ => {}
            }
        }
        res
    }     

    /// process group notify message from network
    pub fn on_group_notify(_sender_id: &Vec<u8>, receiver_id: &Vec<u8>, notify: &super::proto_net::GroupNotify){
        let user_id = PeerId::from_bytes(receiver_id).unwrap();
        let mut groups = Group::get_groups_of_user(user_id);

        let mut group_idx = groups.group_id_to_index(&notify.group_id);        

        if group_idx == 0{
            group_idx = groups.last_group + 1;
            groups.last_group = group_idx;
            groups.group_ids.insert(notify.group_id.clone(), group_idx);
        }

        let mut members: BTreeMap<Vec<u8>, super::GroupMember> = BTreeMap::new();
        for m in &notify.members{
            members.insert(m.user_id.clone(), super::GroupMember{
                user_id: m.user_id.clone(),
                role: m.role as u8,
                joined_at: m.joined_at,
                state: m.state as u8,
            });
        }

        let group = super::Group{
            id: notify.group_id.clone(),
            name: notify.group_name.clone(),
            created_at: notify.created_at,
            creator_id: notify.creator_id.clone(),
            members,
        };
        if let Err(error) = groups.db_ref.insert(&group_idx.to_be_bytes(), group){
            log::error!("group db updating error {}", error.to_string());
        }
        Group::update_groups_of_user(user_id.clone(), groups);

    }

}
