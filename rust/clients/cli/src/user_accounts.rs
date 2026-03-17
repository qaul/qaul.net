// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # User Accounts Module Functions

use super::rpc::Rpc;
use prost::Message;
use serde::{Deserialize, Serialize};

/// protobuf RPC definition
use qaul_proto::qaul_rpc_user_accounts as proto;

/// default user initialization
pub enum MyUserAccountInitialiation {
    /// there was no request sent yet
    Uninitialized,
    /// no user account created yet
    NoDefaultAccount,
    /// user account is initialized
    Initialized,
}

/// Authentication State
pub struct AuthState {
    pub session_token: Option<String>,
    pub pending_auth: Option<PendingAuth>,
    pub pending_username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingAuth {
    pub username: String,
    pub password: String,
    pub salt: Option<String>,
    pub user_id: Option<Vec<u8>>,
}

/// user accounts module function handling
pub struct UserAccounts {
    initialiation: MyUserAccountInitialiation,
    my_user_account: Option<proto::MyUserAccount>,
    auth: AuthState,
}

impl UserAccounts {
    /// Initialize User Accounts
    pub fn init(state: &super::CliState) {
        // create user accounts state
        let user_accounts = UserAccounts {
            initialiation: MyUserAccountInitialiation::Uninitialized,
            my_user_account: None,
            auth: AuthState {
                session_token: None,
                pending_auth: None,
                pending_username: None,
            },
        };
        *state.user_accounts.write().unwrap() = Some(user_accounts);

        // request default user
        Self::request_default_account(state);
        // check for existing session
        super::authentication::Auth::restore_session(state);
    }

    /// return user id
    pub fn get_user_id(state: &super::CliState) -> Option<Vec<u8>> {
        let guard = state.user_accounts.read().unwrap();
        let user_accounts = guard.as_ref().expect("UserAccounts not initialized");

        if let Some(my_user_account) = &user_accounts.my_user_account {
            return Some(my_user_account.id.clone());
        }

        None
    }

    /// CLI command interpretation
    ///
    /// The CLI commands of user accounts module are processed here
    pub fn cli(state: &super::CliState, command: &str) {
        match command {
            // request default user account
            cmd if cmd.starts_with("default") => {
                Self::request_default_account(state);
            }
            // create new user account
            cmd if cmd.starts_with("create ") => {
                Self::create_user_account(state, cmd.strip_prefix("create ").unwrap().to_string());
            }
            // set/reset password for an existing account
            cmd if cmd.starts_with("password") => {
                Self::handle_password_change(state);
            }
            // login command
            cmd if cmd.starts_with("login ") => {
                Self::handle_login(state, cmd.strip_prefix("login ").unwrap().to_string());
            }
            // logout command
            "logout" => {
                Self::handle_logout(state);
            }
            // check authentication status
            "status" => {
                Self::check_auth_status(state);
            }
            // unknown command
            _ => log::error!("unknown account command"),
        }
    }

