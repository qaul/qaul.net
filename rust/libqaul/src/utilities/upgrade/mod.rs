// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Upgrade Module
//!
//! This module is used to automatically upgrade to a new version
//! with incompatible configuration or data base structure.
//!
//! The following upgrades to new versions are included:
//!
//! * 2.0.0-beta.9

use semver::Version;
use std::fs;
use std::path::Path;

use crate::utilities::upgrade::backup::Backup;

pub mod backup;
mod v2_0_0_beta_15;
mod v2_0_0_beta_17;
mod v2_0_0_beta_18;
mod v2_0_0_beta_9;

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

        // Move the existing content to the backup folder
        if Backup::backup(storage_path, old_version) == false {
            println!("backup failed");
            return false;
        }
        let mut backup_path = storage_path.join("backup").join(old_version);

        // upgrade one version after the other
        // put new upgrade version below this chain.

        // upgrade to version 2.0.0-beta.9
        if version < Version::parse("2.0.0-beta.9").unwrap() {
            match v2_0_0_beta_9::VersionUpgrade::upgrade(storage_path, &backup_path) {
                Ok((new_version, new_path)) => {
                    // update values
                    version = Version::parse(&new_version).unwrap();
                    backup_path = new_path;
                }
                Err(e) => {
                    println!("Upgrade to 2.0.0-beta.9 failed: {}", e);
                    return false;
                }
            }
        }

        // upgrade to version 2.0.0-beta.15
        if version < Version::parse("2.0.0-beta.15").unwrap() {
            match v2_0_0_beta_15::VersionUpgrade::upgrade(storage_path, &backup_path) {
                Ok((new_version, new_path)) => {
                    // update values
                    version = Version::parse(&new_version).unwrap();
                    backup_path = new_path;
                }
                Err(e) => {
                    println!("Upgrade to 2.0.0-beta.15 failed: {}", e);
                    return false;
                }
            }
        }

        // upgrade to version 2.0.0-beta.17
        if version < Version::parse("2.0.0-beta.17").unwrap() {
            match v2_0_0_beta_17::VersionUpgrade::upgrade(storage_path, &backup_path) {
                Ok((new_version, new_path)) => {
                    // update values
                    version = Version::parse(&new_version).unwrap();
                    backup_path = new_path;
                }
                Err(e) => {
                    println!("Upgrade to 2.0.0-beta.17 failed: {}", e);
                    return false;
                }
            }
        }

        // upgrade to version 2.0.0-beta.18
        if version < Version::parse("2.0.0-beta.18").unwrap() {
            match v2_0_0_beta_18::VersionUpgrade::upgrade(storage_path, &backup_path) {
                Ok((new_version, new_path)) => {
                    // update values
                    version = Version::parse(&new_version).unwrap();
                    backup_path = new_path;
                }
                Err(e) => {
                    println!("Upgrade to 2.0.0-beta.18 failed: {}", e);
                    return false;
                }
            }
        }

        // restore the upgraded last version
        log::trace!("restore upgraded version {}", version);
        if backup::Backup::restore(&storage_path, &backup_path) == true {
            // remove the latest upgraded
            backup::Backup::remove_folder(&storage_path.join("backup").join(version.to_string()));
            return true;
        }

        false
    }
}
