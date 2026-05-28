// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld-ctl - CLI client for controlling a running qauld daemon instance via Unix socket

use std::time::Duration;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use uuid::Uuid;

use qauld_rpc::transport::{ConnectInfo, SocketTransport};
use qauld_rpc::RpcTransport;

use crate::commands::RpcCommand;

/// protobuf RPC definition
pub use qauld_rpc::proto;

mod cli;
mod commands;
mod shell;
mod subscribe;
mod supervise;
#[cfg(feature = "tui")]
mod tui;

/// Build a `ConnectInfo` from the parsed CLI flags.
pub(crate) fn connect_info(cli: &Cli) -> ConnectInfo {
    ConnectInfo {
        socket: cli.socket.clone(),
        dir: cli.dir.clone(),
    }
}

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
        // Shell, Subscribe, Completions, Run modes are dispatched in
        // `main` before reaching `run`. Match is kept exhaustive for
        // clarity.
        Commands::Shell(_)
        | Commands::Subscribe(_)
        | Commands::Completions { .. }
        | Commands::Run(_) => return Ok(()),
        #[cfg(feature = "tui")]
        Commands::Tui(_) => return Ok(()),
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

    // Completion generation never touches the daemon.
    if let Commands::Completions { shell } = cli.command {
        let mut cmd = Cli::command();
        let name = cmd.get_name().to_string();
        clap_complete::generate(
            clap_complete::Shell::from(shell),
            &mut cmd,
            name,
            &mut std::io::stdout(),
        );
        return Ok(());
    }

    // Supervised daemon mode bypasses the transport entirely.
    if matches!(cli.command, Commands::Run(_)) {
        return supervise::run(cli).await;
    }

    // Shell mode runs its own loop and opens sockets per command.
    if matches!(cli.command, Commands::Shell(_)) {
        return shell::run(cli).await;
    }
    // Subscribe mode is a long-running RPC that streams events back.
    if matches!(cli.command, Commands::Subscribe(_)) {
        return subscribe::run(cli).await;
    }
    // TUI takes over the terminal; it owns its own event loop.
    #[cfg(feature = "tui")]
    if let Commands::Tui(args) = &cli.command {
        let refresh = args.refresh;
        return tui::run(cli, refresh).await;
    }

    #[cfg(feature = "embedded")]
    {
        let storage_path = cli
            .dir
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap().to_string_lossy().into_owned());
        let mut transport = qauld_rpc::EmbeddedTransport::start(storage_path).await?;
        if cli.verbose {
            eprintln!("qauld-ctl running with embedded libqaul");
        }
        run(&mut transport, cli).await?;
    }
    #[cfg(not(feature = "embedded"))]
    {
        let mut transport = SocketTransport::connect(&connect_info(&cli)).await?;
        if cli.verbose {
            eprintln!("qauld-ctl connected to qauld daemon at: {}", transport.addr);
        }
        run(&mut transport, cli).await?;
    }

    Ok(())
}
