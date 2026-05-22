// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld-ctl - CLI client for controlling a running qauld daemon instance via Unix socket

use std::time::Duration;

use clap::Parser;
use cli::{Cli, Commands};
use uuid::Uuid;

use crate::commands::RpcCommand;
use crate::transport::RpcTransport;

/// protobuf RPC definition
pub use qaul_proto::qaul_rpc as proto;

mod cli;
mod commands;
mod shell;
mod subscribe;
mod transport;

/// A pre-flight request to get the user ID before executing any command.
async fn preflight_request(
    transport: &mut dyn RpcTransport,
    timeout: Duration,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    log::info!("executing preflight request");
    let (data, module) = commands::default_user_proto_message();
    let envelope = proto::QaulRpc {
        module: module.into(),
        request_id: Uuid::new_v4().to_string(),
        user_id: Vec::new(),
        data,
    };

    let user_id_bytes = match transport.request(envelope, timeout, true).await? {
        Some(resp) => commands::decode_default_user(&resp.data),
        None => {
            log::warn!("preflight: no response received");
            Vec::new()
        }
    };

    log::info!("preflight request completed");
    Ok(user_id_bytes)
}

/// Open a transport to qauld and connect; pure socket-mode entry point.
/// Kept so shell-mode + subscribe-mode (which need raw stream access)
/// can keep using it directly. Embedded mode goes through `run` via
/// `EmbeddedTransport` instead.
#[cfg(unix)]
pub(crate) async fn connect_to_qauld(
    cli: &Cli,
) -> Result<(tokio::net::UnixStream, String), Box<dyn std::error::Error>> {
    use std::path::PathBuf;
    let path = if let Some(socket) = &cli.socket {
        socket.clone()
    } else if let Some(dir) = &cli.dir {
        PathBuf::from(dir)
            .join("qauld.sock")
            .to_string_lossy()
            .into_owned()
    } else {
        "qauld.sock".to_string()
    };
    let stream = tokio::net::UnixStream::connect(&path).await?;
    Ok((stream, path))
}

#[cfg(windows)]
pub(crate) async fn connect_to_qauld(
    cli: &Cli,
) -> Result<(tokio::net::TcpStream, String), Box<dyn std::error::Error>> {
    let addr = cli
        .socket
        .clone()
        .unwrap_or_else(|| "127.0.0.1:9199".to_string());
    let stream = tokio::net::TcpStream::connect(&addr).await?;
    Ok((stream, addr))
}

/// Dispatch a single command through any transport.
pub(crate) async fn run(
    transport: &mut dyn RpcTransport,
    cli: Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let timeout = Duration::from_secs(cli.timeout);
    let user_id = preflight_request(transport, timeout).await?;

    let rpc_command: Box<dyn RpcCommand> = match cli.command {
        Commands::Node(c) => Box::new(c.command) as Box<dyn RpcCommand>,
        Commands::Account(a) => Box::new(a.command) as Box<dyn RpcCommand>,
        Commands::Users(u) => Box::new(u.command) as Box<dyn RpcCommand>,
        Commands::Feed(f) => Box::new(f.command) as Box<dyn RpcCommand>,
        Commands::Group(g) => Box::new(g.command) as Box<dyn RpcCommand>,
        Commands::Chat(c) => Box::new(c.command) as Box<dyn RpcCommand>,
        Commands::File(f) => Box::new(f.command) as Box<dyn RpcCommand>,
        Commands::Router(r) => Box::new(r.command) as Box<dyn RpcCommand>,
        Commands::Crypto(c) => Box::new(c.command) as Box<dyn RpcCommand>,
        Commands::Debug(d) => Box::new(d.command) as Box<dyn RpcCommand>,
        Commands::Connections(c) => Box::new(c.command) as Box<dyn RpcCommand>,
        Commands::Dtn(d) => Box::new(d.command) as Box<dyn RpcCommand>,
        Commands::Transports(t) => Box::new(t.command) as Box<dyn RpcCommand>,
        Commands::Ble(b) => Box::new(b.command) as Box<dyn RpcCommand>,
        Commands::Auth(a) => Box::new(a.command) as Box<dyn RpcCommand>,
        #[cfg(feature = "rtc")]
        Commands::Rtc(r) => Box::new(r.command) as Box<dyn RpcCommand>,
        // Shell and Subscribe modes are dispatched in `main` before reaching
        // `run`, so these arms are unreachable. The match is kept exhaustive
        // for clarity.
        Commands::Shell(_) | Commands::Subscribe(_) => return Ok(()),
    };

    let (data, module) = rpc_command.encode_request()?;
    let envelope = proto::QaulRpc {
        module: module.into(),
        request_id: Uuid::new_v4().to_string(),
        user_id,
        data,
    };

    let response = transport
        .request(envelope, timeout, rpc_command.expects_response())
        .await?;
    if let Some(resp) = response {
        rpc_command.decode_response(&resp.data[..], cli.json)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let cli = Cli::parse();

    // Shell mode runs its own loop and opens sockets per command (uses
    // SocketTransport internally via the helpers in this module).
    if matches!(cli.command, Commands::Shell(_)) {
        return shell::run(cli).await;
    }
    // Subscribe mode is a long-running RPC that streams events back;
    // it can't reuse the single-shot path.
    if matches!(cli.command, Commands::Subscribe(_)) {
        return subscribe::run(cli).await;
    }

    #[cfg(feature = "embedded")]
    {
        let mut transport = transport::EmbeddedTransport::start(&cli).await?;
        if cli.verbose {
            eprintln!("qauld-ctl running with embedded libqaul");
        }
        run(&mut transport, cli).await?;
    }
    #[cfg(not(feature = "embedded"))]
    {
        let mut transport = transport::SocketTransport::connect(&cli).await?;
        if cli.verbose {
            eprintln!("qauld-ctl connected to qauld daemon at: {}", transport.addr);
        }
        run(&mut transport, cli).await?;
    }

    Ok(())
}
