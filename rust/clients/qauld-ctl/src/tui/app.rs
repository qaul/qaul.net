// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! In-memory state for the TUI.

use std::collections::VecDeque;

use crate::data::{DtnConfig, DtnState, EventLine};

const MAX_EVENTS: usize = 200;
const MAX_DTN_EVENTS: usize = 100;
/// Width of the unconfirmed-count sparkline (number of samples kept).
pub const UNCONFIRMED_HISTORY: usize = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Users,
    Feed,
    Dtn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Composing,
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
    tab: Tab,
    pub cursor: usize,
    pub input_mode: InputMode,
    pub compose_buffer: String,
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
            tab: Tab::Users,
            cursor: 0,
            input_mode: InputMode::Normal,
            compose_buffer: String::new(),
        }
    }

    pub fn current_tab(&self) -> Tab {
        self.tab
    }

    pub fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Users => Tab::Feed,
            Tab::Feed => Tab::Dtn,
            Tab::Dtn => Tab::Users,
        };
        self.cursor = 0;
    }

    pub fn prev_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Users => Tab::Dtn,
            Tab::Feed => Tab::Users,
            Tab::Dtn => Tab::Feed,
        };
        self.cursor = 0;
    }

    pub fn cursor_down(&mut self) {
        let len = self.list_len();
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

    fn list_len(&self) -> usize {
        match self.tab {
            Tab::Users => self.users.len(),
            Tab::Feed => self.feed.len(),
            Tab::Dtn => self.dtn_config.as_ref().map(|c| c.users.len()).unwrap_or(0),
        }
    }

    /// Route a structured event from the subscribe stream. DTN
    /// delivery responses get their own deque (rendered on the DTN
    /// tab); everything else lands in the general events panel.
    pub fn push_event_line(&mut self, line: EventLine) {
        if line.topic == "dtn.delivery_response" {
            if self.dtn_events.len() >= MAX_DTN_EVENTS {
                self.dtn_events.pop_front();
            }
            self.dtn_events.push_back(line.text);
        } else {
            self.push_event(line.text);
        }
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
}
