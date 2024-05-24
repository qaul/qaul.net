// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Upgrade Module
//!
//! This module is used to automatically upgrade to a new version
//! with incompatible configuration or data base structure.
//!
//! The last version an upgrade is possible from is version 2.0.0-beta.18

use semver::Version;
use std::fs;
use std::path::Path;

use crate::utilities::upgrade::backup::Backup;

pub mod backup;
pub mod v2_0_0_rc_1;

/// upgrade module
pub struct Upgrade {}
impl Upgrade {
    /// initialize and run upgrade module
    ///
    /// requires the path to the data storage folder
    pub fn init(storage_path: String) -> bool {
        // get current version from Cargo.toml
        let cur_version: &str = env!("CARGO_PKG_VERSION");
        println!("running libqaul {}", cur_version);
        let mut old_version: String = String::from(cur_version);
        let storage_path_buf = Path::new(&storage_path);

        // read old version
        let path = storage_path_buf.join("version");
        if path.exists() == false {
            // check if there is already a configuration file
            let config_path = storage_path_buf.join("config.yaml");
            if config_path.exists() {
                old_version = "2.0.0-beta.8".to_string();
            }
            // create new version file otherwise
            else if let Err(_) = fs::write(path.clone(), cur_version) {
                println!(
                    "failed to creating version file path: {}",
                    path.to_str().unwrap()
                );
                return false;
            }
        } else {
            old_version = fs::read_to_string(path).unwrap();
        }

        // check if old version is equal to new version
        if cur_version == old_version {
            println!("libqaul data on latest version");
        } else {
            // run upgrade steps if the old and the new version differ
            return Self::upgrade(&storage_path_buf, &old_version);
        }
        true
    }

    /// process the upgrade steps one after the next
    fn upgrade(storage_path: &Path, old_version: &str) -> bool {
        println!("running upgrade check for version {}", old_version);

        let mut version = Version::parse(old_version).unwrap();

        // check if libqaul is upgradable
        // the last possible upgradable version at the moment is 2.0.0-beta.18
        // all previous versions need to upgrade to that one in order to upgrade further.
        let last_upgradable_version = Version::parse("2.0.0-beta.18").unwrap();
        if version < last_upgradable_version {
            // issue an informative message
            println!(
                "Your current qaul version {} is too old to be upgraded to this version directly.",
                version
            );
            println!(
                "Please install to qaul version {} first, before upgrading to this version.",
                last_upgradable_version
            );

            return false;
        }

        // Move the existing content to the backup folder
        if Backup::backup(storage_path, old_version) == false {
            println!("backup failed");
            return false;
        }
        let mut backup_path = storage_path.join("backup").join(old_version);

        // upgrade one version after the other
        // put new upgrade version below this chain.

        // upgrade to version 2.0.0-rc.1
        if version < Version::parse("2.0.0-rc.1").unwrap() {
            match v2_0_0_rc_1::VersionUpgrade::upgrade(storage_path, &backup_path) {
                Ok((new_version, new_path)) => {
                    // update values
                    version = Version::parse(&new_version).unwrap();
                    backup_path = new_path;
                }
                Err(e) => {
                    println!("Upgrade to 2.0.0-rc.1 failed: {}", e);
                    return false;
                }
            }
        }

        // restore the upgraded last version
        if backup::Backup::restore(&storage_path, &backup_path) == true {
            // remove the latest upgraded
            backup::Backup::remove_folder(&storage_path.join("backup").join(version.to_string()));
            println!("libqaul successfully upgraded");
            return true;
        }

        false
    }
}
