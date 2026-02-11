// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld-ctl - CLI client for controlling a running qauld daemon instance via Unix socket
//!

use std::path::PathBuf;

use clap::Parser;
use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio::net::UnixStream;
use tokio_util::codec::LengthDelimitedCodec;

/// include generated protobuf RPC rust definition file
pub mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs");
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.node.rs");
}

/// qauld-ctl CLI: Control a running qauld daemon instance
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Explicit path to qauld sock. e.g /path/to/qauld.sock
    #[arg(short, long, env = "QAULD_SOCKET")]
    socket: Option<String>,
    /// Specify a directory to look for qauld.sock in
    #[arg(short, long)]
    dir: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    log::info!("qauld-ctl connected to qauld daemon at: {qauld_sock}");

    let mut framed_client = LengthDelimitedCodec::builder()
        .length_field_offset(0)
        .length_field_type::<u16>()
        .length_adjustment(0)
        .new_framed(client);

    // create info request message
    let node_proto_message = proto::Node {
        message: Some(proto::node::Message::GetNodeInfo(true)),
    };

    // encode message
    let mut buf = Vec::with_capacity(node_proto_message.encoded_len());
    node_proto_message
        .encode(&mut buf)
        .expect("Vec<u8> provides capacity as needed");

    // Create RPC message container
    let proto_message = proto::QaulRpc {
        module: proto::Modules::Node.into(),
        request_id: String::from("test-id"),
        user_id: Vec::new(),
        data: buf,
    };

    let mut rpc_msg = Vec::with_capacity(proto_message.encoded_len());
    proto_message
        .encode(&mut rpc_msg)
        .expect("Vec<u8> provides capacity as needed");
    framed_client.send(rpc_msg).await?;

    if let Some(Ok(data)) = framed_client.next().await {
        match proto::QaulRpc::decode(&data[..]) {
            Ok(msg) => {
                println!("{msg:#?}")
            }
            _ => {}
        }
    }

    Ok(())
}
