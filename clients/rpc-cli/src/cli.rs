//! # Process CLI input
//! 
//! Analyze the strings and create RPC messages accordingly.

use super::node::Node;
use super::user_accounts::UserAccounts;
use super::router::Router;
use super::feed::Feed;

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
            // router functions
            cmd if cmd.starts_with("router ") => {
                Router::cli(cmd.strip_prefix("router ").unwrap());
            },
            // feed functions
            cmd if cmd.starts_with("feed ") => {
                Feed::cli(cmd.strip_prefix("feed ").unwrap());
            },
            // unknown command
            _ => log::error!("unknown command"),
        }
    }
}
