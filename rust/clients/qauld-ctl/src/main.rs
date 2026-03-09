// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld-ctl - CLI client for controlling a running qauld daemon instance via Unix socket

use std::path::PathBuf;

use clap::Parser;
use cli::{Cli, Commands};
use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use uuid::Uuid;

#[cfg(windows)]
use tokio::net::TcpStream;
#[cfg(unix)]
use tokio::net::UnixStream;

use crate::commands::RpcCommand;

/// protobuf RPC definition
pub use qaul_proto::qaul_rpc as proto;

mod cli;
mod commands;

/// Default TCP address used for windows
#[cfg(windows)]
const DEFAULT_TCP_ADDR: &str = "127.0.0.1:9199";

/// A Pre flight requesst to get the user ID before executing any command
async fn preflight_request<T>(
    client: &mut Framed<T, LengthDelimitedCodec>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
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

    let user_id_bytes = if let Some(Ok(data)) = client.next().await {
        match proto::QaulRpc::decode(&data[..]) {
            Ok(msg) => commands::decode_default_user(&msg.data),
            _ => {
                log::warn!("preflight: failed to decode RPC envelope");
                Vec::new()
            }
        }
    } else {
        log::warn!("preflight: no response received");
        Vec::new()
    };

    log::info!("preflight request completed");
    Ok(user_id_bytes)
}

async fn run<T>(client: T, cli: Cli) -> Result<(), Box<dyn std::error::Error>>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
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
        Commands::Chat(c) => Box::new(c.command) as Box<dyn RpcCommand>,
        Commands::File(f) => Box::new(f.command) as Box<dyn RpcCommand>,
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
                Ok(msg) => rpc_command.decode_response(&msg.data[..], cli.json)?,
                _ => {}
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let cli = Cli::parse();

    #[cfg(unix)]
    {
        let qauld_sock = if let Some(socket) = &cli.socket {
            socket.clone()
        } else if let Some(socket_dir) = &cli.dir {
            let path = PathBuf::from(socket_dir).join("qauld.sock");
            path.to_str()
                .expect("failed to get name of dir")
                .to_string()
        } else {
            "qauld.sock".to_string()
        };

        let client = UnixStream::connect(&qauld_sock).await?;
        println!("qauld-ctl connected to qauld daemon at: {qauld_sock}");
        run(client, cli).await?;
    }

    #[cfg(windows)]
    {
        let addr = if let Some(socket) = &cli.socket {
            socket.clone()
        } else {
            DEFAULT_TCP_ADDR.to_string()
        };

        let client = TcpStream::connect(&addr).await?;
        println!("qauld-ctl connected to qauld daemon at: {addr}");
        run(client, cli).await?;
    }

    Ok(())
}
