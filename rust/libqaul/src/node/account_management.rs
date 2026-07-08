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
use crate::rpc::Rpc;

/// Protobuf message definitions for the account-management RPC.
pub use qaul_proto::qaul_rpc_account_management as proto;
/// Shared RPC response / error types used by the generated service dispatch.
use qaul_proto::qaul_common::{Ack, RpcError};

/// Manifest included in every export archive.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExportManifest {
    pub qaul_version: String,
    pub sled_version: String,
    pub export_date: String,
    pub user_id: String,
}

pub struct AccountManagement;

impl AccountManagement {
    /// Export a user account to a `.qaul_export` archive (tar.gz).
    ///
    /// The archive contains a manifest, the account config entry,
    /// and the entire user storage directory.
    pub fn export_account(
        state: &crate::QaulState,
        user_id: PeerId,
        output_path: &Path,
    ) -> Result<PathBuf, String> {
        // 1. Verify user exists
        UserAccounts::get_by_id(state, user_id)
            .ok_or_else(|| format!("User {} not found", user_id.to_base58()))?;

        // 2. Get config entry
        let exported_account = Self::get_exported_account(state, user_id)?;

        // 3. Quiesce all per-user storage so the directory can be archived.
        Self::quiesce_user_storage(state, user_id);

        // 4. Build archive
        let result = Self::build_export_archive(state, user_id, &exported_account, output_path);

        // 5. Re-open is lazy — next access to DataBase::get_user_db() will reopen

        result
    }

