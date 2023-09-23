// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Matrix Configuration object for running binary
//!
//! This file contains the data structure to configure the bot which
//! connects qaul with the matrix.

use matrix_sdk::ruma::RoomId;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, sync::RwLock};
use uuid::Uuid;

/// Matrix Configuration
static MATRIX_CONFIG: state::Storage<RwLock<MatrixConfiguration>> = state::Storage::new();

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
    /// initialize and save the matrix configuration
    pub fn init(config_path: String, config: MatrixConfiguration) {
        // set path
        CONFIG_PATH.set(config_path);

        // set configuration
        MATRIX_CONFIG.set(RwLock::new(config.clone()));

        // save the configuration to file
        Self::save(config);
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

    /// get matrix server credentials
    pub fn get_matrix_server_credentials() -> RelayBot {
        let config = MATRIX_CONFIG.get().read().unwrap();
        config.relay_bot.clone()
    }

    /// get last index
    pub fn get_last_index() -> u64 {
        let config = MATRIX_CONFIG.get().read().unwrap();
        config.feed.last_index
    }

    /// get qaul groups
    pub fn get_qaul_groups() -> Vec<Uuid> {
        let config = MATRIX_CONFIG.get().read().unwrap();
        let qaul_groups: Vec<Uuid> = config.room_map.keys().cloned().collect();
        qaul_groups
    }

    /// get feed room
    pub fn get_feed_room() -> RoomId {
        let config = MATRIX_CONFIG.get().read().unwrap();
        config.feed.feed_room.clone()
    }

    /// get feed last index
    pub fn get_feed_last_index() -> u64 {
        let config = MATRIX_CONFIG.get().read().unwrap();
        config.feed.last_index
    }

    /// set feed last index
    pub fn set_feed_last_index(last_index: u64) {
        let mut config = MATRIX_CONFIG.get().write().unwrap();
        config.feed.last_index = last_index;
        Self::save(config.clone());
    }

    /// get matrix room
    pub fn get_matrix_room_last_index(group: &Uuid) -> Option<u64> {
        let config = MATRIX_CONFIG.get().read().unwrap();
        match config.room_map.get(group) {
            Some(matrix_room) => Some(matrix_room.last_index),
            None => None,
        }
    }

    /// get matrix room for qaul group UUID
    pub fn get_related_matrix_room(group_id: Uuid) -> Option<MatrixRoom> {
        let config = MATRIX_CONFIG.get().read().unwrap();
        match config.room_map.get(&group_id) {
            None => None,
            Some(matrix_room) => Some(matrix_room.clone()),
        }
    }

    /// set matrix group index
    pub fn set_matrix_room_last_index(group_id: Uuid, last_index: u64) {
        let mut config = MATRIX_CONFIG.get().write().unwrap();
        if let Some(matrix_room) = config.room_map.get_mut(&group_id) {
            matrix_room.last_index = last_index;
            Self::save(config.clone());
        }
    }

    /// get qaul group UUID for matrix room id
    pub fn get_qaul_group_uuid(room_id: RoomId) -> Option<Uuid> {
        let config = MATRIX_CONFIG.get().read().unwrap();

        config
            .room_map
            .iter()
            .find_map(|(key, &ref val)| {
                if val.matrix_room_id == room_id {
                    Some(key)
                } else {
                    None
                }
            })
            .copied()
    }

    /// get qaul group UUID & matrix room for matrix room id
    pub fn get_matrix_room_by_id(room_id: RoomId) -> Option<(Uuid, MatrixRoom)> {
        let config = MATRIX_CONFIG.get().read().unwrap();

        config.room_map.iter().find_map(|(key, &ref val)| {
            if val.matrix_room_id == room_id {
                Some((key.clone(), val.clone()))
            } else {
                None
            }
        })
    }

    /// insert new matrix room for qaul group Uuid
    pub fn create_qaul_matrix_room(group_id: Uuid, matrix_room: MatrixRoom) {
        let mut config = MATRIX_CONFIG.get().write().unwrap();
        config.room_map.insert(group_id, matrix_room);

        Self::save(config.clone());
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

/*
impl MatrixRoom {
    pub fn update_last_index(&mut self, index: u64) {
        self.last_index = index;
    }
}
 */
