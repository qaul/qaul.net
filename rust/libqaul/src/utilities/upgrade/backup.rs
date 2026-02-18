// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Backup Utility Functions

use fs_extra;
use std::path::Path;

/// Backup functions to move stored data
/// to the backup folder
pub struct Backup {}
impl Backup {
    /// move file
    pub fn move_file(file: &Path, to: &Path) -> bool {
        let options = fs_extra::file::CopyOptions::new();
        if let Err(_) = fs_extra::file::move_file(file, to, &options) {
            println!(
                "Error: move file {} {}",
                file.to_string_lossy(),
                to.to_string_lossy()
            );
            return false;
        }
        true
    }

    /// move folder
    pub fn move_folder(folder: &Path, to: &Path) -> bool {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.content_only = true;
        if let Err(_) = fs_extra::dir::move_dir(folder, to, &options) {
            println!(
                "Error: move folder {} {}",
                folder.to_string_lossy(),
                to.to_string_lossy()
            );
            return false;
        }
        true
    }

    /// remove folder
    pub fn remove_folder(folder: &Path) -> bool {
        if let Err(_) = fs_extra::dir::remove(folder) {
            println!("Error: remove folder {}", folder.to_string_lossy());
            return false;
        }
        true
    }

    /// move files
    pub fn move_files(files: &Vec<String>, src_path: &Path, dst_path: &Path) -> bool {
        for file in files {
            if !Self::move_file(&src_path.join(file), &dst_path.join(file)) {
                return false;
            }
        }
        true
    }

    /// move folders
    pub fn move_folders(folders: &Vec<String>, src_path: &Path, dst_path: &Path) -> bool {
        for folder in folders {
            if Self::move_folder(&src_path.join(folder), &dst_path.join(folder)) == false {
                return false;
            }
        }
        true
    }

    /// backup storage folder into /backup/{old_version}
    pub fn backup(storage_path: &Path, old_version: &str) -> bool {
        // enumerate all files and folders
        let mut files: Vec<String> = vec![];
        let mut folders: Vec<String> = vec![];

        // run through all files and collect those that we want to backup
        for entry_res in std::fs::read_dir(storage_path).unwrap() {
            let entry = entry_res.unwrap();
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();

            if entry.file_type().unwrap().is_dir() {
                // define which folders to backup
                if file_name.starts_with("12D3KooW") || file_name == "node.db" {
                    let path = String::from(file_name);
                    folders.push(path);
                }
            } else {
                // define which files to backup
                if file_name == "version" || file_name == "config.yaml" {
                    let path = String::from(file_name);
                    files.push(path);
                }
            }
        }

        // destination path
        let path_dest = Path::new(&storage_path).join("backup").join(old_version);

        // start backup
        // clear destination folder
        Self::remove_folder(&path_dest);
        // create dest dir
        if let Err(_) = std::fs::create_dir_all(&path_dest) {
            println!("failed to create destination folder");
            return false;
        }

        // move files
        if Self::move_files(&files, storage_path, path_dest.as_path()) == false {
            return false;
        }
        // move folders
        Self::move_folders(&folders, storage_path, path_dest.as_path())
    }

    /// restore content from /backup/{version} to storage folder
    pub fn restore(storage_path: &Path, backup_path: &Path) -> bool {
        let mut files: Vec<String> = vec![];
        let mut folders: Vec<String> = vec![];

        for entry_res in std::fs::read_dir(backup_path).unwrap() {
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
                let path = String::from(file_name);
                files.push(path);
            }
        }

        // move files & folders
        if Self::move_files(&files, backup_path, storage_path) == false {
            return false;
        }
        if Self::move_folders(&folders, backup_path, storage_path) == false {
            return false;
        }

        // create version file
        let version_path = storage_path.join("version");
        let cur_version: &str = env!("CARGO_PKG_VERSION");
        if let Err(_) = std::fs::write(version_path, cur_version) {
            println!("failed to write current version file!");
            return false;
        }

        true
    }
}