    /// Create new user account
    fn create_user_account(state: &super::CliState, args: String) {
        // logout any existing session before creating new account
        if let Some(_session) = super::authentication::Auth::get_session_info(state) {
            super::authentication::Auth::clear_session(state);
        }

        let (username, password) = Self::parse_create_args(&args);
        // create info request message
        let proto_message = proto::UserAccounts {
            message: Some(proto::user_accounts::Message::CreateUserAccount(
                proto::CreateUserAccount {
                    name: username,
                    password,
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            state,
            buf,
            super::rpc::proto::Modules::Useraccounts.into(),
            "".to_string(),
        );
    }

    /// parse 'create' command arguments to extract username and optional password
    /// supports these variations: "name", "name -p password", "name -p", "name --password password"
    fn parse_create_args(args_str: &str) -> (String, Option<String>) {
        // find password flag position
        let flag_pos = args_str
            .find(" -p")
            .or_else(|| args_str.find(" --password"));

        match flag_pos {
            Some(pos) => {
                // extract username
                let username = args_str[..pos].to_string();
                let after_flag = &args_str[pos..];
                let mut parts = after_flag.split_whitespace().skip(1); // skip the flag itself

                match parts.next() {
                    // direct password provided
                    Some(password) => (username, Some(password.to_string())),
                    // if there's no password after flag, prompt the user
                    None => (username, Self::prompt_password()),
                }
            }
            None => (args_str.to_string(), None),
        }
    }

    /// prompt user for password input, returns None for empty input
    fn prompt_password() -> Option<String> {
        use std::io::{self, Write};

        print!("Password: ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let password = input.trim().to_string();
                // return Some only if password is not empty
                (!password.is_empty()).then(|| password)
            }
            Err(_) => None,
        }
    }

    /// handle the password change for current user
    fn handle_password_change(state: &super::CliState) {
        // check if user is logged in
        if super::authentication::Auth::get_session_info(state).is_none() {
            println!("You are not logged in, please log into a user account first.");
            return;
        }

        // prompt for new password (or empty to remove)
        let password = Self::prompt_password();
        // create password change request
        let proto_message = proto::UserAccounts {
            message: Some(proto::user_accounts::Message::SetPasswordRequest(
                proto::SetPasswordRequest { password },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).unwrap();
        // send message
        Rpc::send_message(
            state,
            buf,
            super::rpc::proto::Modules::Useraccounts.into(),
            "".to_string(),
        );
    }

    /// Request default user account
    fn request_default_account(state: &super::CliState) {
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
            state,
            buf,
            super::rpc::proto::Modules::Useraccounts.into(),
            "".to_string(),
        );
    }

    // Handle login command
    fn handle_login(state: &super::CliState, args: String) {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.is_empty() {
            println!("Usage: account login <username> -p <password>");
            return;
        }

        let username = parts[0].to_string();
        let mut password = None;

        for i in 1..parts.len() {
            if (parts[i] == "-p" || parts[i] == "--password") && i + 1 < parts.len() {
                password = Some(parts[i + 1].to_string());
                break;
            }
        }

        if password.is_none() && parts.iter().any(|&p| p == "-p" || p == "--password") {
            password = Self::prompt_password();
        }

        println!("Authenticating user: {}", username);

        if let Some(pwd) = password {
            Self::set_pending_auth(state, username.clone(), pwd);
        } else {
            // for passwordless logins
            Self::set_pending_auth(state, username.clone(), String::new());
        }

        super::authentication::Auth::initiate_login(state, username);
    }

    fn handle_logout(state: &super::CliState) {
        if let Some(ref account) = Self::get_my_account(state) {
            super::authentication::Auth::logout(state, account.id.clone());
            println!("Logging out...");
        } else {
            println!("Not logged in");
        }
    }

    fn check_auth_status(state: &super::CliState) {
        if let Some(session) = super::authentication::Auth::get_session_info(state) {
            println!("Authentication Status: Logged In");
            println!("  User: {}", session.username);
            println!("  Session created: {}", session.created_at);
        } else {
            println!("Authentication Status: Not Logged In");
        }
    }

    fn get_my_account(state: &super::CliState) -> Option<proto::MyUserAccount> {
        let guard = state.user_accounts.read().unwrap();
        let user_accounts = guard.as_ref().expect("UserAccounts not initialized");
        user_accounts.my_user_account.clone()
    }

    /// Store pending auth info (called by auth module)
    pub fn set_pending_auth_salt(state: &super::CliState, salt: String) {
        let mut guard = state.user_accounts.write().unwrap();
        let user_accounts = guard.as_mut().expect("UserAccounts not initialized");
        if let Some(ref mut pending) = user_accounts.auth.pending_auth {
            pending.salt = Some(salt);
        }
    }

    /// Get pending auth info
    pub fn get_pending_auth(state: &super::CliState) -> Option<PendingAuth> {
        let guard = state.user_accounts.read().unwrap();
        let user_accounts = guard.as_ref().expect("UserAccounts not initialized");
        user_accounts.auth.pending_auth.clone()
    }

    pub fn set_pending_auth(state: &super::CliState, username: String, password: String) {
        let mut guard = state.user_accounts.write().unwrap();
        let user_accounts = guard.as_mut().expect("UserAccounts not initialized");
        user_accounts.auth.pending_auth = Some(PendingAuth {
            username,
            password,
            salt: None,
            user_id: None,
        });
    }

    pub fn set_pending_user_id(state: &super::CliState, user_id: Vec<u8>) {
        let mut guard = state.user_accounts.write().unwrap();
        let user_accounts = guard.as_mut().expect("UserAccounts not initialized");
        if let Some(ref mut pending) = user_accounts.auth.pending_auth {
            pending.user_id = Some(user_id);
        }
    }

    pub fn clear_pending_auth(state: &super::CliState) {
        let mut guard = state.user_accounts.write().unwrap();
        let user_accounts = guard.as_mut().expect("UserAccounts not initialized");
        user_accounts.auth.pending_auth = None;
        user_accounts.auth.pending_username = None;
    }

    // Username management
    pub fn set_pending_username(state: &super::CliState, username: String) {
        let mut guard = state.user_accounts.write().unwrap();
        let user_accounts = guard.as_mut().expect("UserAccounts not initialized");
        user_accounts.auth.pending_username = Some(username);
    }

    pub fn get_pending_username(state: &super::CliState) -> Option<String> {
        let guard = state.user_accounts.read().unwrap();
        let user_accounts = guard.as_ref().expect("UserAccounts not initialized");
        user_accounts.auth.pending_username.clone()
    }

    // Session management
    pub fn set_session_token(state: &super::CliState, token: Option<String>) {
        let mut guard = state.user_accounts.write().unwrap();
        let user_accounts = guard.as_mut().expect("UserAccounts not initialized");
        user_accounts.auth.session_token = token;
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the user accounts module.
    pub fn rpc(state: &super::CliState, data: Vec<u8>) {
        match proto::UserAccounts::decode(&data[..]) {
            Ok(user_accounts) => {
                match user_accounts.message {
                    Some(proto::user_accounts::Message::DefaultUserAccount(
                        proto_defaultuseraccount,
                    )) => {
                        // get state
                        let mut state_guard = state.user_accounts.write().unwrap();
                        let ua_state = state_guard.as_mut().expect("UserAccounts not initialized");

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

                                // display password status
                                if my_user_account.has_password {
                                    println!("Your password is enabled");
                                } else {
                                    println!("Your password is disabled");
                                }

                                // save it to state
                                ua_state.my_user_account = Some(my_user_account);
                                ua_state.initialiation =
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
                            ua_state.initialiation =
                                MyUserAccountInitialiation::NoDefaultAccount;
                        }
                    }
                    Some(proto::user_accounts::Message::MyUserAccount(proto_myuseraccount)) => {
                        // get state
                        let mut state_guard = state.user_accounts.write().unwrap();
                        let ua_state = state_guard.as_mut().expect("UserAccounts not initialized");

                        // print received user
                        println!("New user account created:");
                        println!(
                            "{}, ID[{}]",
                            proto_myuseraccount.name, proto_myuseraccount.id_base58
                        );
                        println!("    public key: {}", proto_myuseraccount.key_base58);

                        // save it to state
                        ua_state.my_user_account = Some(proto_myuseraccount);
                        ua_state.initialiation = MyUserAccountInitialiation::Initialized;
                    }
                    // handle password change response
                    Some(proto::user_accounts::Message::SetPasswordResponse(response)) => {
                        if response.success {
                            println!(" Password updated");
                        } else {
                            println!("{}", response.error_message);
                        }
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
