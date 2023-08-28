// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # User Accounts Module Functions

use libqaul::node::user_accounts::UserAccount;
use state::Storage;
use std::sync::RwLock;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.user_accounts.rs");
}

/// mutable user account state
static USERACCOUNTS: Storage<RwLock<UserAccounts>> = Storage::new();
pub static BOT_USER_ACCOUNT_ID: Storage<String> = Storage::new();

/// user accounts module function handling
pub struct UserAccounts {
    my_user_account: Option<UserAccount>,
}

impl UserAccounts {
    /// Initialize User Accounts
    pub fn init(user_account: UserAccount) {
        // create user accounts state
        let user_accounts = UserAccounts {
            my_user_account: Some(user_account),
        };
        USERACCOUNTS.set(RwLock::new(user_accounts));
    }

    /// return user id
    pub fn get_user_id() -> Option<Vec<u8>> {
        // get state
        let user_accounts = USERACCOUNTS.get().read().unwrap();

        if let Some(my_user_account) = &user_accounts.my_user_account {
            return Some(my_user_account.id.to_bytes());
        }

        None
    }
}
