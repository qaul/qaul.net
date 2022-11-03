use std::fs;
use std::path::Path;
extern crate fs_extra;

pub struct Backup {}
impl Backup {
    // pub fn copy_file(file: &str, to: &str) -> bool {
    //     let options = fs_extra::file::CopyOptions::new();
    //     fs_extra::file::copy(file, to, &options);
    //     true
    // }
    // pub fn copy_folder(folder: &str, to: &str) -> bool {
    //     let options = fs_extra::dir::CopyOptions::new();
    //     fs_extra::dir::copy(folder, to, &options);
    //     true
    // }

    /// move file
    pub fn move_file(file: &str, to: &str) -> bool {
        let options = fs_extra::file::CopyOptions::new();
        if let Err(_) = fs_extra::file::move_file(file, to, &options) {
            println!("Error: move file {} {}", file, to);
            return false;
        }
        true
    }

    /// move folder
    pub fn move_folder(folder: &str, to: &str) -> bool {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.content_only = true;
        if let Err(_) = fs_extra::dir::move_dir(folder, to, &options) {
            println!("Error: move folder {} {}", folder, to);
            return false;
        }
        true
    }

    /// remove folder
    pub fn remove_folder(folder: &str) -> bool {
        if let Err(_) = fs::remove_dir_all(folder) {
            println!("Error: remove folder {}", folder);
            return false;
        }
        true
    }

    /// move files
    pub fn move_files(files: &Vec<String>, src_path: &str, dst_path: &str) -> bool {
        for file in files {
            if let Some(src) = Path::new(src_path).join(file.as_str()).to_str() {
                if let Some(dst) = Path::new(dst_path).join(file.as_str()).to_str() {
                    if Self::move_file(src, dst) == false {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// move folders
    pub fn move_folders(folders: &Vec<String>, src_path: &str, dst_path: &str) -> bool {
        for folder in folders {
            if let Some(src) = Path::new(src_path).join(folder.as_str()).to_str() {
                if let Some(dst) = Path::new(dst_path).join(folder.as_str()).to_str() {
                    if Self::move_folder(src, dst) == false {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// backup storage folder into /backup/{old_version}
    pub fn backup(storage_path: String, old_version: &str) -> bool {
        //enumerate all files and folders
        let mut files: Vec<String> = vec![];
        let mut folders: Vec<String> = vec![];
        for entry_res in std::fs::read_dir(storage_path.as_str()).unwrap() {
            let entry = entry_res.unwrap();
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();
            if entry.file_type().unwrap().is_dir() {
                if file_name.starts_with(".") || file_name == "backup" || file_name == "logs" {
                    continue;
                }
                let path = String::from(file_name);
                folders.push(path);
            } else {
                let path = String::from(file_name);
                files.push(path);
            }
        }

        //destination path
        let path_dest = Path::new(storage_path.as_str())
            .join("backup")
            .join(old_version);

        //start backup
        //clear destination folder
        Self::remove_folder(path_dest.to_str().unwrap());
        //create dest dir
        if let Err(_) = std::fs::create_dir(path_dest.to_str().unwrap()) {
            println!("failed to crete destination folder");
            return false;
        }

        //move files
        if Self::move_files(&files, storage_path.as_str(), path_dest.to_str().unwrap()) == false {
            return false;
        }

        Self::move_folders(&folders, storage_path.as_str(), path_dest.to_str().unwrap())
    }

    /// restore contents from /backup/{version} to stroage folder
    pub fn restore(storage_path: String, version: &str) -> bool {
        let mut files: Vec<String> = vec![];
        let mut folders: Vec<String> = vec![];
        let scr_path = Path::new(storage_path.as_str())
            .join("backup")
            .join(version);
        for entry_res in std::fs::read_dir(scr_path.to_str().unwrap()).unwrap() {
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
        //move files
        if Self::move_files(&files, scr_path.to_str().unwrap(), storage_path.as_str()) == false {
            return false;
        }
        Self::move_folders(&folders, scr_path.to_str().unwrap(), storage_path.as_str())
    }
}
