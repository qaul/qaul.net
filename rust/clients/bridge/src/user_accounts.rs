// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # User Accounts Module Functions

use super::rpc::Rpc;
use prost::Message;
use state::Storage;
use std::sync::RwLock;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.user_accounts.rs");
}

/// mutable user account state
static USERACCOUNTS: Storage<RwLock<UserAccounts>> = Storage::new();
pub static BOT_USER_ACCOUNT_ID: Storage<String> = Storage::new();

/// default user initialization
pub enum MyUserAccountInitialiation {
    /// there was no request sent yet
    Uninitialized,
    /// no user account created yet
    NoDefaultAccount,
    /// user account is initialized
    Initialized,
}

/// user accounts module function handling
pub struct UserAccounts {
    initialiation: MyUserAccountInitialiation,
    my_user_account: Option<proto::MyUserAccount>,
}

impl UserAccounts {
    /// Initialize User Accounts
    pub fn init() {
        // create user accounts state
        let user_accounts = UserAccounts {
            initialiation: MyUserAccountInitialiation::Uninitialized,
            my_user_account: None,
        };
        USERACCOUNTS.set(RwLock::new(user_accounts));

        // request default user
        Self::request_default_account();
    }

    /// return user id
    pub fn get_user_id() -> Option<Vec<u8>> {
        // get state
        let user_accounts = USERACCOUNTS.get().read().unwrap();

        if let Some(my_user_account) = &user_accounts.my_user_account {
            return Some(my_user_account.id.clone());
        }

        None
    }

    /// Request default user account
    fn request_default_account() {
        // create info request message
        let proto_message = proto::UserAccounts {
            message: Some(proto::user_accounts::Message::GetDefaultUserAccount(true)),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Useraccounts.into(),
            "".to_string(),
        );
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the user accounts module.
    pub fn rpc(data: Vec<u8>) {
        match proto::UserAccounts::decode(&data[..]) {
            Ok(user_accounts) => {
                match user_accounts.message {
                    Some(proto::user_accounts::Message::DefaultUserAccount(
                        proto_defaultuseraccount,
                    )) => {
                        // get state
                        let mut user_accounts = USERACCOUNTS.get().write().unwrap();

                        // check if default user is set
                        if proto_defaultuseraccount.user_account_exists {
                            if let Some(my_user_account) = proto_defaultuseraccount.my_user_account
                            {
                                // print user account
                                println!("Your user account is:");
                                println!(
                                    "{}, ID[{}]",
                                    my_user_account.name, my_user_account.id_base58
                                );
                                println!("    public key: {}", my_user_account.key_base58);
                                BOT_USER_ACCOUNT_ID.set(my_user_account.clone().id_base58);
                                // save it to state
                                user_accounts.my_user_account = Some(my_user_account);
                                user_accounts.initialiation =
                                    MyUserAccountInitialiation::Initialized;
                            } else {
                                log::error!("unexpected message configuration");
                            }
                        } else {
                            // print message to create a new user account
                            println!("No user account created yet");
                            println!("Please create a user account:");
                            println!("");
                            println!("    account create {{Your User Name}}");
                            println!("");

                            // save it to state
                            user_accounts.initialiation =
                                MyUserAccountInitialiation::NoDefaultAccount;
                        }
                    }
                    Some(proto::user_accounts::Message::MyUserAccount(proto_myuseraccount)) => {
                        // get state
                        let mut user_accounts = USERACCOUNTS.get().write().unwrap();

                        // print received user
                        println!("New user account created:");
                        println!(
                            "{}, ID[{}]",
                            proto_myuseraccount.name, proto_myuseraccount.id_base58
                        );
                        println!("    public key: {}", proto_myuseraccount.key_base58);

                        // save it to state
                        user_accounts.my_user_account = Some(proto_myuseraccount);
                        user_accounts.initialiation = MyUserAccountInitialiation::Initialized;
                    }
                    _ => {
                        log::error!("unprocessable RPC user accounts message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
