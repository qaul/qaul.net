// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Unix socket server for qauld

use std::{fs, path::PathBuf};

use tokio::{net::UnixListener, signal};

/// Starts the qauld unix socket server.
/// Runs infinitely until a shutdown signal is receibed.
/// It accepts connections on `qauld.sock` in cwd,
/// forwards requests to libqaul, and sends responses back to clients.
pub async fn start_server(socket_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = socket_dir.join("qauld.sock");
    if socket_path.exists() {
        fs::remove_file(&socket_path)?;
    }

    let listener = UnixListener::bind(&socket_path)?;
    println!("qauld unix socket server started");

    loop {
        tokio::select! {
            res = listener.accept() => {
                println!("client connected");
            },
            _ = signal::ctrl_c() => {
                println!("shutdown triggered");
                break;
            }
        }
    }

    fs::remove_file(socket_path)?;

    Ok(())
}
