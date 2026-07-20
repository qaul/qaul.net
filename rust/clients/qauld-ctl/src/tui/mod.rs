// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Terminal UI for the qauld daemon, surfaced as the `qauld-ctl tui`
//! subcommand. The implementation lives under this module so a
//! scripts-only build can drop the `tui` cargo feature and avoid the
//! ratatui / crossterm deps entirely.

use std::io;
use std::time::Duration;

use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event as CtEvent, EventStream, KeyCode,
        KeyEvent, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::cli::Cli;

mod app;
mod data;
mod ui;

use app::{App, InputMode, Tab};

/// Entry point invoked from `main` when the user runs `qauld-ctl tui`.
///
/// `refresh_secs` is the auto-refresh interval; the rest of the
/// connection info (socket / dir / timeout) is read from the parent
/// `Cli` so the TUI honours the same connection flags as every other
/// subcommand.
pub async fn run(cli: Cli, refresh_secs: u64) -> Result<(), Box<dyn std::error::Error>> {
    let connect = crate::connect_info(&cli);
    let timeout = Duration::from_secs(cli.timeout);

    // Verify the daemon is reachable before tearing the screen so the
    // failure mode is a one-line message, not a partially-initialised
    // alt-screen.
    if let Err(e) = qauld_rpc::SocketTransport::connect(&connect).await {
        eprintln!("qauld-ctl tui: cannot reach qauld: {e}");
        eprintln!("        hint: start it with `qauld` or `qauld-ctl run`.");
        std::process::exit(1);
    }

    // Subscribe stream (bg task pushes events into a channel).
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel::<data::EventLine>();
    {
        let connect = connect.clone();
        tokio::spawn(async move {
            if let Err(e) = data::spawn_subscribe(connect, event_tx.clone()).await {
                let _ = event_tx.send(data::EventLine {
                    topic: "tui.internal".into(),
                    text: format!("subscribe stream ended: {e}"),
                    parsed: data::ParsedEvent::None,
                });
            }
        });
    }

    // Background refresh: a task fans out every polled fetch
    // concurrently (never on the render loop) and posts a `Snapshot`
    // into this channel. A slow or hung RPC can no longer stall input
    // or redraws — the loop simply keeps painting the last snapshot.
    let (snapshot_tx, mut snapshot_rx) = tokio::sync::mpsc::unbounded_channel::<data::Snapshot>();
    // Manual-refresh trigger (`r` key, and after sending a feed post).
    let (refresh_tx, mut refresh_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    {
        let connect = connect.clone();
        let refresh_secs = refresh_secs.max(1);
        tokio::spawn(async move {
            // The poll floor for crypto events lives with the task (the
            // sole owner of the poll path); the UI dedups against the
            // push path, so overlaps are harmless.
            let mut crypto_since: u64 = 0;
            let mut interval = tokio::time::interval(Duration::from_secs(refresh_secs));
            // interval's first tick is immediate → prime without blocking
            loop {
                tokio::select! {
                    _ = interval.tick() => {}
                    _ = refresh_rx.recv() => {}
                }
                let snap = data::refresh_once(&connect, timeout, crypto_since).await;
                if let Ok(ref events) = snap.crypto_events {
                    for e in events {
                        crypto_since = crypto_since.max(e.timestamp_ms);
                    }
                }
                if snapshot_tx.send(snap).is_err() {
                    break; // UI gone
                }
            }
        });
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let mut term_events = EventStream::new();
    let result = loop {
        // The frame skeleton renders immediately (even before the first
        // snapshot lands), so startup never shows a blank alt-screen.
        terminal.draw(|f| ui::draw(f, &app))?;

        tokio::select! {
            biased;
            ev = term_events.next() => match ev {
                Some(Ok(CtEvent::Key(k))) if k.kind == KeyEventKind::Press => {
                    if let Some(exit) = handle_key(&mut app, k, &connect, timeout, &refresh_tx).await {
                        break exit;
                    }
                }
                Some(Ok(_)) => {} // ignore resize / mouse for now
                Some(Err(e)) => break Err(e.into()),
                None => break Ok(()),
            },
            line = event_rx.recv() => if let Some(line) = line {
                app.push_event_line(line);
            },
            snap = snapshot_rx.recv() => match snap {
                Some(snap) => apply_snapshot(&mut app, snap),
                None => {} // refresh task ended; keep the UI up
            },
        }
    };

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("qauld-ctl tui error: {e}");
        return Err(e);
    }
    Ok(())
}

async fn handle_key(
    app: &mut App,
    key: KeyEvent,
    connect: &qauld_rpc::transport::ConnectInfo,
    timeout: Duration,
    refresh_tx: &tokio::sync::mpsc::UnboundedSender<()>,
) -> Option<Result<(), Box<dyn std::error::Error>>> {
    // Modal text input wins.
    if app.input_mode == InputMode::Composing {
        match key.code {
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
                app.compose_buffer.clear();
            }
            KeyCode::Enter => {
                let body = std::mem::take(&mut app.compose_buffer);
                app.input_mode = InputMode::Normal;
                if !body.trim().is_empty() {
                    let user_id = app.default_user_id.clone();
                    match data::send_feed(connect, &body, &user_id, timeout).await {
                        Ok(()) => app.push_event(format!("feed sent: {body}")),
                        Err(e) => app.push_event(format!("feed send FAILED: {e}")),
                    }
                    // Ask the background task to pull the new post in;
                    // the snapshot arrives on the render loop's channel.
                    let _ = refresh_tx.send(());
                }
            }
            KeyCode::Backspace => {
                app.compose_buffer.pop();
            }
            KeyCode::Char(c) => {
                app.compose_buffer.push(c);
            }
            _ => {}
        }
        return None;
    }

    if app.input_mode == InputMode::Filtering {
        match key.code {
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
                app.filter.clear();
                app.cursor = 0;
            }
            KeyCode::Enter => {
                // Accept the filter; stay in Normal mode so the
                // user can navigate the filtered list.
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                app.filter.pop();
                app.cursor = 0;
            }
            KeyCode::Char(c) => {
                app.filter.push(c);
                app.cursor = 0;
            }
            _ => {}
        }
        return None;
    }

    if app.input_mode == InputMode::Viewing {
        // Any key dismisses the detail modal. Esc is the obvious
        // one; Enter feels natural too.
        if matches!(key.code, KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q')) {
            app.input_mode = InputMode::Normal;
        }
        return None;
    }

    match key.code {
        KeyCode::Char('q') => return Some(Ok(())),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            return Some(Ok(()));
        }
        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.prev_tab(),
        KeyCode::Char('r') => {
            let _ = refresh_tx.send(());
        }
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Filtering;
            app.filter.clear();
            app.cursor = 0;
        }
        KeyCode::Enter => {
            if app.selected_detail().is_some() {
                app.input_mode = InputMode::Viewing;
            }
        }
        KeyCode::Char('s') if app.current_tab() == Tab::Feed => {
            app.input_mode = InputMode::Composing;
            app.compose_buffer.clear();
        }
        KeyCode::Down => app.cursor_down(),
        KeyCode::Up => app.cursor_up(),
        _ => {}
    }
    None
}

