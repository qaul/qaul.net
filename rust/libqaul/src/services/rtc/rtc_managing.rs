//use bs58::decode;
use super::Rtc;
use crate::{node::user_accounts::UserAccounts, utilities::timestamp};
use libp2p::PeerId;
use prost::Message;

pub struct RtcManaging {}
impl RtcManaging {
    /// process session list from cli
    pub fn session_list(_my_user_id: &PeerId) -> super::proto_rpc::RtcSessionListResponse {
        let mut res = super::proto_rpc::RtcSessionListResponse { sessions: vec![] };

        let sessions = super::RTCSESSIONS.get().read().unwrap();
        for (_id, session) in sessions.sessions.iter() {
            let entry = super::proto_rpc::RtcSession {
                group_id: session.group_id.clone(),
                session_type: session.session_type,
                state: session.state as u32,
                created_at: session.created_at,
            };
            res.sessions.push(entry);
        }
        res
    }
    /// process session request from cli
    pub fn session_request(
        my_user_id: &PeerId,
        req: &super::proto_rpc::RtcSessionRequest,
    ) -> Result<Vec<u8>, String> {
        //check if session already exists
        if let Some(_session) = super::Rtc::get_session_from_id(&req.group_id) {
            return Err("session alrady exists!".to_string());
        }

        //insert new session entry
        let session = super::RtcSession {
            user_id: my_user_id.to_bytes(),
            group_id: req.group_id.clone(),
            session_type: 1,
            state: 1, //sent request
            created_at: timestamp::Timestamp::get_timestamp(),
        };
        super::Rtc::update_session(session);

        //send session request message on the messaging service
        let proto_message = super::proto_net::RtcContainer {
            message: Some(super::proto_net::rtc_container::Message::RtcSessionRequest(
                super::proto_net::RtcSessionRequest { session_type: 1 },
            )),
        };

        let mut message_buff = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut message_buff)
            .expect("Vec<u8> provides capacity as needed");

        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id) {
            let receiver = PeerId::from_bytes(&req.group_id).unwrap();
            super::Rtc::send_rtc_message_through_message(&user_account, receiver, &message_buff);
        } else {
            return Err("user account has problem".to_string());
        }

        Ok(req.group_id.clone())
    }

    /// send session management message
    fn send_session_management(
        my_user_id: &PeerId,
        group_id: &Vec<u8>,
        option: u32,
    ) -> Result<bool, String> {
        //send message on the messaging service
        let proto_message = super::proto_net::RtcContainer {
            message: Some(
                super::proto_net::rtc_container::Message::RtcSessionManagement(
                    super::proto_net::RtcSessionManagement { option },
                ),
            ),
        };

        let mut message_buff = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut message_buff)
            .expect("Vec<u8> provides capacity as needed");

        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id) {
            let receiver = PeerId::from_bytes(group_id).unwrap();
            super::Rtc::send_rtc_message_through_message(&user_account, receiver, &message_buff);
        } else {
            return Err("user account has problem".to_string());
        }
        Ok(true)
    }

    /// process session mangement request from cli
    pub fn session_management(
        my_user_id: &PeerId,
        req: &super::proto_rpc::RtcSessionManagement,
    ) -> Result<Vec<u8>, String> {
        //check if session already exists
        match super::Rtc::get_session_from_id(&req.group_id) {
            Some(mut session) => {
                match req.option {
                    1 => {
                        //update entry
                        session.created_at = timestamp::Timestamp::get_timestamp();
                        session.state = 3;
                        super::Rtc::update_session(session.clone());
                    }
                    2 => {
                        //decline
                        super::Rtc::remove_session(&req.group_id);
                    }
                    3 => {
                        //end
                        super::Rtc::remove_session(&req.group_id);
                    }
                    _ => {
                        return Err("unknown session management option".to_string());
                    }
                }
                if let Err(error) =
                    Self::send_session_management(my_user_id, &session.group_id.clone(), req.option)
                {
                    return Err(error);
                }
                Ok(session.group_id.clone())
            }
            None => Err("session dese not exists!".to_string()),
        }
    }

    /// process session request from network
    pub fn on_session_request(
        sender_id: &PeerId,
        receiver_id: &PeerId,
        _req: &super::proto_net::RtcSessionRequest,
    ) {
        // check session already exist
        if let Some(_session) = super::Rtc::get_session_from_id(&sender_id.to_bytes()) {
            log::error!("session already exists id: {}", sender_id.to_string());
            return;
        }

        // make new entry
        let session = super::RtcSession {
            user_id: receiver_id.to_bytes(),
            group_id: sender_id.to_bytes(),
            session_type: 1,
            state: 2, //received state
            created_at: timestamp::Timestamp::get_timestamp(),
        };
        super::Rtc::update_session(session);
    }

    /// process session request from network
    pub fn on_session_management(
        sender_id: &PeerId,
        _receiver_id: &PeerId,
        req: &super::proto_net::RtcSessionManagement,
    ) {
        match super::Rtc::get_session_from_id(&sender_id.to_bytes()) {
            Some(mut session) => {
                match req.option {
                    1 => {
                        //established
                        session.state = 3;
                        session.created_at = timestamp::Timestamp::get_timestamp();
                        Rtc::update_session(session);
                    }
                    2 => {
                        //decline
                        super::Rtc::remove_session(&sender_id.to_bytes());
                    }
                    3 => {
                        //end
                        super::Rtc::remove_session(&sender_id.to_bytes());
                    }
                    _ => {
                        log::error!("session management unknown option: {}", req.option);
                    }
                }
            }
            None => {
                log::error!(
                    "session management dose not exist session: {}",
                    sender_id.to_string()
                );
            }
        }
        // save chat messge
        //Chat::save_incoming_group_invite_reply_message(receiver_id.clone(), sender_id.clone(), &resp.group_id, resp.accept, signature);
    }
}
