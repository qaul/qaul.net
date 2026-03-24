// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Account Management: Export, Delete, Restore
//!
//! Provides functions to export a user account to a portable archive,
//! delete an account from the node, and restore an account from an archive.

use base64::Engine;
use bs58;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use libp2p::{identity::ed25519, identity::Keypair, PeerId};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tar::{Archive, Builder};

use crate::router::connections::ConnectionTable;
use crate::router::users::Users;
use crate::rpc::authentication::Authentication;
use crate::services::chat::file::AllFiles;
use crate::services::chat::ChatStorage;
use crate::services::crypto::storage::CryptoStorage;
use crate::services::group::GroupStorage;
use crate::storage::configuration::{self, Configuration};
use crate::storage::database::DataBase;
use crate::storage::Storage;

use super::user_accounts::UserAccounts;

/// Manifest included in every export archive.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExportManifest {
    pub qaul_version: String,
    pub sled_version: String,
    pub export_date: String,
    pub user_id: String,
}

/// Serializable subset of the config UserAccount (excludes session_token).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExportedAccount {
    pub name: String,
    pub id: String,
    pub keys: String,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
    pub storage: configuration::StorageOptions,
}

pub struct AccountManagement;

impl AccountManagement {
    /// Export a user account to a `.qaul_export` archive (tar.gz).
    ///
    /// The archive contains a manifest, the account config entry,
    /// and the entire user storage directory.
    pub fn export_account(user_id: PeerId, output_path: &Path) -> Result<PathBuf, String> {
        // 1. Verify user exists
        UserAccounts::get_by_id(user_id)
            .ok_or_else(|| format!("User {} not found", user_id.to_base58()))?;

        // 2. Get config entry
        let exported_account = Self::get_exported_account(user_id)?;

        // 3. Flush cached sled::Tree handles
        ChatStorage::flush_account(&user_id);
        GroupStorage::flush_account(&user_id);

        // 4. Release sled::Tree handles from all caches
        ChatStorage::remove_account(user_id);
        GroupStorage::remove_account(user_id);
        CryptoStorage::remove_account(user_id);
        AllFiles::remove_account(user_id);

        // 5. Close user's sled DB (flush + drop handle)
        DataBase::close_user_db(user_id);

        // 6. Build archive
        let result = Self::build_export_archive(user_id, &exported_account, output_path);

        // 7. Re-open is lazy — next access to DataBase::get_user_db() will reopen

        result
    }

    /// Delete a user account and all associated data from this node.
    pub fn delete_account(user_id: PeerId) -> Result<(), String> {
        // 1. Verify user exists
        UserAccounts::get_by_id(user_id)
            .ok_or_else(|| format!("User {} not found", user_id.to_base58()))?;

        // 2. Flush cached sled::Tree handles
        ChatStorage::flush_account(&user_id);
        GroupStorage::flush_account(&user_id);

        // 3. Release sled::Tree handles from all caches
        ChatStorage::remove_account(user_id);
        GroupStorage::remove_account(user_id);
        CryptoStorage::remove_account(user_id);
        AllFiles::remove_account(user_id);

        // 4. Close user's sled DB
        DataBase::close_user_db(user_id);

        // 5. Remove from USERACCOUNTS in-memory state
        Self::remove_from_user_accounts(user_id);

        // 6. Remove from config and save
        Self::remove_from_config(user_id);

        // 7. Remove from routing tables
        ConnectionTable::remove_local_user(user_id);
        Users::remove(user_id);

        // 8. Remove authentication session (if auth system is initialized)
        Authentication::try_logout(user_id);

        // 9. Delete user directory from disk
        let user_dir = Storage::get_account_path(user_id);
        if user_dir.exists() {
            fs::remove_dir_all(&user_dir).map_err(|e| {
                format!(
                    "Failed to delete user directory {}: {}",
                    user_dir.display(),
                    e
                )
            })?;
        }

        log::info!("Deleted user account {}", user_id.to_base58());
        Ok(())
    }

