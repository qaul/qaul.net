// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld-ctl - CLI client for controlling a running qauld daemon instance via Unix socket

use std::path::PathBuf;

use clap::Parser;
use cli::{Cli, Commands};
use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio::net::UnixStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use uuid::Uuid;

use crate::commands::RpcCommand;

/// include generated protobuf RPC rust definition file
pub mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.node.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.user_accounts.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.users.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.feed.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.group.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.chat.rs");
}

mod cli;
mod commands;

/// A Pre flight requesst to get the user ID before executing any command
async fn preflight_request(
    client: &mut Framed<UnixStream, LengthDelimitedCodec>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    log::info!("executing preflight request");
    let (data, module) = commands::default_user_proto_message();
    let request_id = Uuid::new_v4().to_string();

    let proto_message = proto::QaulRpc {
        module: module.into(),
        request_id,
        user_id: Vec::new(),
        data,
    };

    let mut rpc_msg = Vec::with_capacity(proto_message.encoded_len());
    proto_message
        .encode(&mut rpc_msg)
        .expect("Vec<u8> provides capacity as needed");
    client.send(rpc_msg.into()).await?;

    let (user_id_bytes, id) = if let Some(Ok(data)) = client.next().await {
        match proto::QaulRpc::decode(&data[..]) {
            Ok(msg) => {
                let user_accounts = proto::UserAccounts::decode(&msg.data[..])?;
                match user_accounts.message {
                    Some(proto::user_accounts::Message::DefaultUserAccount(
                        default_useraccount,
                    )) => {
                        if default_useraccount.user_account_exists {
                            if let Some(my_user_account) = default_useraccount.my_user_account {
                                (my_user_account.id, my_user_account.id_base58)
                            } else {
                                log::warn!("user account not found");
                                (Vec::new(), "".to_string())
                            }
                        } else {
                            log::warn!("default account not found");
                            (Vec::new(), "".to_string())
                        }
                    }
                    _ => {
                        log::error!("failed to parse RPC message");
                        (Vec::new(), "".to_string())
                    }
                }
            }
            _ => {
                log::error!("failed to decode RPC");
                (Vec::new(), "".to_string())
            }
        }
    } else {
        (Vec::new(), "".to_string())
    };

    log::info!("preflight request succedded for user_id: {id}");
    Ok(user_id_bytes)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();
    let qauld_sock = if let Some(socket) = cli.socket {
        socket
    } else if let Some(socket_dir) = cli.dir {
        let path = PathBuf::from(socket_dir).join("qauld.sock");
        path.to_str()
            .expect("failed to get name of dir")
            .to_string()
    } else {
        "qauld.sock".to_string()
    };

    let client = UnixStream::connect(&qauld_sock).await?;
    println!("qauld-ctl connected to qauld daemon at: {qauld_sock}");

    let mut framed_client = LengthDelimitedCodec::builder()
        .length_field_offset(0)
        .length_field_type::<u16>()
        .length_adjustment(0)
        .new_framed(client);

    let request_id = Uuid::new_v4().to_string();
    let user_id = preflight_request(&mut framed_client).await?;

    let rpc_command: Box<dyn RpcCommand> = match cli.command {
        Commands::Node(c) => Box::new(c.command) as Box<dyn RpcCommand>,
        Commands::Account(a) => Box::new(a.command) as Box<dyn RpcCommand>,
        Commands::Users(u) => Box::new(u.command) as Box<dyn RpcCommand>,
        Commands::Feed(f) => Box::new(f.command) as Box<dyn RpcCommand>,
        Commands::Group(g) => Box::new(g.command) as Box<dyn RpcCommand>,
    };

    let (data, module) = rpc_command.encode_request()?;
    // Create RPC message container
    let proto_message = proto::QaulRpc {
        module: module.into(),
        request_id,
        user_id,
        data,
    };

    let mut rpc_msg = Vec::with_capacity(proto_message.encoded_len());
    proto_message
        .encode(&mut rpc_msg)
        .expect("Vec<u8> provides capacity as needed");
    framed_client.send(rpc_msg.into()).await?;

    if rpc_command.expects_response() {
        if let Some(Ok(data)) = framed_client.next().await {
            match proto::QaulRpc::decode(&data[..]) {
                Ok(msg) => rpc_command.decode_response(&msg.data[..])?,
                _ => {}
            }
        }
    }

    Ok(())
}
