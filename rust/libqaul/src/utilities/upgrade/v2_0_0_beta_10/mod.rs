// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Upgrade to new version 2.0.0-beta.10
//!
//! Breaking changes that need to be upgraded
//!
//! * InternetPeer structure added name field: config.internet.peers<InternetPeer>

use super::backup;
use std::path::Path;

mod old_config;

/// # Upgrade Logic
pub struct Mig200b4To200b5 {}
impl Mig200b4To200b5 {
    /// process migrating
    pub fn do_process(old_path: &str, new_path: &str, new_version: &str) -> bool {
        //cleanup dest
        backup::Backup::remove_folder(new_path);

        //create dest
        if let Err(_) = std::fs::create_dir(new_path) {
            println!("failed to creating destinaton folder");
            return false;
        }

        //move unchanged contents
        println!("\t\tmove contents..");
        if Self::move_contents(old_path, new_path) == false {
            return false;
        }

        //create new version file
        println!("\t\tcreate version file..");
        let path = Path::new(new_path).join("version");
        if let Err(_) = std::fs::write(path, new_version) {
            println!("failed to create version file!");
        }

        //update config.yaml
        println!("\t\tmigrating config.yaml..");
        Self::upgrade_config(old_path, new_path)
    }

    /// upgrade config structure
    fn upgrade_config(old_path: &str, new_path: &str) -> bool {
        //load old config
        if let Some(old_cfg) = old_config::Configuration::load(
            Path::new(old_path).join("config.yaml").to_str().unwrap(),
        ) {
            let node = crate::storage::configuration::Node {
                initialized: old_cfg.node.initialized,
                id: old_cfg.node.id.clone(),
                keys: old_cfg.node.keys.clone(),
            };
            let lan = crate::storage::configuration::Lan {
                active: old_cfg.lan.active,
                listen: old_cfg.lan.listen,
            };

            let mut peers: Vec<crate::storage::configuration::InternetPeer> = vec![];
            for peer in &old_cfg.internet.peers {
                peers.push(crate::storage::configuration::InternetPeer {
                    address: peer.address.clone(),
                    name: String::from("undefined"),
                    enabled: peer.enabled,
                });
            }
            let internet = crate::storage::configuration::Internet {
                active: old_cfg.internet.active,
                peers,
                do_listen: old_cfg.internet.do_listen,
                listen: old_cfg.internet.listen,
            };

            let mut user_accounts: Vec<crate::storage::configuration::UserAccount> = vec![];
            for user in &old_cfg.user_accounts {
                user_accounts.push(crate::storage::configuration::UserAccount {
                    name: user.name.clone(),
                    id: user.id.clone(),
                    keys: user.keys.clone(),
                    storage: crate::storage::configuration::StorageOptions {
                        users: user.storage.users.clone(),
                        size_total: user.storage.size_total,
                    },
                });
            }

            let debug = crate::storage::configuration::DebugOption {
                log: old_cfg.debug.log,
            };
            let routing = crate::storage::configuration::RoutingOptions {
                sending_table_period: old_cfg.routing.sending_table_period,
                ping_neighbour_period: old_cfg.routing.ping_neighbour_period,
                hop_count_penalty: old_cfg.routing.hop_count_penalty,
                maintain_period_limit: old_cfg.routing.maintain_period_limit,
            };

            let new_config = crate::storage::configuration::Configuration {
                node,
                lan,
                internet,
                user_accounts,
                debug,
                routing,
            };

            if let Ok(yaml) = serde_yaml::to_string(&new_config) {
                if let Err(_) = std::fs::write(Path::new(new_path).join("config.yaml"), yaml) {
                    println!("Error: creating config.yaml");
                    return false;
                }
            } else {
                println!("Error: config serialize");
                return false;
            }
            return true;
        }

        false
    }

    /// move unchanged contents
    fn move_contents(old_path: &str, new_path: &str) -> bool {
        let mut files: Vec<String> = vec![];
        let mut folders: Vec<String> = vec![];
        for entry_res in std::fs::read_dir(old_path).unwrap() {
            let entry = entry_res.unwrap();
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();
            if entry.file_type().unwrap().is_dir() {
                if file_name.starts_with(".") {
                    continue;
                }
                let path = String::from(file_name);
                folders.push(path);
            } else {
                if file_name == "version" || file_name == "config.yaml" {
                    continue;
                }
                let path = String::from(file_name);
                files.push(path);
            }
        }
        if backup::Backup::move_files(&files, old_path, new_path) == false {
            return false;
        }
        backup::Backup::move_folders(&folders, old_path, new_path)
    }
}
