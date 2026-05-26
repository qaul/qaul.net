// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Rendering for the TUI. All read-only against `App`; no I/O happens here.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Sparkline, Table, Tabs, Wrap},
    Frame,
};

use super::app::{App, InputMode, Tab};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // tabs + header
            Constraint::Min(8),      // main content
            Constraint::Length(8),   // events panel
            Constraint::Length(1),   // help line
        ])
        .split(area);

    draw_header(frame, chunks[0], app);
    match app.current_tab() {
        Tab::Users => draw_users(frame, chunks[1], app),
        Tab::Feed => draw_feed(frame, chunks[1], app),
        Tab::Dtn => draw_dtn(frame, chunks[1], app),
        Tab::Network => draw_network(frame, chunks[1], app),
    }
    draw_events(frame, chunks[2], app);
    draw_help(frame, chunks[3], app);

    if app.input_mode == InputMode::Composing {
        draw_compose_modal(frame, area, app);
    }
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App) {
    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(40)])
        .split(area);

    let titles: Vec<Line> = vec!["Users", "Feed", "DTN", "Network"]
        .into_iter()
        .map(|t| Line::from(Span::raw(t)))
        .collect();
    let idx = match app.current_tab() {
        Tab::Users => 0,
        Tab::Feed => 1,
        Tab::Dtn => 2,
        Tab::Network => 3,
    };
    let tabs = Tabs::new(titles)
        .select(idx)
        .block(Block::default().borders(Borders::ALL).title("qauld-tui"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan));
    frame.render_widget(tabs, split[0]);

    let header = Paragraph::new(Line::from(vec![
        Span::raw("node: "),
        Span::styled(&app.node_name, Style::default().fg(Color::Green)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, split[1]);
}

fn draw_users(frame: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app
        .users
        .iter()
        .enumerate()
        .map(|(i, u)| {
            let style = if i == app.cursor {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };
            Row::new(vec![
                Cell::from(u.name.clone()),
                Cell::from(short_id(&u.id)),
                Cell::from(u.connectivity.clone()),
                Cell::from(format!("v{}", u.profile_version)),
                Cell::from(u.bio.clone()),
            ])
            .style(style)
        })
        .collect();
    let widths = [
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Min(10),
    ];
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Name", "ID", "Connectivity", "Ver", "Bio"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Users ({})", app.users.len())),
        );
    frame.render_widget(table, area);
}

fn draw_feed(frame: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app
        .feed
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let style = if i == app.cursor {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };
            Row::new(vec![
                Cell::from(m.index.to_string()),
                Cell::from(m.time_sent.clone()),
                Cell::from(short_id(&m.sender)),
                Cell::from(m.content.clone()),
            ])
            .style(style)
        })
        .collect();
    let widths = [
        Constraint::Length(4),
        Constraint::Length(22),
        Constraint::Length(18),
        Constraint::Min(10),
    ];
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["#", "Time", "Sender", "Content"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Feed ({})", app.feed.len())),
        );
    frame.render_widget(table, area);
}

fn draw_dtn(frame: &mut Frame, area: Rect, app: &App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // stats + sparkline
            Constraint::Length(8),  // custodian users
            Constraint::Min(4),     // delivery events
        ])
        .split(area);

    // Stats row split into KPI cards on the left and sparkline on the right.
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(38), Constraint::Min(20)])
        .split(vertical[0]);
    draw_dtn_kpis(frame, top[0], app);
    draw_unconfirmed_sparkline(frame, top[1], app);

    // Configured custodian users
    draw_dtn_custodians(frame, vertical[1], app);

    // Delivery-response event log
    draw_dtn_delivery_events(frame, vertical[2], app);
}

fn draw_dtn_kpis(frame: &mut Frame, area: Rect, app: &App) {
    let lines: Vec<Line> = match &app.dtn_state {
        Some(s) => {
            let limit = app
                .dtn_config
                .as_ref()
                .map(|c| format!("/{} MB cap", c.total_size))
                .unwrap_or_default();
            vec![
                Line::from(vec![
                    Span::raw("used:        "),
                    Span::styled(
                        format!("{} MB{}", s.used_size, limit),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("messages:    "),
                    Span::styled(
                        s.message_count.to_string(),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("unconfirmed: "),
                    Span::styled(
                        s.unconfirmed_count.to_string(),
                        Style::default()
                            .fg(if s.unconfirmed_count > 0 {
                                Color::Yellow
                            } else {
                                Color::Green
                            })
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
            ]
        }
        None => vec![Line::from(Span::raw("(no DTN state yet)"))],
    };
    let p = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("DTN state"),
    );
    frame.render_widget(p, area);
}

fn draw_unconfirmed_sparkline(frame: &mut Frame, area: Rect, app: &App) {
    let samples: Vec<u64> = app.dtn_unconfirmed_history.iter().copied().collect();
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Unconfirmed (rolling)"),
        )
        .data(&samples)
        .style(Style::default().fg(Color::Magenta));
    frame.render_widget(sparkline, area);
}

fn draw_dtn_custodians(frame: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = match &app.dtn_config {
        Some(cfg) => cfg
            .users
            .iter()
            .enumerate()
            .map(|(i, u)| {
                let style = if i == app.cursor {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };
                Row::new(vec![Cell::from(short_id(u))]).style(style)
            })
            .collect(),
        None => Vec::new(),
    };
    let widths = [Constraint::Min(20)];
    let count = app.dtn_config.as_ref().map(|c| c.users.len()).unwrap_or(0);
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Configured custodian users"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Allowed custodians ({})", count)),
        );
    frame.render_widget(table, area);
}

