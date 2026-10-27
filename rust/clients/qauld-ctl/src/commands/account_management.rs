// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Account export / delete / restore.
//!
//! Requests go to libqaul's `AccountManagement` module. `export` and
//! `delete` act on the calling account, i.e. the default user resolved
//! by the preflight request; `restore` mints a new account from an
//! archive and needs no caller identity.
//!
//! File paths are opened by the daemon, which runs in its own working
//! directory, so they are made absolute here before being sent.

use std::path::Path;

use prost::Message;

use crate::proto::Modules;

use qaul_proto::qaul_rpc_account_management as proto;

use proto::{
    account_management, AccountManagement, DeleteAccountRequest, ExportAccountRequest,
    RestoreAccountRequest,
};

/// Resolve a client-side path to an absolute path the daemon can open.
fn absolute_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let absolute = std::fs::canonicalize(Path::new(path))
        .map_err(|e| format!("cannot access '{}': {}", path, e))?;
    Ok(absolute.to_string_lossy().into_owned())
}

fn encode(message: account_management::Message) -> (Vec<u8>, Modules) {
    let proto_message = AccountManagement {
        message: Some(message),
    };
    (proto_message.encode_to_vec(), Modules::AccountManagement)
}

/// Export the calling account to a `.qaul_export` archive in `output_dir`
/// (the daemon's storage directory when omitted).
pub fn encode_export(
    output_dir: Option<&str>,
) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
    let output_path = match output_dir {
        Some(dir) => {
            let dir = absolute_path(dir)?;
            if !Path::new(&dir).is_dir() {
                return Err(format!("'{}' is not a directory", dir).into());
            }
            dir
        }
        None => String::new(),
    };

    Ok(encode(account_management::Message::ExportAccountRequest(
        ExportAccountRequest { output_path },
    )))
}

/// Delete the calling account and all of its data. Refuses unless `confirmed`.
pub fn encode_delete(confirmed: bool) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
    if !confirmed {
        return Err(
            "account delete permanently removes the default account and all of its \
                    data; run `account default` to check which account that is, then \
                    re-run with --yes"
                .into(),
        );
    }

    Ok(encode(account_management::Message::DeleteAccountRequest(
        DeleteAccountRequest {},
    )))
}

/// Restore an account from a `.qaul_export` archive.
pub fn encode_restore(archive: &str) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
    let archive_path = absolute_path(archive)?;
    if !Path::new(&archive_path).is_file() {
        return Err(format!("'{}' is not a file", archive_path).into());
    }

    Ok(encode(account_management::Message::RestoreAccountRequest(
        RestoreAccountRequest { archive_path },
    )))
}

pub fn decode_response(data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let response = AccountManagement::decode(data)?;
    match response.message {
        Some(account_management::Message::ExportAccountResponse(r)) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({ "path": r.path }))?
                );
            } else {
                println!("Account exported to: {}", r.path);
            }
        }
        Some(account_management::Message::RestoreAccountResponse(r)) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "user_id": r.user_id_base58,
                    }))?
                );
            } else {
                println!("Account restored: {}", r.user_id_base58);
            }
        }
        Some(account_management::Message::Ack(_)) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({ "ok": true }))?
                );
            } else {
                println!("Account deleted.");
            }
        }
        Some(account_management::Message::Error(e)) => {
            return Err(format!("account: {} (code {})", e.message, e.code).into());
        }
        _ => {
            return Err("unprocessable RPC account management message".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_request(buf: &[u8]) -> account_management::Message {
        AccountManagement::decode(buf)
            .expect("valid AccountManagement payload")
            .message
            .expect("message set")
    }

    #[test]
    fn export_without_dir_leaves_path_empty() {
        let (buf, module) = encode_export(None).unwrap();
        assert_eq!(module, Modules::AccountManagement);
        match decode_request(&buf) {
            account_management::Message::ExportAccountRequest(r) => {
                assert!(r.output_path.is_empty())
            }
            other => panic!("unexpected message {:?}", other),
        }
    }

    #[test]
    fn export_dir_is_sent_as_absolute_path() {
        let (buf, _) = encode_export(Some(".")).unwrap();
        match decode_request(&buf) {
            account_management::Message::ExportAccountRequest(r) => {
                assert!(Path::new(&r.output_path).is_absolute())
            }
            other => panic!("unexpected message {:?}", other),
        }
    }

    #[test]
    fn export_rejects_missing_dir() {
        assert!(encode_export(Some("./does-not-exist-qauld-ctl-test")).is_err());
    }

    #[test]
    fn delete_requires_confirmation() {
        assert!(encode_delete(false).is_err());
        let (buf, module) = encode_delete(true).unwrap();
        assert_eq!(module, Modules::AccountManagement);
        assert!(matches!(
            decode_request(&buf),
            account_management::Message::DeleteAccountRequest(_)
        ));
    }

    #[test]
    fn restore_rejects_missing_or_non_file_archive() {
        assert!(encode_restore("./does-not-exist.qaul_export").is_err());
        assert!(encode_restore(".").is_err());
    }

    #[test]
    fn restore_archive_is_sent_as_absolute_path() {
        let (buf, _) = encode_restore("Cargo.toml").unwrap();
        match decode_request(&buf) {
            account_management::Message::RestoreAccountRequest(r) => {
                assert!(Path::new(&r.archive_path).is_absolute());
                assert!(r.archive_path.ends_with("Cargo.toml"));
            }
            other => panic!("unexpected message {:?}", other),
        }
    }

    #[test]
    fn error_response_is_an_err() {
        let payload = AccountManagement {
            message: Some(account_management::Message::Error(
                qaul_proto::qaul_common::RpcError {
                    code: 2,
                    message: "boom".to_string(),
                    details: String::new(),
                },
            )),
        }
        .encode_to_vec();
        assert!(decode_response(&payload, false).is_err());
    }
}
