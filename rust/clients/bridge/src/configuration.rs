// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Matrix Configuration object for running binary
//!
//! This file contains the data structure to configure the bot which
//! connects qaul with the matrix.

use crate::libqaul::storage::Storage;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, sync::RwLockWriteGuard};

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

impl MatrixConfiguration {
    pub fn save(config: MatrixConfiguration) {
        println!("{:#?}", config);
        let path_string = Storage::get_path();
        let path = Path::new(path_string.as_str());
        let config_path = path.join("matrix.yaml");
        let yaml = serde_yaml::to_string(&config).expect("Could not encode into YAML values");
        fs::write(config_path, yaml).expect("Could not write config");
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
