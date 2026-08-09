// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Interactive shell mode for qauld-ctl.
//!
//! Reads command lines through `rustyline` (so arrow up/down recall history
//! and arrow left/right move the cursor for editing), tokenises them with
//! shell-style quoting (`shlex`, so `chat send -m "hello my friend"` works),
//! parses each line through the same `clap` grammar used by single-shot mode,
//! and dispatches it via the existing `run` function. A fresh socket is opened
//! per command — the same code path single-shot uses, so behaviour is
//! identical aside from the prompt and the persistent process.
//!
//! A long-running event subscription runs in the background and prints
//! incoming events via rustyline's `ExternalPrinter`, which redraws the line
//! being edited so async events never corrupt the user's input.

use clap::{CommandFactory, Parser};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, ExternalPrinter};

use crate::cli::{Cli, Commands};

const PROMPT: &str = "qauld> ";

/// Run the interactive shell loop until the user types `quit` / `exit`
/// or sends EOF (Ctrl-D).
///
/// `shell_cli` carries the connection flags (`--socket`, `--dir`, `--json`)
/// the user supplied when launching qauld-ctl. Per-line re-parsed `--socket`
/// / `--dir` are ignored so the connection target is predictable; per-line
/// `--json` is honoured per command.
pub async fn run(shell_cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Line editor: history (↑/↓), cursor movement + editing (←/↑ etc.).
    let mut rl = DefaultEditor::new()?;
    let history = history_path();
    if let Some(path) = history.as_ref() {
        // A missing history file on first run is fine.
        let _ = rl.load_history(path);
    }

    // Background event subscription. Events print via rustyline's external
    // printer so they don't corrupt the line being edited. If the daemon
    // isn't up yet we warn but keep the shell usable.
    match crate::subscribe::spawn_event_listener(&shell_cli).await {
        Ok(mut event_rx) => match rl.create_external_printer() {
            Ok(mut printer) => {
                tokio::spawn(async move {
                    while let Some(line) = event_rx.recv().await {
                        if printer.print(format!("{line}\n")).is_err() {
                            break;
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("shell: live events unavailable ({e}); commands still work");
            }
        },
        Err(e) => {
            eprintln!("shell: event subscription unavailable ({e}); commands still work");
        }
    }

    // Show the help screen on startup.
    print_intro();
    print_help();

    loop {
        // `rustyline::readline` is blocking, so run it on the blocking pool;
        // move the editor in and back out so history persists across lines.
        let prompt = PROMPT.to_string();
        let (editor, readline) = tokio::task::spawn_blocking(move || {
            let res = rl.readline(&prompt);
            (rl, res)
        })
        .await?;
        rl = editor;

        let line = match readline {
            Ok(l) => l,
            // Ctrl-C: abandon the current line and show a fresh prompt.
            Err(ReadlineError::Interrupted) => continue,
            // Ctrl-D: exit the shell.
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("shell: input error: {e}");
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let _ = rl.add_history_entry(trimmed);

        if trimmed == "quit" || trimmed == "exit" {
            break;
        }
        if trimmed == "help" {
            print_help();
            continue;
        }

        // Tokenise with shell-style quoting so quoted values with spaces are a
        // single argument. `shlex::split` returns None on unbalanced quotes.
        let tokens = match shlex::split(trimmed) {
            Some(t) if !t.is_empty() => t,
            Some(_) => continue,
            None => {
                eprintln!("error: unbalanced quotes in command");
                continue;
            }
        };

        // Prepend the binary name so clap's parser is happy.
        let argv = std::iter::once("qauld-ctl".to_string())
            .chain(tokens)
            .collect::<Vec<_>>();
        let parsed = match Cli::try_parse_from(&argv) {
            Ok(p) => p,
            Err(e) => {
                // clap renders help/usage/errors here, including `<cmd> --help`.
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

        let mut transport =
            match qauld_rpc::SocketTransport::connect(&crate::connect_info(&shell_cli)).await {
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

    if let Some(path) = history.as_ref() {
        let _ = rl.save_history(path);
    }
    println!("bye");
    Ok(())
}

/// Where to persist shell history across sessions. `None` (no `$HOME`) just
/// means in-session-only history — arrow-key recall still works.
fn history_path() -> Option<std::path::PathBuf> {
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    Some(std::path::PathBuf::from(home).join(".qauld-ctl_history"))
}

fn print_intro() {
    println!("qauld-ctl interactive shell — type a command (no leading binary name).");
    println!("  history: ↑/↓   edit line: ←/→   help: help   exit: quit / Ctrl-D");
    println!("Live events from qauld appear inline as they happen.");
    println!();
}

/// Render the real clap help (command list + options) so `help` and the
/// startup screen show the actual grammar rather than a vague pointer.
fn print_help() {
    let mut cmd = Cli::command();
    println!("{}", cmd.render_help());
}
