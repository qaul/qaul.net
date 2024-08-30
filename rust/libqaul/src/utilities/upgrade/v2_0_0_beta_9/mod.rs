// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Upgrade to new version 2.0.0-beta.9
//!
//! Breaking changes that need to be upgraded:
//!
//! * configuration file: InternetPeer structure added name field:
//!   config.internet.peers<InternetPeer>

use std::path::{Path, PathBuf};

mod old_config;

use super::backup;

/// # Version Upgrade Logic
pub struct VersionUpgrade {}
impl VersionUpgrade {
    /// Upgrade to new Version
    ///
    /// Returns a result, containing a tuple with ( new_version, new_path )
    pub fn upgrade(storage_path: &Path, old_path: &Path) -> Result<(String, PathBuf), String> {
        let version = "2.0.0-beta.9";
        println!("upgrade to version {}", version);
        let new_path = storage_path.join("backup").join(version);

        // cleanup dest
        backup::Backup::remove_folder(&new_path);

        // create dest
        if let Err(_) = std::fs::create_dir(&new_path) {
            return Err("failed to create destinaton folder".to_string());
        }

        // move unchanged contents
        println!("move content");
        if Self::move_content(Path::new(old_path), &new_path) == false {
            return Err("Error moving content".to_string());
        }

        // create new version file
        println!("create version file");
        let path = Path::new(new_path.to_str().unwrap()).join("version");
        if let Err(_) = std::fs::write(path, version) {
            println!("failed to create version file!");
        }

        // update config.yaml
        println!("upgrade config.yaml");
        if !Self::upgrade_config(old_path, &new_path) {
            return Err("configuration upgrade failed".to_string());
        }

        // remove old backup
        backup::Backup::remove_folder(old_path);

        Ok((version.to_string(), new_path))
    }

    /// upgrade config structure
    fn upgrade_config(old_path: &Path, new_path: &Path) -> bool {
        // load old config
        if let Some(old_cfg) =
            old_config::Configuration::load(old_path.join("config.yaml").to_str().unwrap())
        {
            let node = super::v2_0_0_beta_15::old_config::Node {
                initialized: old_cfg.node.initialized,
                id: old_cfg.node.id.clone(),
                keys: old_cfg.node.keys.clone(),
            };
            let lan = super::v2_0_0_beta_15::old_config::Lan {
                active: old_cfg.lan.active,
                listen: old_cfg.lan.listen,
            };

            let mut peers: Vec<super::v2_0_0_beta_15::old_config::InternetPeer> = vec![];
            for peer in &old_cfg.internet.peers {
                peers.push(super::v2_0_0_beta_15::old_config::InternetPeer {
                    address: peer.address.clone(),
                    name: String::from(""),
                    enabled: peer.enabled,
                });
            }
            let internet = super::v2_0_0_beta_15::old_config::Internet {
                active: old_cfg.internet.active,
                peers,
                do_listen: old_cfg.internet.do_listen,
                listen: old_cfg.internet.listen,
            };

            let mut user_accounts: Vec<super::v2_0_0_beta_15::old_config::UserAccount> = vec![];
            for user in &old_cfg.user_accounts {
                user_accounts.push(super::v2_0_0_beta_15::old_config::UserAccount {
                    name: user.name.clone(),
                    id: user.id.clone(),
                    keys: user.keys.clone(),
                    storage: super::v2_0_0_beta_15::old_config::StorageOptions {
                        users: user.storage.users.clone(),
                        size_total: user.storage.size_total,
                    },
                });
            }

            let debug = super::v2_0_0_beta_15::old_config::DebugOption {
                log: old_cfg.debug.log,
            };
            let routing = super::v2_0_0_beta_15::old_config::RoutingOptions {
                sending_table_period: old_cfg.routing.sending_table_period,
                ping_neighbour_period: old_cfg.routing.ping_neighbour_period,
                hop_count_penalty: old_cfg.routing.hop_count_penalty,
                maintain_period_limit: old_cfg.routing.maintain_period_limit,
            };

            let new_config = super::v2_0_0_beta_15::old_config::Configuration {
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

    /// move unchanged content
    ///
    /// TODO: write this function more generic in backup, to only
    /// provide paths of folders and files, that should be ignored by the backup
    fn move_content(old_path: &Path, new_path: &Path) -> bool {
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

        if super::backup::Backup::move_files(&files, old_path, new_path) == false {
            return false;
        }
        super::backup::Backup::move_folders(&folders, old_path, new_path)
    }
}
