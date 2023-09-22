// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Matrix Configuration object for running binary
//!
//! This file contains the data structure to configure the bot which
//! connects qaul with the matrix.

use matrix_sdk::ruma::RoomId;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use uuid::Uuid;

/// make storage path accessible
static CONFIG_PATH: state::Storage<String> = state::Storage::new();

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MatrixConfiguration {
    pub relay_bot: RelayBot,
    pub feed: Feed,
    pub room_map: HashMap<Uuid, MatrixRoom>,
}

impl Default for MatrixConfiguration {
    fn default() -> Self {
        MatrixConfiguration {
            relay_bot: RelayBot::default(),
            feed: Feed::default(),
            room_map: HashMap::new(),
        }
    }
}

impl MatrixConfiguration {
    /// initialize the matrix configuration
    pub fn init(config_path: String) {
        // put path to state
        CONFIG_PATH.set(config_path);
    }

    /// save the matrix configuration
    pub fn save(config: MatrixConfiguration) {
        let path_string = Self::get_path();
        let path = Path::new(path_string.as_str());
        let config_path = path.join("matrix.yaml");
        let yaml = serde_yaml::to_string(&config).expect("Could not encode into YAML values");
        fs::write(config_path, yaml).expect("Could not write config");
    }

    /// get configuration storage path
    pub fn get_path() -> String {
        CONFIG_PATH.get().clone()
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
    pub feed_room: RoomId,
}

impl Default for Feed {
    fn default() -> Self {
        Feed {
            last_index: 0,
            feed_room: RoomId::try_from("!nGnOGFPgRafNcUAJJA:matrix.org")
                .expect("Please add a valid room ID"),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MatrixRoom {
    pub matrix_room_id: RoomId,
    pub qaul_group_name: String,
    pub last_index: u64,
}

impl Default for MatrixRoom {
    fn default() -> Self {
        MatrixRoom {
            matrix_room_id: RoomId::try_from("").expect("Please add a valid room ID"),
            qaul_group_name: String::new(),
            last_index: 0,
        }
    }
}

impl MatrixRoom {
    pub fn update_last_index(&mut self, index: u64) {
        self.last_index = index;
    }
}
