// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! In-memory state for the TUI.

use std::collections::VecDeque;

const MAX_EVENTS: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Users,
    Feed,
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
            Tab::Feed => Tab::Users,
        };
        self.cursor = 0;
    }

    pub fn prev_tab(&mut self) {
        self.next_tab(); // only two tabs, same as next
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
        }
    }

    pub fn push_event(&mut self, line: String) {
        if self.events.len() >= MAX_EVENTS {
            self.events.pop_front();
        }
        self.events.push_back(line);
    }
}
