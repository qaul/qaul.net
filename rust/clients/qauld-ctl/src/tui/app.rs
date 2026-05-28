// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! In-memory state for the TUI.

use std::collections::VecDeque;

use super::data::{
    CryptoConfig, CryptoRotationEvent, DtnConfig, DtnState, EventLine, NetworkSnapshot,
    ParsedEvent,
};

const MAX_EVENTS: usize = 200;
const MAX_DTN_EVENTS: usize = 100;
const MAX_NETWORK_EVENTS: usize = 100;
/// Width of the unconfirmed-count sparkline (number of samples kept).
pub const UNCONFIRMED_HISTORY: usize = 60;
/// Width of the per-module-total sparklines on the Network tab.
pub const NETWORK_HISTORY: usize = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Users,
    Feed,
    Dtn,
    Network,
    Crypto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    /// Typing a feed message into the compose modal.
    Composing,
    /// Typing into the per-tab filter input.
    Filtering,
    /// A modal is open showing the selected row's untruncated detail.
    Viewing,
}

#[derive(Debug, Clone)]
pub struct UserRow {
    pub name: String,
    pub id: String,
    pub connectivity: String,
    pub bio: String,
    pub profile_version: u32,
}

#[derive(Debug, Clone)]
pub struct FeedRow {
    pub index: u64,
    pub sender: String,
    pub content: String,
    pub time_sent: String,
}

pub struct App {
    pub node_name: String,
    /// Raw PeerId bytes of the daemon's default user account. Cached
    /// from each `fetch_default_user` call so mutating commands
    /// (e.g. feed send) can populate the `user_id` field on their
    /// RPC envelopes — libqaul rejects requests with an empty
    /// user_id ("InvalidMultihash(UnexpectedEof)").
    pub default_user_id: Vec<u8>,
    pub users: Vec<UserRow>,
    pub feed: Vec<FeedRow>,
    pub events: VecDeque<String>,
    pub dtn_state: Option<DtnState>,
    pub dtn_config: Option<DtnConfig>,
    pub dtn_events: VecDeque<String>,
    /// Rolling samples of `unconfirmed_count` taken once per refresh,
    /// oldest → newest. Capped at [`UNCONFIRMED_HISTORY`].
    pub dtn_unconfirmed_history: VecDeque<u64>,
    pub network: Option<NetworkSnapshot>,
    pub network_events: VecDeque<String>,
    /// Rolling per-module connection counts (LAN, Internet, BLE),
    /// taken once per refresh. Each `Vec` is capped at
    /// [`NETWORK_HISTORY`].
    pub network_history: NetworkHistory,
    pub crypto_config: Option<CryptoConfig>,
    pub crypto_events: Vec<CryptoRotationEvent>,
    /// Wall-clock ms of the newest event seen, used as the floor for
    /// the next `GetRotationEventsRequest` so we don't refetch the
    /// whole log on each tick.
    pub crypto_event_floor_ms: u64,
    tab: Tab,
    pub cursor: usize,
    pub input_mode: InputMode,
    pub compose_buffer: String,
    /// Per-session filter text. Applies to the active tab and is
    /// cleared on tab switch so each tab starts fresh.
    pub filter: String,
}

#[derive(Default)]
pub struct NetworkHistory {
    pub lan: VecDeque<u64>,
    pub internet: VecDeque<u64>,
    pub ble: VecDeque<u64>,
}

impl App {
    pub fn new() -> Self {
        Self {
            node_name: String::from("(loading…)"),
            default_user_id: Vec::new(),
            users: Vec::new(),
            feed: Vec::new(),
            events: VecDeque::new(),
            dtn_state: None,
            dtn_config: None,
            dtn_events: VecDeque::new(),
            dtn_unconfirmed_history: VecDeque::new(),
            network: None,
            network_events: VecDeque::new(),
            network_history: NetworkHistory::default(),
            crypto_config: None,
            crypto_events: Vec::new(),
            crypto_event_floor_ms: 0,
            tab: Tab::Users,
            cursor: 0,
            input_mode: InputMode::Normal,
            compose_buffer: String::new(),
            filter: String::new(),
        }
    }

    pub fn current_tab(&self) -> Tab {
        self.tab
    }

