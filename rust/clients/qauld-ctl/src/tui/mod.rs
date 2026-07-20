// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Terminal UI for the qauld daemon, surfaced as the `qauld-ctl tui`
//! subcommand. The implementation lives under this module so a
//! scripts-only build can drop the `tui` cargo feature and avoid the
//! ratatui / crossterm deps entirely.

use std::io;
use std::time::{Duration, Instant};

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

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    // Paint the frame skeleton immediately: the priming refresh below
    // performs network round-trips, and until it returns the user
    // would otherwise stare at a blank alternate screen.
    terminal.draw(|f| ui::draw(f, &app))?;
    // Prime data on launch
    refresh(&mut app, &connect, timeout).await;

    let mut term_events = EventStream::new();
    let mut next_refresh = Instant::now() + Duration::from_secs(refresh_secs);
    let result = loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let tick = tokio::time::sleep_until(tokio::time::Instant::from_std(next_refresh));
        tokio::select! {
            biased;
            ev = term_events.next() => match ev {
                Some(Ok(CtEvent::Key(k))) if k.kind == KeyEventKind::Press => {
                    if let Some(exit) = handle_key(&mut app, k, &connect, timeout).await {
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
            _ = tick => {
                refresh(&mut app, &connect, timeout).await;
                next_refresh = Instant::now() + Duration::from_secs(refresh_secs);
            }
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
                    refresh(app, connect, timeout).await;
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
        KeyCode::Char('r') => refresh(app, connect, timeout).await,
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

async fn refresh(
    app: &mut App,
    connect: &qauld_rpc::transport::ConnectInfo,
    timeout: Duration,
) {
    match data::fetch_default_user(connect, timeout).await {
        Ok(d) => {
            app.node_name = d.label;
            app.default_user_id = d.id_bytes;
        }
        Err(e) => app.push_event(format!("default user fetch failed: {e}")),
    }
    match data::fetch_users(connect, timeout).await {
        Ok(users) => app.users = users,
        Err(e) => app.push_event(format!("users fetch failed: {e}")),
    }
    match data::fetch_feed(connect, timeout).await {
        Ok(feed) => app.feed = feed,
        Err(e) => app.push_event(format!("feed fetch failed: {e}")),
    }
    match data::fetch_dtn_state(connect, timeout, &app.default_user_id).await {
        Ok(state) => {
            app.record_unconfirmed(state.unconfirmed_count);
            app.dtn_state = Some(state);
        }
        Err(e) => app.push_event(format!("dtn state fetch failed: {e}")),
    }
    match data::fetch_dtn_config(connect, timeout, &app.default_user_id).await {
        Ok(cfg) => app.dtn_config = Some(cfg),
        Err(e) => app.push_event(format!("dtn config fetch failed: {e}")),
    }
    match data::fetch_network(connect, timeout).await {
        Ok(snapshot) => {
            app.record_network(&snapshot);
            app.network = Some(snapshot);
        }
        Err(e) => app.push_event(format!("network fetch failed: {e}")),
    }
    match data::fetch_crypto_config(connect, timeout).await {
        Ok(cfg) => app.crypto_config = Some(cfg),
        Err(e) => app.push_event(format!("crypto config fetch failed: {e}")),
    }
    let since = app.crypto_event_floor_ms;
    match data::fetch_crypto_events(connect, timeout, since).await {
        Ok(events) => {
            // The `since_ms` filter is inclusive on the daemon side, so
            // drop anything we've already buffered.
            let fresh: Vec<_> = events
                .into_iter()
                .filter(|e| since == 0 || e.timestamp_ms > since)
                .collect();
            app.append_crypto_events(fresh);
        }
        Err(e) => app.push_event(format!("crypto events fetch failed: {e}")),
    }
}
