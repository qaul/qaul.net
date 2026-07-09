// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Account-management & auth RPC service-dispatch tests
//!
//! Drives the generated `dispatch()` functions end-to-end against a live
//! `QaulState`, exercising:
//! - account management: export -> delete -> restore round-trip,
//! - auth: session-status ("active session") and logout,
//! - the uniform error channel (malformed request -> `error` variant).
//!
//! These cover the RPC surface added on top of the existing CLI-only
//! account-management / authentication logic.

use std::path::Path;
use std::sync::Arc;

use prost::Message;
use tempfile::TempDir;

use libqaul::node::account_management::{proto as am, AccountManagement};
use libqaul::node::user_accounts::UserAccounts;
use libqaul::rpc::authentication::{proto as auth, Authentication};
use libqaul::storage::database::DataBase;
use libqaul::storage::Storage;
use libqaul::{Libqaul, QaulState, RequestContext};

/// Start a fresh libqaul instance in a temp dir (no event loop).
fn start() -> (Arc<Libqaul>, TempDir) {
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().to_str().unwrap().to_string();
    let instance = futures::executor::block_on(libqaul::start_instance(path, None));
    (instance, dir)
}

/// Encode an account-management request, run it through the generated
/// dispatcher, and return the decoded response oneof variant.
fn am_dispatch(
    state: &QaulState,
    caller: Vec<u8>,
    msg: am::account_management::Message,
) -> am::account_management::Message {
    let ctx = RequestContext {
        state,
        user_id: caller,
        request_id: String::new(),
    };
    let env = am::AccountManagement { message: Some(msg) };
    let out = am::dispatch::<RequestContext, AccountManagement>(&ctx, env.encode_to_vec());
    am::AccountManagement::decode(&out[..])
        .expect("decode account_management reply")
        .message
        .expect("account_management reply variant")
}

/// Same, for the auth service. `caller` populates the RequestContext's
/// `user_id` (the outer-envelope identity) — used by the self-scoped
/// `logout` / `session_status` ops.
fn auth_dispatch(
    state: &QaulState,
    caller: Vec<u8>,
    msg: auth::auth_rpc::Message,
) -> auth::auth_rpc::Message {
    let ctx = RequestContext {
        state,
        user_id: caller,
        request_id: String::new(),
    };
    let env = auth::AuthRpc { message: Some(msg) };
    let out = auth::dispatch::<RequestContext, Authentication>(&ctx, env.encode_to_vec());
    auth::AuthRpc::decode(&out[..])
        .expect("decode auth reply")
        .message
        .expect("auth reply variant")
}

