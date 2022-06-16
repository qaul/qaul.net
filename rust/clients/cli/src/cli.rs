// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Process CLI input
//! 
//! Analyze the strings and create RPC messages accordingly.

use super::node::Node;
use super::user_accounts::UserAccounts;
use super::connections::Connections;
use super::users::Users;
use super::router::Router;
use super::feed::Feed;
use super::chat::Chat;
use super::debug::Debug;
use super::ble::Ble;
use super::fileshare::FileShare;

/// CLI command analizer and processing
pub struct Cli {}

impl Cli {
    /// enter a program line to be processed
    pub fn process_command(command: String) {
        match command {
            // node functions
            cmd if cmd.starts_with("node ") => {
                Node::cli(cmd.strip_prefix("node ").unwrap());
            },
            // user accounts functions
            cmd if cmd.starts_with("account ") => {
                UserAccounts::cli(cmd.strip_prefix("account ").unwrap());
            },
            // users functions
            cmd if cmd.starts_with("users ") => {
                Users::cli(cmd.strip_prefix("users ").unwrap());
            },
            // router functions
            cmd if cmd.starts_with("router ") => {
                Router::cli(cmd.strip_prefix("router ").unwrap());
            },
            // feed functions
            cmd if cmd.starts_with("feed ") => {
                Feed::cli(cmd.strip_prefix("feed ").unwrap());
            },
            // chat functions
            cmd if cmd.starts_with("chat ") => {
                Chat::cli(cmd.strip_prefix("chat ").unwrap());
            },
            // connections functions
            cmd if cmd.starts_with("connections ") => {
                Connections::cli(cmd.strip_prefix("connections ").unwrap());
            },
            // ble functions
            cmd if cmd.starts_with("ble ") => {
                Ble::cli(cmd.strip_prefix("ble ").unwrap());
            },
            // debugging functions
            cmd if cmd.starts_with("debug ") => {
                Debug::cli(cmd.strip_prefix("debug ").unwrap());
            },
            // file sharing functions
            cmd if cmd.starts_with("file ") => {
                FileShare::cli(cmd.strip_prefix("file ").unwrap());
            },
            // unknown command
            _ => log::error!("unknown command"),
        }
    }
}
