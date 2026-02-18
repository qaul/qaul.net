// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld-ctl - CLI client for controlling a running qauld daemon instance via Unix socket

use std::path::PathBuf;

use clap::Parser;
use cli::{Cli, Commands};
use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio::net::UnixStream;
use tokio_util::codec::LengthDelimitedCodec;
use uuid::Uuid;

use crate::commands::RpcCommand;

/// include generated protobuf RPC rust definition file
pub mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.node.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.user_accounts.rs");
}

mod cli;
mod commands;

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

    let rpc_command: Box<dyn RpcCommand> = match cli.command {
        Commands::Node(c) => Box::new(c.command) as Box<dyn RpcCommand>,
        Commands::Account(a) => Box::new(a.command) as Box<dyn RpcCommand>,
    };

    let request_id = Uuid::new_v4().to_string();
    let (data, module) = rpc_command.encode_request()?;
    // Create RPC message container
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
    framed_client.send(rpc_msg.into()).await?;

    if let Some(Ok(data)) = framed_client.next().await {
        match proto::QaulRpc::decode(&data[..]) {
            Ok(msg) => rpc_command.decode_response(&msg.data[..])?,
            _ => {}
        }
    }

    Ok(())
}