    /// Restore a user account from a `.qaul_export` archive.
    ///
    /// Returns the PeerId of the restored account.
    pub fn restore_account(archive_path: &Path) -> Result<PeerId, String> {
        // 1. Extract archive to a temp directory
        let storage_path = Storage::get_path();
        let temp_dir = Path::new(&storage_path).join(".restore_temp");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).map_err(|e| e.to_string())?;
        }
        fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

        let extract_result = Self::extract_archive(archive_path, &temp_dir);
        if let Err(e) = extract_result {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(e);
        }

        // 2. Read and validate manifest
        let manifest_path = temp_dir.join("manifest.yaml");
        let manifest_str = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest.yaml: {}", e))?;
        let manifest: ExportManifest = serde_yaml_ng::from_str(&manifest_str)
            .map_err(|e| format!("Failed to parse manifest.yaml: {}", e))?;

        // Version compatibility check
        let current_version = env!("CARGO_PKG_VERSION");
        if manifest.qaul_version != current_version {
            log::warn!(
                "Archive was created with qaul version {}, current is {}",
                manifest.qaul_version,
                current_version
            );
        }

        // 3. Read account.yaml
        let account_path = temp_dir.join("account.yaml");
        let account_str = fs::read_to_string(&account_path)
            .map_err(|e| format!("Failed to read account.yaml: {}", e))?;
        let exported_account: ExportedAccount = serde_yaml_ng::from_str(&account_str)
            .map_err(|e| format!("Failed to parse account.yaml: {}", e))?;

        // 4. Parse user_id and check for conflicts
        let user_id_base58 = &exported_account.id;
        let mut key_bytes = base64::engine::general_purpose::STANDARD
            .decode(&exported_account.keys)
            .map_err(|e| format!("Failed to decode keys: {}", e))?;
        let ed25519_keys = ed25519::Keypair::try_from_bytes(&mut key_bytes)
            .map_err(|e| format!("Failed to parse Ed25519 keypair: {}", e))?;
        let keys = Keypair::from(ed25519_keys);
        let user_id = PeerId::from(keys.public());

        if user_id.to_base58() != *user_id_base58 {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err("User ID mismatch between account.yaml and derived key".to_string());
        }

        if UserAccounts::is_account(user_id) {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(format!(
                "User {} already exists on this node",
                user_id.to_base58()
            ));
        }

        let dest_user_dir = Storage::get_account_path(user_id);
        if dest_user_dir.exists() {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(format!(
                "User directory already exists: {}",
                dest_user_dir.display()
            ));
        }

        // 5. Move user directory from temp to storage
        let temp_user_dir = temp_dir.join(user_id_base58);
        if temp_user_dir.exists() {
            Self::copy_dir_all(&temp_user_dir, &dest_user_dir).map_err(|e| {
                let _ = fs::remove_dir_all(&temp_dir);
                format!("Failed to restore user directory: {}", e)
            })?;
        }

        // 6. Add to config and save
        {
            let mut config = Configuration::get_mut();
            config.user_accounts.push(configuration::UserAccount {
                name: exported_account.name.clone(),
                id: exported_account.id.clone(),
                keys: exported_account.keys.clone(),
                password_hash: exported_account.password_hash.clone(),
                password_salt: exported_account.password_salt.clone(),
                session_token: None,
                storage: exported_account.storage.clone(),
            });
        }
        Configuration::save();

        // 7. Add to in-memory USERACCOUNTS
        Self::add_to_user_accounts(user_id, keys.clone(), &exported_account);

        // 8. Open user's sled DB (lazy init)
        DataBase::get_user_db(user_id);

        // 9. Add to router tables
        Users::add(
            user_id,
            keys.public(),
            exported_account.name.clone(),
            false,
            false,
        );
        ConnectionTable::add_local_user(user_id);

        // 10. Clean up temp directory
        let _ = fs::remove_dir_all(&temp_dir);

        log::info!("Restored user account {}", user_id.to_base58());
        Ok(user_id)
    }

    // ---------------------------------------------------------------
    // String-based convenience methods (for CLI / external callers
    // that don't have access to libp2p::PeerId)
    // ---------------------------------------------------------------

    /// Export by base58 user ID string.
    pub fn export_account_by_id(user_id_base58: &str, output_path: &str) -> Result<String, String> {
        let user_id = Self::parse_peer_id(user_id_base58)?;
        let path = Path::new(output_path);
        let result = Self::export_account(user_id, path)?;
        Ok(result.to_string_lossy().to_string())
    }

    /// Delete by base58 user ID string.
    pub fn delete_account_by_id(user_id_base58: &str) -> Result<(), String> {
        let user_id = Self::parse_peer_id(user_id_base58)?;
        Self::delete_account(user_id)
    }

    /// Restore from archive path string. Returns the restored user's base58 ID.
    pub fn restore_account_from(archive_path: &str) -> Result<String, String> {
        let path = Path::new(archive_path);
        let user_id = Self::restore_account(path)?;
        Ok(user_id.to_base58())
    }

    // ---------------------------------------------------------------
    // Private helpers
    // ---------------------------------------------------------------

    fn parse_peer_id(base58: &str) -> Result<PeerId, String> {
        let bytes =
            bs58::decode(base58).into_vec().map_err(|e| format!("Invalid base58 '{}': {}", base58, e))?;
        PeerId::from_bytes(&bytes).map_err(|e| format!("Invalid PeerId '{}': {}", base58, e))
    }

    fn get_exported_account(user_id: PeerId) -> Result<ExportedAccount, String> {
        let config = Configuration::get();
        let user_id_str = user_id.to_string();
        let config_account = config
            .user_accounts
            .iter()
            .find(|u| u.id == user_id_str)
            .ok_or_else(|| format!("User {} not found in config", user_id.to_base58()))?;

        Ok(ExportedAccount {
            name: config_account.name.clone(),
            id: config_account.id.clone(),
            keys: config_account.keys.clone(),
            password_hash: config_account.password_hash.clone(),
            password_salt: config_account.password_salt.clone(),
            storage: config_account.storage.clone(),
        })
    }

    fn build_export_archive(
        user_id: PeerId,
        exported_account: &ExportedAccount,
        output_path: &Path,
    ) -> Result<PathBuf, String> {
        let storage_path = Storage::get_path();
        let user_id_base58 = user_id.to_base58();

        // Create staging directory
        let staging_dir = Path::new(&storage_path).join(".export_staging");
        if staging_dir.exists() {
            fs::remove_dir_all(&staging_dir).map_err(|e| e.to_string())?;
        }
        fs::create_dir_all(&staging_dir).map_err(|e| e.to_string())?;

        // Write manifest.yaml
        let manifest = ExportManifest {
            qaul_version: env!("CARGO_PKG_VERSION").to_string(),
            sled_version: "0.34.7".to_string(),
            export_date: humantime::format_rfc3339(std::time::SystemTime::now()).to_string(),
            user_id: user_id_base58.clone(),
        };
        let manifest_yaml =
            serde_yaml_ng::to_string(&manifest).map_err(|e| e.to_string())?;
        fs::write(staging_dir.join("manifest.yaml"), manifest_yaml)
            .map_err(|e| e.to_string())?;

        // Write account.yaml
        let account_yaml =
            serde_yaml_ng::to_string(exported_account).map_err(|e| e.to_string())?;
        fs::write(staging_dir.join("account.yaml"), account_yaml)
            .map_err(|e| e.to_string())?;

        // Copy entire user directory
        let user_dir = Path::new(&storage_path).join(&user_id_base58);
        if user_dir.exists() {
            let dest = staging_dir.join(&user_id_base58);
            Self::copy_dir_all(&user_dir, &dest).map_err(|e| {
                format!("Failed to copy user directory: {}", e)
            })?;
        }

        // Create tar.gz archive
        let archive_name = format!("{}.qaul_export", user_id_base58);
        let archive_path = output_path.join(&archive_name);
        Self::create_tar_gz(&staging_dir, &archive_path)?;

        // Clean up staging directory
        let _ = fs::remove_dir_all(&staging_dir);

        Ok(archive_path)
    }

    fn create_tar_gz(source_dir: &Path, output_file: &Path) -> Result<(), String> {
        let file =
            fs::File::create(output_file).map_err(|e| format!("Failed to create archive: {}", e))?;
        let enc = GzEncoder::new(file, Compression::default());
        let mut tar = Builder::new(enc);
        tar.append_dir_all(".", source_dir)
            .map_err(|e| format!("Failed to write archive contents: {}", e))?;
        tar.into_inner()
            .map_err(|e| format!("Failed to finalize tar: {}", e))?
            .finish()
            .map_err(|e| format!("Failed to finalize gz: {}", e))?;
        Ok(())
    }

    fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<(), String> {
        let file = fs::File::open(archive_path)
            .map_err(|e| format!("Failed to open archive: {}", e))?;
        let dec = GzDecoder::new(file)
            .map_err(|e| format!("Failed to create gz decoder: {}", e))?;
        let mut archive = Archive::new(dec);
        archive
            .unpack(dest_dir)
            .map_err(|e| format!("Failed to extract archive: {}", e))?;
        Ok(())
    }

    fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let dest_path = dst.join(entry.file_name());
            if file_type.is_dir() {
                Self::copy_dir_all(&entry.path(), &dest_path)?;
            } else {
                fs::copy(entry.path(), &dest_path)?;
            }
        }
        Ok(())
    }

    fn remove_from_user_accounts(user_id: PeerId) {
        UserAccounts::remove(user_id);
    }

    fn remove_from_config(user_id: PeerId) {
        let user_id_str = user_id.to_string();
        {
            let mut config = Configuration::get_mut();
            config.user_accounts.retain(|u| u.id != user_id_str);
        }
        Configuration::save();
    }

    fn add_to_user_accounts(user_id: PeerId, keys: Keypair, account: &ExportedAccount) {
        let user = super::user_accounts::UserAccount {
            id: user_id,
            keys,
            name: account.name.clone(),
            password_hash: account.password_hash.clone(),
            password_salt: account.password_salt.clone(),
        };
        UserAccounts::add(user);
    }
}
