//! # Process CLI input
//! 
//! Analyze the strings and create RPC messages accordingly.

use super::node::Node;

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
            // unknown command
            _ => log::error!("unknown command"),
        }
    }
}
