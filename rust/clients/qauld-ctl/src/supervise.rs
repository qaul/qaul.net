// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Supervised mode (`qauld-ctl run`): spawn qauld as a child process,
//! mirror its stdout/stderr to our stderr, wait for Ctrl-C, then
//! cleanly tear down the child.

use std::process::Stdio;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::signal;

use crate::cli::{Cli, Commands};

pub async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let args = match cli.command {
        Commands::Run(ref a) => a,
        _ => return Err("supervise::run called with non-Run command".into()),
    };
    let qauld_path = args
        .qauld_path
        .clone()
        .unwrap_or_else(|| "qauld".to_string());

    let mut command = Command::new(&qauld_path);
    // Run qauld in the directory the user pointed at — qauld picks
    // its storage path from cwd.
    if let Some(dir) = &cli.dir {
        command.current_dir(dir);
    }
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    // Forward our env (RUST_LOG, env_log_level, etc.) so daemon log
    // verbosity is controllable from the user's shell.

    let mut child = command
        .spawn()
        .map_err(|e| format!("failed to spawn `{qauld_path}`: {e}"))?;
    if cli.verbose {
        eprintln!(
            "qauld-ctl: started qauld (pid {})",
            child.id().unwrap_or(0)
        );
    }

    // Stream child's stdout + stderr to our stderr so the user sees
    // exactly what qauld is printing, regardless of how they invoked us.
    if let Some(out) = child.stdout.take() {
        tokio::spawn(forward_lines(BufReader::new(out), "stdout"));
    }
    if let Some(err) = child.stderr.take() {
        tokio::spawn(forward_lines(BufReader::new(err), "stderr"));
    }

    // Wait for either the daemon to die on its own or for Ctrl-C.
    tokio::select! {
        status = child.wait() => {
            match status {
                Ok(s) => {
                    if !s.success() {
                        return Err(format!("qauld exited: {s}").into());
                    }
                    Ok(())
                }
                Err(e) => Err(format!("qauld wait failed: {e}").into()),
            }
        }
        _ = signal::ctrl_c() => {
            if cli.verbose {
                eprintln!("qauld-ctl: SIGINT received, asking qauld to exit");
            }
            // Best-effort graceful shutdown: send SIGINT to the child
            // (it has its own ctrl_c handler).
            #[cfg(unix)]
            unsafe {
                if let Some(pid) = child.id() {
                    libc::kill(pid as i32, libc::SIGINT);
                }
            }
            #[cfg(not(unix))]
            {
                let _ = child.kill().await;
            }

            // Give the child up to 5 seconds to shut down cleanly,
            // then SIGKILL.
            let deadline = std::time::Instant::now() + Duration::from_secs(5);
            loop {
                if std::time::Instant::now() >= deadline {
                    let _ = child.kill().await;
                    break;
                }
                match child.try_wait() {
                    Ok(Some(_)) => break,
                    Ok(None) => tokio::time::sleep(Duration::from_millis(100)).await,
                    Err(_) => {
                        let _ = child.kill().await;
                        break;
                    }
                }
            }
            Ok(())
        }
    }
}

async fn forward_lines<R: tokio::io::AsyncRead + Unpin>(reader: BufReader<R>, _which: &'static str) {
    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        eprintln!("[qauld] {line}");
    }
}
