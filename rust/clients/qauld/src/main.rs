// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld - qaul Daemon
//!
//! qaul daemon is running headless in the background.
//! It can be used to run on an embedded device, such as a raspberry Pi,
//! or as a static node on a server in the Internet.

use clap::Parser;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use libqaul;
mod socket;

/// qauld - qaul daemon : CLI Arguments
#[derive(Parser)]
struct CliArguments {
    /// User Name
    #[arg(short, long)]
    name: Option<String>,
    /// Port Number
    #[arg(short, long)]
    port: Option<u16>,
    /// Fork into the background after startup (Unix only).
    ///
    /// stdout/stderr are redirected to `qauld.out` / `qauld.err` and the
    /// process id is written to `qauld.pid`, both in the storage directory.
    /// Stop the daemon with `kill $(cat qauld.pid)`.
    #[arg(short, long)]
    daemonize: bool,
}

/// create a default user account for zero configuration Community Node startups
/// without providing a user name
pub fn create_default_named() -> String {
    let mut user_name: String;
    user_name = "Community Node ".to_string();
    user_name.push_str(
        libqaul::utilities::timestamp::Timestamp::get_timestamp()
            .to_string()
            .as_str(),
    );
    user_name
}

fn main() {
    // Capture the storage path (qauld keeps its data in the working directory)
    // up front: daemonizing changes the working directory, and the fork below
    // must happen before the tokio runtime spawns any threads.
    let storage_path = match std::env::current_dir() {
        Ok(path) => match path.to_str() {
            Some(s) => s.to_string(),
            None => {
                eprintln!("qauld: working directory path is not valid UTF-8");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("qauld: cannot determine working directory: {e}");
            std::process::exit(1);
        }
    };

    let cli_arguments = CliArguments::parse();

    // Daemonize BEFORE building the async runtime. Forking a multi-threaded
    // process (which the tokio runtime would make us) is unsafe, so the fork
    // must happen while we are still single-threaded.
    if cli_arguments.daemonize {
        #[cfg(unix)]
        daemonize(&storage_path);
        #[cfg(not(unix))]
        eprintln!("qauld: --daemonize is only supported on Unix; staying in the foreground");
    }

    // Build the multi-threaded runtime by hand (equivalent to `#[tokio::main]`)
    // so the daemonize fork above can run first.
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("qauld: failed to start async runtime: {e}");
            std::process::exit(1);
        }
    };
    runtime.block_on(run(cli_arguments, storage_path));
}

/// Fork into the background: detach from the controlling terminal, redirect
/// stdout/stderr to log files, and write a PID file — all inside the storage
/// directory, which is preserved as the working directory so qauld's
/// CWD-relative storage keeps working.
///
/// Returns only in the daemon child; the parent process has already exited.
#[cfg(unix)]
fn daemonize(storage_path: &str) {
    use daemonix::Daemonize;

    let dir = std::path::Path::new(storage_path);

    let mut daemon = Daemonize::new()
        .working_directory(storage_path)
        .pid_file(dir.join("qauld.pid"));
    if let Ok(file) = std::fs::File::create(dir.join("qauld.out")) {
        daemon = daemon.stdout(file);
    }
    if let Ok(file) = std::fs::File::create(dir.join("qauld.err")) {
        daemon = daemon.stderr(file);
    }

    // Printed by the parent, before the fork, so the user sees it on the
    // terminal they launched from.
    println!(
        "qauld: daemonizing; pid file {}, logs in qauld.out / qauld.err",
        dir.join("qauld.pid").display()
    );

    if let Err(e) = daemon.start() {
        eprintln!("qauld: failed to daemonize: {e}");
        std::process::exit(1);
    }
}

/// Async entry point: start libqaul, ensure an account exists, and serve the
/// control socket. Runs on the tokio runtime built in `main`.
async fn run(cli_arguments: CliArguments, storage_path: String) {
    // create default config
    let mut def_config: BTreeMap<String, String> = BTreeMap::new();
    {
        if let Some(v) = cli_arguments.name.as_deref() {
            def_config.insert("name".to_string(), v.to_string());
        }
        if let Some(v) = cli_arguments.port {
            def_config.insert("port".to_string(), v.to_string());
        }
    }

    // start libqaul in new thread and save configuration file to current working path
    let instance =
        libqaul::api::start_instance_in_thread(storage_path.clone(), Some(def_config.clone()));

    // wait until libqaul finished initializing
    while !instance.is_initialized() {
        // wait a little while
        std::thread::sleep(Duration::from_millis(10));
    }

    // if no account, creating new accounts
    if libqaul::node::user_accounts::UserAccounts::len(&*instance.state) == 0 {
        let user_name: String = match cli_arguments.name.as_deref() {
            Some(usr_name) => usr_name.to_string(),
            None => create_default_named(),
        };
        libqaul::node::user_accounts::UserAccounts::create(
            &*instance.state,
            user_name.clone(),
            None,
        );
    }

    // since we're now starting a socket server, the main thread now has work to do,
    // so, we can get rid of the loop
    if let Err(err) = socket::start_server(PathBuf::from(storage_path), instance).await {
        eprintln!("Err: {err}")
    }
}
