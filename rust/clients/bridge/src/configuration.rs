// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Matrix Configuration object for running binary
//!
//! This file contains the data structure to configure the bot which
//! connects qaul with the matrix.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MatrixConfiguration {
    pub relay_bot: RelayBot,
    pub feed: Feed,
}

impl Default for MatrixConfiguration {
    fn default() -> Self {
        MatrixConfiguration {
            relay_bot: RelayBot::default(),
            feed: Feed::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RelayBot {
    pub homeserver: String,
    pub bot_id: String,
    pub bot_password: String,
}

impl Default for RelayBot {
    fn default() -> Self {
        RelayBot {
            homeserver: String::new(),
            bot_id: String::new(),
            bot_password: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Feed {
    pub last_index: u64,
}

impl Default for Feed {
    fn default() -> Self {
        Feed { last_index: 0 }
    }
}
