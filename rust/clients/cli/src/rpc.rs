// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # RPC client functions

use libqaul;
use prost::Message;

/// include generated protobuf RPC rust definition file
pub mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs");
}

/// RPC message communication between client
/// and libqaul.
pub struct Rpc {}

impl Rpc {
    /// encode and send an rpc message to libqaul
    pub fn send_message(data: Vec<u8>, module: i32, request_id: String) {
        // get user
        let my_user_id = super::user_accounts::UserAccounts::get_user_id();

        // check authorisation
        if my_user_id == None {
            if module == proto::Modules::Node as i32 {
                // print message
                println!("Operation not permitted");
                println!("No user account set yet");
                println!("Please create a user account");
                println!("");
                println!("    account create {{Your User Name}}");
                println!("");

                // stop here
                return;
            }
        }

        // create user id
        let user_id;
        if let Some(data) = my_user_id {
            user_id = data;
        } else {
            user_id = Vec::new();
        }

        // Create RPC message container
        let proto_message = proto::QaulRpc {
            module,
            request_id,
            user_id,
            data,
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send the message
        libqaul::api::send_rpc(buf);
    }

    /// receive an rpc message from libqaul
    pub fn received_message(data: Vec<u8>) {
        match proto::QaulRpc::decode(&data[..]) {
            Ok(message) => {
                log::trace!("qaul rpc message received");

                match proto::Modules::try_from(message.module) {
                    Ok(proto::Modules::Node) => {
                        super::node::Node::rpc(message.data);
                    }
                    Ok(proto::Modules::Rpc) => {
                        // TODO: authorisation
                    }
                    Ok(proto::Modules::Useraccounts) => {
                        super::user_accounts::UserAccounts::rpc(message.data);
                    }
                    Ok(proto::Modules::Users) => {
                        super::users::Users::rpc(message.data);
                    }
                    Ok(proto::Modules::Router) => {
                        super::router::Router::rpc(message.data);
                    }
                    Ok(proto::Modules::Feed) => {
                        super::feed::Feed::rpc(message.data);
                    }
                    Ok(proto::Modules::Chat) => {
                        super::chat::Chat::rpc(message.data);
                    }
                    Ok(proto::Modules::Connections) => {
                        super::connections::Connections::rpc(message.data);
                    }
                    Ok(proto::Modules::Ble) => {
                        super::ble::Ble::rpc(message.data);
                    }
                    Ok(proto::Modules::Debug) => {
                        super::debug::Debug::rpc(message.data);
                    }
                    Ok(proto::Modules::Chatfile) => {
                        super::chatfile::ChatFile::rpc(message.data);
                    }
                    Ok(proto::Modules::Group) => {
                        super::group::Group::rpc(message.data);
                    }
                    Ok(proto::Modules::Rtc) => {
                        super::rtc::Rtc::rpc(message.data);
                    }
                    Ok(proto::Modules::Dtn) => {
                        super::dtn::Dtn::rpc(message.data);
                    }
                    Ok(proto::Modules::None) => {}
                    Err(_) => {}
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