    /// Delete a user account and all associated data from this node.
    pub fn delete_account(state: &crate::QaulState, user_id: PeerId) -> Result<(), String> {
        // 1. Verify user exists
        UserAccounts::get_by_id(state, user_id)
            .ok_or_else(|| format!("User {} not found", user_id.to_base58()))?;

        // 2. Quiesce all per-user storage so the directory can be removed.
        Self::quiesce_user_storage(state, user_id);

        // 3. Remove from in-memory user accounts
        UserAccounts::remove(state, user_id);

        // 4. Remove from config and save
        Self::remove_from_config(state, user_id);

        // 5. Remove from routing tables
        let router = state.get_router();
        ConnectionTable::remove_local_user(&router, user_id);
        Users::remove(state, &router, user_id);

        // 6. Remove authentication session (no-op if not logged in)
        Authentication::logout(state, user_id);

        // 7. Delete user directory from disk
        let user_dir = Storage::get_account_path(state, user_id);
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
    pub fn restore_account(
        state: &crate::QaulState,
        archive_path: &Path,
    ) -> Result<PeerId, String> {
        // 1. Extract archive to a temp directory
        let storage_path = Storage::get_path(state);
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
        let exported_account: configuration::UserAccount = serde_yaml_ng::from_str(&account_str)
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

        if UserAccounts::is_account(state, user_id) {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(format!(
                "User {} already exists on this node",
                user_id.to_base58()
            ));
        }

        // The account is not registered on this node (verified by the
        // `is_account` check above), so a directory here is an orphan left by
        // an incomplete teardown — e.g. a background storage access lazily
        // re-created an empty `user.db` (via `DataBase::get_user_db`) after
        // `delete_account` had already removed the real directory. Reclaim it
        // instead of hard-failing; erroring here would permanently soft-lock
        // re-import of an account the node otherwise has no trace of.
        let dest_user_dir = Storage::get_account_path(state, user_id);
        if dest_user_dir.exists() {
            log::warn!(
                "Reclaiming orphan directory for unregistered account before restore: {}",
                dest_user_dir.display()
            );
            // Release any lingering sled handle so the on-disk directory and
            // its cache entry are gone before the restored data is laid down
            // (and so step 8's lazy open reopens the restored DB, not a stale
            // handle to the removed inode).
            DataBase::close_user_db(state, user_id);
            if let Err(e) = fs::remove_dir_all(&dest_user_dir) {
                let _ = fs::remove_dir_all(&temp_dir);
                return Err(format!(
                    "Failed to reclaim orphan user directory {}: {}",
                    dest_user_dir.display(),
                    e
                ));
            }
        }

        // 5. Move user directory from temp to storage
        let temp_user_dir = temp_dir.join(user_id_base58);
        if temp_user_dir.exists() {
            Self::copy_dir_all(&temp_user_dir, &dest_user_dir).map_err(|e| {
                let _ = fs::remove_dir_all(&temp_dir);
                format!("Failed to restore user directory: {}", e)
            })?;
        }

        // 6. Add to config and save. The archive stores the config account
        //    verbatim (including `session_token`, so qauld deployments keep
        //    automated clients' sessions working after a restore).
        {
            let mut config = Configuration::get_mut(state);
            config.user_accounts.push(exported_account.clone());
        }
        Configuration::save(state);

        // 7. Add to in-memory user accounts
        Self::add_to_user_accounts(state, user_id, keys.clone(), &exported_account);

        // 8. Open user's sled DB (lazy init)
        DataBase::get_user_db(state, user_id);

        // 9. Register into the router via the canonical local-user path,
        //    identical to account creation. The old-format archive carries no
        //    profile data, so a fresh signed profile is regenerated from the
        //    restored keypair.
        let router = state.get_router();
        Users::register_local_user(state, &router, exported_account.name.clone(), &keys);

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
    pub fn export_account_by_id(
        state: &crate::QaulState,
        user_id_base58: &str,
        output_path: &str,
    ) -> Result<String, String> {
        let user_id = Self::parse_peer_id(user_id_base58)?;
        let path = Path::new(output_path);
        let result = Self::export_account(state, user_id, path)?;
        Ok(result.to_string_lossy().to_string())
    }

    /// Delete by base58 user ID string.
    pub fn delete_account_by_id(
        state: &crate::QaulState,
        user_id_base58: &str,
    ) -> Result<(), String> {
        let user_id = Self::parse_peer_id(user_id_base58)?;
        Self::delete_account(state, user_id)
    }

    /// Restore from archive path string. Returns the restored user's base58 ID.
    pub fn restore_account_from(
        state: &crate::QaulState,
        archive_path: &str,
    ) -> Result<String, String> {
        let path = Path::new(archive_path);
        let user_id = Self::restore_account(state, path)?;
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

    fn get_exported_account(
        state: &crate::QaulState,
        user_id: PeerId,
    ) -> Result<configuration::UserAccount, String> {
        let config = Configuration::get(state);
        let user_id_str = user_id.to_string();
        config
            .user_accounts
            .iter()
            .find(|u| u.id == user_id_str)
            .cloned()
            .ok_or_else(|| format!("User {} not found in config", user_id.to_base58()))
    }

    /// Flush and release every per-user storage handle, then close the user's
    /// sled DB, so the on-disk directory can be safely archived or removed.
    ///
    /// Cached `sled::Tree` handles hold `Arc`s to the DB, so they must all be
    /// dropped before the DB can be closed and its file locks released. Any new
    /// per-user sled cache MUST be released here too: this is the single site
    /// both export and delete rely on to quiesce a user's on-disk state.
    fn quiesce_user_storage(state: &crate::QaulState, user_id: PeerId) {
        // Flush buffered writes for the caches that buffer.
        ChatStorage::flush_account(state, &user_id);
        GroupStorage::flush_account(state, &user_id);

        // Release cached Tree handles from every per-user cache.
        ChatStorage::remove_account(state, user_id);
        GroupStorage::remove_account(state, user_id);
        CryptoStorage::remove_account(state, user_id);
        AllFiles::remove_account(state, user_id);

        // Close the user's sled DB (flush + drop handle, releasing file locks).
        DataBase::close_user_db(state, user_id);
    }

    fn build_export_archive(
        state: &crate::QaulState,
        user_id: PeerId,
        exported_account: &configuration::UserAccount,
        output_path: &Path,
    ) -> Result<PathBuf, String> {
        let storage_path = Storage::get_path(state);
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
        let dec = GzDecoder::new(file);
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

    fn remove_from_config(state: &crate::QaulState, user_id: PeerId) {
        let user_id_str = user_id.to_string();
        {
            let mut config = Configuration::get_mut(state);
            config.user_accounts.retain(|u| u.id != user_id_str);
        }
        Configuration::save(state);
    }

    fn add_to_user_accounts(
        state: &crate::QaulState,
        user_id: PeerId,
        keys: Keypair,
        account: &configuration::UserAccount,
    ) {
        let user = super::user_accounts::UserAccount {
            id: user_id,
            keys,
            name: account.name.clone(),
            password_hash: account.password_hash.clone(),
            password_salt: account.password_salt.clone(),
        };
        UserAccounts::add(state, user);
    }
}

impl AccountManagement {
    /// Process incoming RPC request messages for the account-management module.
    ///
    /// Thin shim over the generated service dispatcher: it decodes the
    /// `AccountManagement` envelope, routes the request to the matching
    /// `AccountManagementService` method, and encodes the reply — then sends
    /// it back on the ACCOUNT_MANAGEMENT module channel. All per-method logic
    /// lives in the `AccountManagementService` impl below.
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
        let ctx = crate::RequestContext {
            state,
            user_id,
            request_id: request_id.clone(),
        };
        let response_bytes = proto::dispatch::<crate::RequestContext, AccountManagement>(&ctx, data);
        Rpc::send_message(
            state,
            response_bytes,
            crate::rpc::proto::Modules::AccountManagement.into(),
            request_id,
            Vec::new(),
        );
    }
}

/// Map a libqaul `String` error into the uniform RPC error variant.
fn rpc_error(message: String) -> RpcError {
    RpcError {
        code: 2,
        message,
        details: String::new(),
    }
}

/// Generated-service implementation exposing export / delete / restore over
/// RPC. `export` and `delete` are self-scoped: they act on the calling account,
/// taken from the [`crate::RequestContext`] (outer envelope), not from the
/// request body — a caller can only export/delete its own account. (A by-id
/// node-admin variant can be reintroduced behind an authorization gate if a
/// multi-account deployment needs it.) `restore` mints a new account from an
/// archive, so it has no caller identity to scope to.
impl proto::AccountManagementService<crate::RequestContext<'_>> for AccountManagement {
    fn export(
        ctx: &crate::RequestContext<'_>,
        req: proto::ExportAccountRequest,
    ) -> Result<proto::ExportAccountResponse, RpcError> {
        let state = ctx.state;
        let peer_id = PeerId::from_bytes(&ctx.user_id)
            .map_err(|_| rpc_error("invalid caller identity".to_string()))?;

        // Empty output_path defaults to the storage root.
        let output_dir = if req.output_path.is_empty() {
            PathBuf::from(Storage::get_path(state))
        } else {
            PathBuf::from(req.output_path)
        };

        let path = Self::export_account(state, peer_id, &output_dir).map_err(rpc_error)?;

        Ok(proto::ExportAccountResponse {
            path: path.to_string_lossy().into_owned(),
        })
    }

    fn delete(ctx: &crate::RequestContext<'_>, _req: proto::DeleteAccountRequest) -> Result<Ack, RpcError> {
        let peer_id = PeerId::from_bytes(&ctx.user_id)
            .map_err(|_| rpc_error("invalid caller identity".to_string()))?;

        Self::delete_account(ctx.state, peer_id).map_err(rpc_error)?;
        Ok(Ack {})
    }

    fn restore(
        ctx: &crate::RequestContext<'_>,
        req: proto::RestoreAccountRequest,
    ) -> Result<proto::RestoreAccountResponse, RpcError> {
        let state = ctx.state;
        let peer_id =
            Self::restore_account(state, Path::new(&req.archive_path)).map_err(rpc_error)?;

        Ok(proto::RestoreAccountResponse {
            user_id: peer_id.to_bytes(),
            user_id_base58: peer_id.to_base58(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `session_token` set on an account survives the export YAML
    /// round-trip, so a restored account keeps automated clients'
    /// existing sessions working (qauld deployment use case).
    #[test]
    fn exported_account_roundtrips_session_token() {
        let account = configuration::UserAccount {
            name: "alice".to_string(),
            id: "12D3KooExample".to_string(),
            keys: "base64keys".to_string(),
            password_hash: Some("hash".to_string()),
            password_salt: Some("salt".to_string()),
            session_token: Some("session-abc".to_string()),
            storage: configuration::StorageOptions::default(),
        };

        let yaml = serde_yaml_ng::to_string(&account).expect("serialize");
        assert!(
            yaml.contains("session-abc"),
            "token must be written into the export archive"
        );

        let restored: configuration::UserAccount =
            serde_yaml_ng::from_str(&yaml).expect("deserialize");
        assert_eq!(restored.session_token.as_deref(), Some("session-abc"));
    }
}
