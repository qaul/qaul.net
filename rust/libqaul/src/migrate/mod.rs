// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Migrating Module
//!
use std::fs;
use std::path::Path;

pub mod backup;
pub mod mig200b4to200b5;

// whenever update version and structure, we have to track them.
// (version, structure_updated)
static G_HISTORIES: &'static [(&str, bool)] = &[("2.0.0-beta.4", true), ("2.0.0-beta.5", true)];

/// check version and determin if we need to migrate.
struct MigrateStep {}
impl MigrateStep {
    /// get migrating steps such as [(beta.4, beta.5), (beta.5, beta.6)]
    pub fn get_migrate_steps(old_version: &str, cur_version: &str) -> Vec<(usize, usize)> {
        if let Some(idx_old) = Self::version_to_index(old_version) {
            if let Some(idx_cur) = Self::version_to_index(cur_version) {
                return Self::check_tasks(idx_old, idx_cur);
            }
        }
        vec![]
    }

    /// convert version striong into index
    fn version_to_index(version: &str) -> Option<usize> {
        for i in 0..G_HISTORIES.len() {
            if version == G_HISTORIES[i].0 {
                return Some(i);
            }
        }
        None
    }

    /// check migrating
    fn check_migrate(idx_version: usize) -> (usize, usize) {
        //find left
        let mut idx_left = idx_version;
        let mut idx_right = idx_version + 1;

        for i in (0..(idx_version + 1)).rev() {
            if G_HISTORIES[i].1 == true {
                idx_left = i;
                break;
            }
        }

        for i in (idx_version + 1)..G_HISTORIES.len() {
            if G_HISTORIES[i].1 == true {
                idx_right = i;
                break;
            }
        }
        (idx_left, idx_right)
    }

    /// check all migrating steps
    fn check_tasks(idx_old: usize, idx_target: usize) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = vec![];
        let mut cur_idx = idx_old;

        loop {
            let migrate = Self::check_migrate(cur_idx);
            cur_idx = migrate.1;
            if cur_idx >= idx_target {
                if cur_idx == idx_target && G_HISTORIES[idx_target].1 == true {
                    res.push(migrate);
                }
                break;
            } else {
                res.push(migrate);
            }
        }
        res
    }
}

/// migrate module structure
pub struct Migrate {}
impl Migrate {
    /// initialize migrate module
    /// requires the path to the data storage folder
    pub fn init(storage_path: String) -> bool {
        let cur_version: &str = env!("CARGO_PKG_VERSION");
        let mut old_version: String = String::from(cur_version.clone());

        //read old version
        let path = Path::new(storage_path.as_str()).join("version");
        if path.exists() == false {
            // create new version file
            if let Err(_) = fs::write(path.clone(), cur_version) {
                println!(
                    "failed to creating version file path: {}",
                    path.to_str().unwrap()
                );
                return false;
            }
        } else {
            let old_version_str = fs::read_to_string(path).unwrap();
            old_version = old_version_str.clone();
        }
        println!("checking version {}", cur_version);

        //check migrate task
        let tasks = MigrateStep::get_migrate_steps(old_version.as_str(), cur_version.clone());
        if tasks.len() == 0 {
            return true;
        }

        //first backup
        println!("");
        println!(
            "#Starting migration\n\t##backup old version {}  ...",
            old_version
        );
        backup::Backup::backup(storage_path.clone(), old_version.as_str());

        Self::migrate_all(storage_path.clone(), &tasks)
    }

    /// process all migrating steps
    fn migrate_all(storage_path: String, tasks: &Vec<(usize, usize)>) -> bool {
        for (old_idx, next_idx) in tasks {
            if Self::migrate_one(
                storage_path.clone(),
                G_HISTORIES[*old_idx].0,
                G_HISTORIES[*next_idx].0,
            ) == false
            {
                println!(
                    "\t\tmigrating from {} to {} was failed!",
                    G_HISTORIES[*old_idx].0, G_HISTORIES[*next_idx].0
                );
                return false;
            }
        }

        let last_version = G_HISTORIES[tasks[tasks.len() - 1].1].0;

        //restore the migrated last version
        println!("\n\t#restore latest migrated {}", last_version);
        if backup::Backup::restore(storage_path.clone(), last_version) == true {
            //remove the latest migrated
            backup::Backup::remove_folder(
                Path::new(storage_path.as_str())
                    .join("backup")
                    .join(last_version)
                    .to_str()
                    .unwrap(),
            );
            return true;
        }
        false
    }

    /// process one migrating step
    fn migrate_one(storage_path: String, old_version: &str, next_version: &str) -> bool {
        println!("\n\t##migrate one {} - {}", old_version, next_version);

        let old_path = Path::new(storage_path.as_str())
            .join("backup")
            .join(old_version);
        let next_path = Path::new(storage_path.as_str())
            .join("backup")
            .join(next_version);

        let mut result = false;
        match old_version {
            "2.0.0-beta.4" => {
                result = mig200b4to200b5::Mig200b4To200b5::do_process(
                    old_path.to_str().unwrap(),
                    next_path.to_str().unwrap(),
                    next_version,
                );
            }
            _ => {
                println!("undefined migrating version");
            }
        }

        if result == true {
            //remove old version
            backup::Backup::remove_folder(old_path.to_str().unwrap());
        }
        result
    }
}