#[test]
fn account_and_auth_rpc_dispatch_roundtrips() {
    let (instance, _dir) = start();
    let state = &*instance.state;

    // Create an account and force its on-disk DB/dir to exist so the export
    // archive carries a real user directory.
    let account = UserAccounts::create(state, "alice".to_string(), None);
    let id = account.id;
    let _ = DataBase::get_user_db(state, id);

    // --- export ---
    let reply = am_dispatch(
        state,
        id.to_bytes(), // caller identity (RequestContext) — export is self-scoped
        am::account_management::Message::ExportAccountRequest(am::ExportAccountRequest {
            output_path: String::new(), // default = storage root
        }),
    );
    let archive_path = match reply {
        am::account_management::Message::ExportAccountResponse(r) => r.path,
        _ => panic!("expected ExportAccountResponse"),
    };
    assert!(
        Path::new(&archive_path).exists(),
        "export archive should exist at {archive_path}"
    );

    // --- auth: session-status (creating an account signs you in) ---
    let status = auth_dispatch(
        state,
        id.to_bytes(), // caller identity (RequestContext)
        auth::auth_rpc::Message::SessionStatusRequest(auth::SessionStatusRequest {}),
    );
    match status {
        auth::auth_rpc::Message::SessionStatusResponse(r) => {
            assert!(r.authenticated, "a freshly created account is authenticated")
        }
        _ => panic!("expected SessionStatusResponse"),
    }

    // --- auth: logout ---
    let logout = auth_dispatch(
        state,
        id.to_bytes(), // caller identity (RequestContext)
        auth::auth_rpc::Message::LogoutRequest(auth::LogoutRequest {}),
    );
    assert!(
        matches!(logout, auth::auth_rpc::Message::Ack(_)),
        "logout should acknowledge"
    );

    // --- auth: session-status (logout ended the session) ---
    let status = auth_dispatch(
        state,
        id.to_bytes(), // caller identity (RequestContext)
        auth::auth_rpc::Message::SessionStatusRequest(auth::SessionStatusRequest {}),
    );
    match status {
        auth::auth_rpc::Message::SessionStatusResponse(r) => {
            assert!(!r.authenticated, "logout should end the session")
        }
        _ => panic!("expected SessionStatusResponse"),
    }

    // --- delete ---
    let del = am_dispatch(
        state,
        id.to_bytes(), // caller identity (RequestContext) — delete is self-scoped
        am::account_management::Message::DeleteAccountRequest(am::DeleteAccountRequest {}),
    );
    assert!(
        matches!(del, am::account_management::Message::Ack(_)),
        "delete should acknowledge"
    );
    assert!(
        UserAccounts::get_by_id(state, id).is_none(),
        "account should be gone after delete"
    );

    // --- restore from the exported archive ---
    let restored = am_dispatch(
        state,
        Vec::new(), // restore mints a new account; no caller identity needed
        am::account_management::Message::RestoreAccountRequest(am::RestoreAccountRequest {
            archive_path: archive_path.clone(),
        }),
    );
    match restored {
        am::account_management::Message::RestoreAccountResponse(r) => {
            assert_eq!(r.user_id_base58, id.to_base58());
            assert_eq!(r.user_id, id.to_bytes());
        }
        _ => panic!("expected RestoreAccountResponse"),
    }
    assert!(
        UserAccounts::get_by_id(state, id).is_some(),
        "account should be back after restore"
    );

    // --- regression: restore must reclaim an orphan directory ---
    //
    // Reproduces the delete/lazy-reopen soft-lock: `delete_account` removes the
    // user directory, but a background storage access (any `get_user_db`
    // caller) then re-creates an empty `user.db` for the just-deleted account.
    // The account is gone from config/in-memory state (no login option), yet
    // the orphan directory previously made restore hard-fail with "User
    // directory already exists", permanently blocking re-import.
    {
        // Delete again (restore above re-added the account).
        let del = am_dispatch(
            state,
            id.to_bytes(),
            am::account_management::Message::DeleteAccountRequest(am::DeleteAccountRequest {}),
        );
        assert!(matches!(del, am::account_management::Message::Ack(_)));
        assert!(UserAccounts::get_by_id(state, id).is_none());

        // Simulate the race via the real mechanism: a lazy DB open re-creates
        // the directory the delete had removed. Then release the handle, as an
        // app restart would (nothing holds the sled lock at restore time).
        let _ = DataBase::get_user_db(state, id);
        DataBase::close_user_db(state, id);
        let orphan_dir = Storage::get_account_path(state, id);
        assert!(
            orphan_dir.exists(),
            "orphan directory should exist to reproduce the soft-lock"
        );

        // Restore must reclaim the orphan and succeed, not error out.
        let restored = am_dispatch(
            state,
            Vec::new(),
            am::account_management::Message::RestoreAccountRequest(am::RestoreAccountRequest {
                archive_path: archive_path.clone(),
            }),
        );
        match restored {
            am::account_management::Message::RestoreAccountResponse(r) => {
                assert_eq!(r.user_id_base58, id.to_base58());
            }
            other => panic!("orphan directory soft-locked restore: got {other:?}"),
        }
        assert!(
            UserAccounts::get_by_id(state, id).is_some(),
            "account should be back after reclaiming the orphan and restoring"
        );
    }

    // --- uniform error channel: malformed caller identity -> error variant ---
    let err = am_dispatch(
        state,
        vec![0xff, 0x00, 0x01], // not a valid PeerId
        am::account_management::Message::DeleteAccountRequest(am::DeleteAccountRequest {}),
    );
    match err {
        am::account_management::Message::Error(e) => {
            assert!(!e.message.is_empty(), "error should carry a message")
        }
        _ => panic!("expected Error variant for malformed request"),
    }
}
