// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Process CLI input
//!
//! Analyze the strings and create RPC messages accordingly.

use super::ble::Ble;
use super::chat::Chat;
use super::chatfile::ChatFile;
use super::connections::Connections;
use super::crypto::Crypto;
use super::debug::Debug;
use super::dtn::Dtn;
use super::feed::Feed;
use super::group::Group;
use super::node::Node;
use super::router::Router;
use super::rtc::Rtc;
use super::user_accounts::UserAccounts;
use super::users::Users;

/// CLI command analizer and processing
pub struct Cli {}

impl Cli {
    /// enter a program line to be processed
    pub fn process_command(state: &super::CliState, command: String) {
        match command {
            // node functions
            cmd if cmd.starts_with("node ") => {
                Node::cli(state, cmd.strip_prefix("node ").unwrap());
            }
            // user accounts functions
            cmd if cmd.starts_with("account ") => {
                UserAccounts::cli(state, cmd.strip_prefix("account ").unwrap());
            }
            // users functions
            cmd if cmd.starts_with("users ") => {
                Users::cli(state, cmd.strip_prefix("users ").unwrap());
            }
            // router functions
            cmd if cmd.starts_with("router ") => {
                Router::cli(state, cmd.strip_prefix("router ").unwrap());
            }
            // feed functions
            cmd if cmd.starts_with("feed ") => {
                Feed::cli(state, cmd.strip_prefix("feed ").unwrap());
            }
            // chat functions
            cmd if cmd.starts_with("chat ") => {
                Chat::cli(state, cmd.strip_prefix("chat ").unwrap());
            }
            // connections functions
            cmd if cmd.starts_with("connections ") => {
                Connections::cli(state, cmd.strip_prefix("connections ").unwrap());
            }
            // ble functions
            cmd if cmd.starts_with("ble ") => {
                Ble::cli(state, cmd.strip_prefix("ble ").unwrap());
            }
            // debugging functions
            cmd if cmd.starts_with("debug ") => {
                Debug::cli(state, cmd.strip_prefix("debug ").unwrap());
            }
            // file sharing functions
            cmd if cmd.starts_with("file ") => {
                ChatFile::cli(state, cmd.strip_prefix("file ").unwrap());
            }
            // group functions
            cmd if cmd.starts_with("group ") => {
                Group::cli(state, cmd.strip_prefix("group ").unwrap());
            }
            // rtc functions
            cmd if cmd.starts_with("rtc ") => {
                Rtc::cli(state, cmd.strip_prefix("rtc ").unwrap());
            }
            // dtn functions
            cmd if cmd.starts_with("dtn ") => {
                Dtn::cli(state, cmd.strip_prefix("dtn ").unwrap());
            }
            // crypto functions (session rotation config)
            cmd if cmd.starts_with("crypto ") => {
                Crypto::cli(cmd.strip_prefix("crypto ").unwrap());
            }
            // unknown command
            _ => log::error!("unknown command"),
        }
    }
}
