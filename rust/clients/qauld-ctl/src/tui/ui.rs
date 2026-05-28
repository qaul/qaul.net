// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Rendering for the TUI. All read-only against `App`; no I/O happens here.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs, Wrap},
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

    let titles: Vec<Line> = vec!["Users", "Feed"]
        .into_iter()
        .map(|t| Line::from(Span::raw(t)))
        .collect();
    let idx = match app.current_tab() {
        Tab::Users => 0,
        Tab::Feed => 1,
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
