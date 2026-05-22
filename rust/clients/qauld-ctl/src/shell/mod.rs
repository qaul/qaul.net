// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Interactive shell mode for qauld-ctl.
//!
//! Reads command lines from stdin, parses each line through the same `clap`
//! grammar used by single-shot mode, and dispatches them via the existing
//! `run` function. A fresh socket is opened per command — the same code path
//! that single-shot uses, so behaviour is identical aside from the prompt
//! and the persistent process.
//!
//! The shell also opens a long-running event subscription on a separate
//! socket and prints incoming events in-line. When an event arrives while
//! the user is typing, we print the event on its own line and re-print the
//! prompt; characters typed before the event remain in the kernel's line
//! buffer (canonical mode) and will be delivered on Enter, but they will
//! visually appear on the new prompt line.
//!
//! Limitations of this initial implementation:
//! - No line editing or history (we use canonical-mode line input via
//!   `tokio::io::stdin`); add `rustyline` or similar later if the
//!   experience demands it.
//! - Argument tokenisation uses `split_whitespace`, so quoted arguments
//!   (`feed send -m "hello world"`) won't parse correctly. Use single-word
//!   arguments for now, or add a shell-style splitter (e.g. `shlex`) later.

use clap::Parser;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::cli::{Cli, Commands};

const PROMPT: &[u8] = b"qauld> ";

/// Run the interactive shell loop until the user types `quit` / `exit`
/// or sends EOF (Ctrl-D).
///
/// `shell_cli` carries the connection flags (`--socket`, `--dir`, `--json`)
/// the user supplied when launching qauld-ctl. Per-line re-parsed `--socket`
/// / `--dir` are ignored so the connection target is predictable; per-line
/// `--json` is honoured per command.
pub async fn run(shell_cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    print_banner();

    // Open a background event subscription. If the daemon isn't running
    // yet we surface a warning but keep the shell usable — the user can
    // still type commands; each command will fail with the same
    // "connection failed" message until the daemon is up.
    let mut event_rx = match crate::subscribe::spawn_event_listener(&shell_cli).await {
        Ok(rx) => Some(rx),
        Err(e) => {
            eprintln!(
                "shell: event subscription unavailable ({e}); commands still work"
            );
            None
        }
    };

    let mut stdout = tokio::io::stdout();
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    loop {
        if let Err(e) = write_prompt(&mut stdout).await {
            log::error!("shell: failed to write prompt: {e}");
            break;
        }

        // Race the user's next line against any pushed events. If an
        // event wins, we print it and loop back to redraw the prompt.
        let line = loop {
            tokio::select! {
                biased;
                ev = recv_event(&mut event_rx) => {
                    if let Some(line) = ev {
                        // Drop to a fresh line, print the event, redraw prompt.
                        // Any chars the user typed-but-not-Entered are still
                        // in the kernel's line buffer and will be delivered
                        // on Enter — they just visually move to the new line.
                        print_async_event(&mut stdout, &line).await;
                        if let Err(e) = write_prompt(&mut stdout).await {
                            log::error!("shell: failed to redraw prompt: {e}");
                            return Ok(());
                        }
                    }
                }
                next = reader.next_line() => {
                    match next {
                        Ok(Some(l)) => break l,
                        Ok(None) => {
                            // EOF (Ctrl-D)
                            println!();
                            return Ok(());
                        }
                        Err(e) => {
                            log::error!("shell: failed to read line: {e}");
                            return Ok(());
                        }
                    }
                }
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "quit" || trimmed == "exit" {
            break;
        }
        if trimmed == "help" {
            print_help();
            continue;
        }

        // Parse the line through the same clap grammar as single-shot mode.
        // Prepend the binary name so clap's parser is happy.
        let argv = std::iter::once("qauld-ctl")
            .chain(trimmed.split_whitespace())
            .collect::<Vec<_>>();
        let parsed = match Cli::try_parse_from(&argv) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        if matches!(parsed.command, Commands::Shell(_)) {
            eprintln!("already in shell; use `quit` or Ctrl-D to exit");
            continue;
        }
        if matches!(parsed.command, Commands::Subscribe(_)) {
            eprintln!(
                "events are already streaming in this shell session; \
                 you don't need a separate `subscribe` here"
            );
            continue;
        }

        let merged = Cli {
            socket: shell_cli.socket.clone(),
            dir: shell_cli.dir.clone(),
            json: parsed.json,
            verbose: shell_cli.verbose,
            timeout: shell_cli.timeout,
            command: parsed.command,
        };

        let mut transport = match crate::transport::SocketTransport::connect(&shell_cli).await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("connection failed: {e}");
                continue;
            }
        };
        if let Err(e) = crate::run(&mut transport, merged).await {
            eprintln!("error: {e}");
        }
    }

    println!("bye");
    Ok(())
}

async fn write_prompt(
    stdout: &mut tokio::io::Stdout,
) -> Result<(), Box<dyn std::error::Error>> {
    stdout.write_all(PROMPT).await?;
    stdout.flush().await?;
    Ok(())
}

/// Receive the next event from the listener. Returns `None` if the
/// listener was never started (no daemon at shell launch) or has gone
/// away. We never return early on `None` from a live channel; instead
/// we drop the receiver so future iterations skip the event branch.
async fn recv_event(rx: &mut Option<tokio::sync::mpsc::UnboundedReceiver<String>>) -> Option<String> {
    match rx {
        Some(r) => match r.recv().await {
            Some(line) => Some(line),
            None => {
                // Channel closed; drop the receiver so we don't poll it again.
                *rx = None;
                None
            }
        },
        None => {
            // Park forever so `tokio::select!` always picks the stdin branch.
            std::future::pending::<Option<String>>().await
        }
    }
}

async fn print_async_event(stdout: &mut tokio::io::Stdout, line: &str) {
    // Carriage return + clear-rest-of-line (ANSI), then the event, then a
    // newline. The clear-rest-of-line keeps the prompt from leaking onto
    // the event line in terminals that support it; in terminals that
    // don't, the worst case is a stale "qauld> " prefix on the event row.
    let payload = format!("\r\x1b[K{line}\n");
    let _ = stdout.write_all(payload.as_bytes()).await;
    let _ = stdout.flush().await;
}

fn print_banner() {
    println!("qauld-ctl interactive shell");
    println!("Type a qauld-ctl command (without the leading binary name).");
    println!("Examples:  node info  |  users list  |  help  |  quit");
    println!("Live events from qauld will appear in this window as they happen.");
    println!();
}

fn print_help() {
    println!("Available commands are the same as qauld-ctl in single-shot mode.");
    println!("Run `qauld-ctl --help` outside the shell for the full grammar, or");
    println!("type any subcommand with `--help` here to see its options, e.g.");
    println!("  users --help");
    println!("Quoted arguments with spaces are not yet supported in shell mode.");
}