    pub fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Users => Tab::Feed,
            Tab::Feed => Tab::Dtn,
            Tab::Dtn => Tab::Network,
            Tab::Network => Tab::Crypto,
            Tab::Crypto => Tab::Users,
        };
        self.cursor = 0;
        self.filter.clear();
    }

    pub fn prev_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Users => Tab::Crypto,
            Tab::Feed => Tab::Users,
            Tab::Dtn => Tab::Feed,
            Tab::Network => Tab::Dtn,
            Tab::Crypto => Tab::Network,
        };
        self.cursor = 0;
        self.filter.clear();
    }

    pub fn cursor_down(&mut self) {
        let len = self.filtered_len();
        if len == 0 {
            self.cursor = 0;
        } else if self.cursor + 1 < len {
            self.cursor += 1;
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Number of rows currently visible on the active tab (after
    /// filter). Used by the cursor + the title bar of each table.
    pub fn filtered_len(&self) -> usize {
        match self.tab {
            Tab::Users => self.filtered_users().count(),
            Tab::Feed => self.filtered_feed().count(),
            Tab::Dtn => self.filtered_dtn_custodians().count(),
            Tab::Network => self.filtered_peers().count(),
            Tab::Crypto => self.filtered_crypto_events().count(),
        }
    }

    /// Substring match against the active filter; empty filter
    /// passes everything. Lower-cased on both sides.
    fn matches_filter(&self, hay: &str) -> bool {
        if self.filter.is_empty() {
            return true;
        }
        let needle = self.filter.to_ascii_lowercase();
        hay.to_ascii_lowercase().contains(&needle)
    }

    pub fn filtered_users(&self) -> impl Iterator<Item = &UserRow> {
        self.users.iter().filter(move |u| {
            self.matches_filter(&format!(
                "{} {} {} {}",
                u.name, u.id, u.connectivity, u.bio
            ))
        })
    }

    pub fn filtered_feed(&self) -> impl Iterator<Item = &FeedRow> {
        self.feed.iter().filter(move |m| {
            self.matches_filter(&format!(
                "{} {} {} {}",
                m.index, m.sender, m.content, m.time_sent
            ))
        })
    }

    pub fn filtered_peers(&self) -> impl Iterator<Item = &super::data::PeerRow> {
        self.network.iter().flat_map(move |n| {
            n.peers.iter().filter(move |p| {
                self.matches_filter(&format!("{} {} h{} rtt{}", p.module, p.user_id, p.hops, p.rtt_ms))
            })
        })
    }

    pub fn filtered_dtn_custodians(&self) -> impl Iterator<Item = &String> {
        self.dtn_config
            .iter()
            .flat_map(move |cfg| cfg.users.iter().filter(move |u| self.matches_filter(u)))
    }

    pub fn filtered_crypto_events(&self) -> impl Iterator<Item = &CryptoRotationEvent> {
        self.crypto_events.iter().filter(move |e| {
            self.matches_filter(&format!(
                "{} {} {} {} {}",
                e.timestamp_ms, e.kind, e.remote_id, e.primary_session_id, e.draining_session_id
            ))
        })
    }

    /// Returns labelled (key, value) pairs for the currently selected
    /// row on the active tab. `None` when nothing is selected
    /// (e.g. empty table after filter). Used by the detail modal.
    pub fn selected_detail(&self) -> Option<(String, Vec<(String, String)>)> {
        match self.tab {
            Tab::Users => {
                let u = self.filtered_users().nth(self.cursor)?;
                Some((
                    format!("User: {}", u.name),
                    vec![
                        ("name".into(), u.name.clone()),
                        ("id".into(), u.id.clone()),
                        ("connectivity".into(), u.connectivity.clone()),
                        ("profile version".into(), u.profile_version.to_string()),
                        ("bio".into(), u.bio.clone()),
                    ],
                ))
            }
            Tab::Feed => {
                let m = self.filtered_feed().nth(self.cursor)?;
                Some((
                    format!("Feed message #{}", m.index),
                    vec![
                        ("index".into(), m.index.to_string()),
                        ("sender".into(), m.sender.clone()),
                        ("time sent".into(), m.time_sent.clone()),
                        ("content".into(), m.content.clone()),
                    ],
                ))
            }
            Tab::Dtn => {
                let id = self.filtered_dtn_custodians().nth(self.cursor)?;
                Some((
                    "Custodian".to_string(),
                    vec![("peer id".into(), id.clone())],
                ))
            }
            Tab::Network => {
                let p = self.filtered_peers().nth(self.cursor)?;
                Some((
                    format!("Peer via {}", p.module),
                    vec![
                        ("module".into(), p.module.to_string()),
                        ("peer id".into(), p.user_id.clone()),
                        ("hops".into(), p.hops.to_string()),
                        ("rtt".into(), format!("{} ms", p.rtt_ms)),
                    ],
                ))
            }
            Tab::Crypto => {
                // Filtered events are reversed (newest-first) for
                // display; selected_detail must match the rendered
                // order so the cursor lines up with what the user
                // sees.
                let events: Vec<_> = self.filtered_crypto_events().collect();
                let e = events.iter().rev().nth(self.cursor)?;
                Some((
                    format!("Rotation event: {}", e.kind),
                    vec![
                        ("timestamp (ms)".into(), e.timestamp_ms.to_string()),
                        ("kind".into(), e.kind.to_string()),
                        ("remote peer".into(), e.remote_id.clone()),
                        ("primary session".into(), e.primary_session_id.to_string()),
                        ("draining session".into(), e.draining_session_id.to_string()),
                    ],
                ))
            }
        }
    }

    /// Append new rotation events and advance the floor so subsequent
    /// fetches only ask for what's newer.
    pub fn append_crypto_events(&mut self, mut new_events: Vec<CryptoRotationEvent>) {
        if new_events.is_empty() {
            return;
        }
        for e in &new_events {
            if e.timestamp_ms > self.crypto_event_floor_ms {
                self.crypto_event_floor_ms = e.timestamp_ms;
            }
        }
        self.crypto_events.append(&mut new_events);
        // Cap the buffer so it doesn't grow unbounded; keep newest.
        const MAX: usize = 500;
        if self.crypto_events.len() > MAX {
            let drop = self.crypto_events.len() - MAX;
            self.crypto_events.drain(..drop);
        }
    }

    /// Route a structured event from the subscribe stream. DTN
    /// delivery responses go to the DTN tab, peer events go to the
    /// Network tab, crypto rotations go to the Crypto tab's typed
    /// buffer (deduped against poll-based fetches), and anything
    /// else lands in the general events panel.
    pub fn push_event_line(&mut self, line: EventLine) {
        // Merge typed payloads first so the typed buffers stay in
        // sync regardless of which buffer the text line lands in.
        if let ParsedEvent::CryptoRotation(ref ev) = line.parsed {
            self.merge_crypto_event(ev.clone());
        }
        match line.topic.as_str() {
            "dtn.delivery_response" => {
                if self.dtn_events.len() >= MAX_DTN_EVENTS {
                    self.dtn_events.pop_front();
                }
                self.dtn_events.push_back(line.text);
            }
            "peers.connected" | "peers.disconnected" => {
                if self.network_events.len() >= MAX_NETWORK_EVENTS {
                    self.network_events.pop_front();
                }
                self.network_events.push_back(line.text);
            }
            "crypto.rotation" => {
                // Already merged into crypto_events above; don't
                // also clutter the general events panel.
            }
            _ => self.push_event(line.text),
        }
    }

    /// Merge a rotation event from the push channel, deduping
    /// against anything the poll path already buffered. Same
    /// (timestamp_ms, kind, primary, draining) tuple ⇒ same event.
    fn merge_crypto_event(&mut self, ev: CryptoRotationEvent) {
        let duplicate = self.crypto_events.iter().any(|existing| {
            existing.timestamp_ms == ev.timestamp_ms
                && existing.kind == ev.kind
                && existing.primary_session_id == ev.primary_session_id
                && existing.draining_session_id == ev.draining_session_id
        });
        if duplicate {
            return;
        }
        if ev.timestamp_ms > self.crypto_event_floor_ms {
            self.crypto_event_floor_ms = ev.timestamp_ms;
        }
        self.crypto_events.push(ev);
    }

    pub fn push_event(&mut self, line: String) {
        if self.events.len() >= MAX_EVENTS {
            self.events.pop_front();
        }
        self.events.push_back(line);
    }

    pub fn record_unconfirmed(&mut self, n: u32) {
        if self.dtn_unconfirmed_history.len() >= UNCONFIRMED_HISTORY {
            self.dtn_unconfirmed_history.pop_front();
        }
        self.dtn_unconfirmed_history.push_back(n as u64);
    }

    pub fn record_network(&mut self, snapshot: &NetworkSnapshot) {
        push_capped(&mut self.network_history.lan, snapshot.lan_peers as u64);
        push_capped(
            &mut self.network_history.internet,
            snapshot.internet_peers as u64,
        );
        push_capped(&mut self.network_history.ble, snapshot.ble_peers as u64);
    }
}

fn push_capped(buf: &mut VecDeque<u64>, value: u64) {
    if buf.len() >= NETWORK_HISTORY {
        buf.pop_front();
    }
    buf.push_back(value);
}