/// Apply a `Snapshot` from the background refresh task to the app
/// state. Pure state mutation — no I/O, no awaits — so it never blocks
/// the render loop. Per-field errors are surfaced in the events pane,
/// mirroring the old inline refresh exactly.
fn apply_snapshot(app: &mut App, snap: data::Snapshot) {
    match snap.default_user {
        Ok(d) => {
            app.node_name = d.label;
            app.default_user_id = d.id_bytes;
        }
        Err(e) => app.push_event(format!("default user fetch failed: {e}")),
    }
    match snap.users {
        Ok(users) => app.users = users,
        Err(e) => app.push_event(format!("users fetch failed: {e}")),
    }
    match snap.feed {
        Ok(feed) => app.feed = feed,
        Err(e) => app.push_event(format!("feed fetch failed: {e}")),
    }
    match snap.dtn_state {
        Ok(state) => {
            app.record_unconfirmed(state.unconfirmed_count);
            app.dtn_state = Some(state);
        }
        Err(e) => app.push_event(format!("dtn state fetch failed: {e}")),
    }
    match snap.dtn_config {
        Ok(cfg) => app.dtn_config = Some(cfg),
        Err(e) => app.push_event(format!("dtn config fetch failed: {e}")),
    }
    match snap.network {
        Ok(snapshot) => {
            app.record_network(&snapshot);
            app.network = Some(snapshot);
        }
        Err(e) => app.push_event(format!("network fetch failed: {e}")),
    }
    match snap.crypto_config {
        Ok(cfg) => app.crypto_config = Some(cfg),
        Err(e) => app.push_event(format!("crypto config fetch failed: {e}")),
    }
    match snap.crypto_events {
        // append_crypto_events dedups against the push path, so events
        // the daemon's inclusive `since_ms` filter re-sends are dropped.
        Ok(events) => app.append_crypto_events(events),
        Err(e) => app.push_event(format!("crypto events fetch failed: {e}")),
    }
}