fn draw_dtn_delivery_events(frame: &mut Frame, area: Rect, app: &App) {
    let lines: Vec<Line> = app
        .dtn_events
        .iter()
        .rev()
        .take(area.height.saturating_sub(2) as usize)
        .rev()
        .map(|e| Line::from(Span::raw(e.clone())))
        .collect();
    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "Delivery responses (live, {} buffered)",
                    app.dtn_events.len()
                )),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(p, area);
}

fn draw_network(frame: &mut Frame, area: Rect, app: &App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),  // per-module KPIs + sparklines
            Constraint::Min(6),     // peers table
            Constraint::Length(6),  // recent peer events
        ])
        .split(area);

    draw_network_kpis(frame, vertical[0], app);
    draw_network_peers(frame, vertical[1], app);
    draw_network_events(frame, vertical[2], app);
}

fn draw_network_kpis(frame: &mut Frame, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    let (lan, internet, ble, local) = match &app.network {
        Some(n) => (n.lan_peers, n.internet_peers, n.ble_peers, n.local_peers),
        None => (0, 0, 0, 0),
    };

    draw_module_card(
        frame,
        cols[0],
        "LAN",
        lan,
        local,
        &app.network_history.lan,
        Color::Cyan,
    );
    draw_module_card(
        frame,
        cols[1],
        "Internet",
        internet,
        0,
        &app.network_history.internet,
        Color::Green,
    );
    draw_module_card(
        frame,
        cols[2],
        "BLE",
        ble,
        0,
        &app.network_history.ble,
        Color::Magenta,
    );
}

fn draw_module_card(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    count: u32,
    local: u32,
    history: &std::collections::VecDeque<u64>,
    color: Color,
) {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(2)])
        .split(area);

    let mut headline = vec![Line::from(vec![
        Span::raw("peers: "),
        Span::styled(
            count.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
    ])];
    if label == "LAN" && local > 0 {
        headline.push(Line::from(vec![
            Span::raw("local: "),
            Span::styled(local.to_string(), Style::default().fg(Color::Gray)),
        ]));
    }
    let stats = Paragraph::new(headline).block(
        Block::default()
            .borders(Borders::ALL)
            .title(label.to_string()),
    );
    frame.render_widget(stats, inner[0]);

    let samples: Vec<u64> = history.iter().copied().collect();
    let sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title("trend"))
        .data(&samples)
        .style(Style::default().fg(color));
    frame.render_widget(sparkline, inner[1]);
}

fn draw_network_peers(frame: &mut Frame, area: Rect, app: &App) {
    let peers: Vec<_> = app
        .network
        .as_ref()
        .map(|n| n.peers.clone())
        .unwrap_or_default();
    let rows: Vec<Row> = peers
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let style = if i == app.cursor {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };
            Row::new(vec![
                Cell::from(p.module),
                Cell::from(short_id(&p.user_id)),
                Cell::from(p.hops.to_string()),
                Cell::from(format!("{} ms", p.rtt_ms)),
            ])
            .style(style)
        })
        .collect();
    let widths = [
        Constraint::Length(10),
        Constraint::Length(20),
        Constraint::Length(5),
        Constraint::Min(8),
    ];
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Module", "Peer", "Hops", "RTT"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Peers ({})", peers.len())),
        );
    frame.render_widget(table, area);
}

fn draw_network_events(frame: &mut Frame, area: Rect, app: &App) {
    let lines: Vec<Line> = app
        .network_events
        .iter()
        .rev()
        .take(area.height.saturating_sub(2) as usize)
        .rev()
        .map(|e| Line::from(Span::raw(e.clone())))
        .collect();
    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "Peer events (live, {} buffered)",
                    app.network_events.len()
                )),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(p, area);
}

fn draw_events(frame: &mut Frame, area: Rect, app: &App) {
    let lines: Vec<Line> = app
        .events
        .iter()
        .rev()
        .take(area.height.saturating_sub(2) as usize)
        .rev()
        .map(|e| Line::from(Span::raw(e.clone())))
        .collect();
    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Events (live)"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(p, area);
}

fn draw_help(frame: &mut Frame, area: Rect, app: &App) {
    let msg = match app.input_mode {
        InputMode::Composing => "Enter: send  Esc: cancel",
        InputMode::Normal => match app.current_tab() {
            Tab::Users => "[Tab] switch  [↑/↓] move  [r] refresh  [q] quit",
            Tab::Feed => "[Tab] switch  [↑/↓] move  [s] send  [r] refresh  [q] quit",
            Tab::Dtn => "[Tab] switch  [↑/↓] move  [r] refresh  [q] quit",
            Tab::Network => "[Tab] switch  [↑/↓] move  [r] refresh  [q] quit",
        },
    };
    frame.render_widget(
        Paragraph::new(msg).style(Style::default().fg(Color::DarkGray)),
        area,
    );
}

fn draw_compose_modal(frame: &mut Frame, area: Rect, app: &App) {
    let width = area.width.saturating_sub(20).min(80);
    let modal = Rect {
        x: (area.width.saturating_sub(width)) / 2,
        y: area.height / 2 - 2,
        width,
        height: 5,
    };
    frame.render_widget(Clear, modal);
    let p = Paragraph::new(app.compose_buffer.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Compose feed message")
                .style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(p, modal);
}

fn short_id(s: &str) -> String {
    if s.len() <= 16 {
        s.to_string()
    } else {
        format!("{}…{}", &s[..6], &s[s.len() - 4..])
    }
}
