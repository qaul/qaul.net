//use bs58::decode;
use libp2p::PeerId;

use crate::node::user_accounts::UserAccounts;
use crate::rpc::Rpc;
use crate::services::messaging;
use prost::Message;

pub struct RtcMessaging {}
impl RtcMessaging {
    /// process rtc message command from cli
    pub fn send_message(
        my_user_id: &PeerId,
        req: &super::proto_rpc::RtcOutgoing,
    ) -> Result<bool, String> {
        match super::Rtc::get_session_from_id(&req.group_id) {
            Some(session) => {
                if session.state != 3 {
                    return Err("session is not established".to_string());
                }

                // make message and send on the messaging service
                let proto_message = super::proto_net::RtcMessage {
                    sequence: 0,
                    content: req.content.clone(),
                };

                let send_message = messaging::proto::Messaging {
                    message: Some(messaging::proto::messaging::Message::RtcStreamMessage(
                        messaging::proto::RtcStreamMessage {
                            content: proto_message.encode_to_vec(),
                        },
                    )),
                };

                let message_id: Vec<u8> = Vec::new();
                if let Some(user_account) = UserAccounts::get_by_id(*my_user_id) {
                    let receiver = PeerId::from_bytes(&req.group_id).unwrap();
                    if let Err(e) = messaging::Messaging::pack_and_send_message(
                        &user_account,
                        &receiver,
                        send_message.encode_to_vec(),
                        messaging::MessagingServiceType::Unconfirmed,
                        &message_id,
                        false,
                    ) {
                        log::error!("error {}", e);
                    }
                } else {
                    return Err("user account has problem".to_string());
                }
                return Ok(true);
            }
            None => {
                return Err("session does not exist".to_string());
            }
        }
    }

    /// proccess message from network
    #[allow(dead_code)]
    pub fn on_message(
        sender_id: &PeerId,
        _receiver_id: &PeerId,
        req: &super::proto_net::RtcMessage,
        _signature: Vec<u8>,
    ) {
        match super::Rtc::get_session_from_id(&sender_id.to_bytes()) {
            None => {
                log::error!(
                    "on_message session does not exist {}",
                    sender_id.to_string()
                );
            }
            Some(_session) => {
                // make session rpc and response to cli
                let proto_message = super::proto_rpc::RtcRpc {
                    message: Some(super::proto_rpc::rtc_rpc::Message::RtcIncoming(
                        super::proto_rpc::RtcIncoming {
                            group_id: sender_id.to_bytes(),
                            content: req.content.clone(),
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
                    crate::rpc::proto::Modules::Rtc.into(),
                    "".to_string(),
                    Vec::new(),
                );
            }
        }
    }
}
