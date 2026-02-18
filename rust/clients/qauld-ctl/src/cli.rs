// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.
//! CLI template for qauld-ctl

use clap::{Args, Parser, Subcommand};

/// qauld-ctl CLI: Control a running qauld daemon instance
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Explicit path to qauld sock. e.g /path/to/qauld.sock
    #[arg(short, long, env = "QAULD_SOCKET")]
    pub socket: Option<String>,
    /// Specify a directory to look for qauld.sock in
    #[arg(short, long)]
    pub dir: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// node details
    Node(NodeArgs),
    /// User Accounts
    Account(AccountArgs),
}

#[derive(Args, Debug)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub command: NodeSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum NodeSubcmd {
    /// prints the local node id
    Info,
}

#[derive(Args, Debug)]
pub struct AccountArgs {
    #[command(subcommand)]
    pub command: AccountSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum AccountSubcmd {
    /// get's and displays the default user account
    Default,
    /// create a new user account with the name {User Name}
    Create {
        /// Specify the username to create an account with
        #[arg(short, long)]
        username: String,
        /// Specify the password to create an account with
        #[arg(short, long)]
        password: Option<String>,
    },
    /// set or change password for the current user account (prompts for input)
    Password,
    /// login to an existing user account
    Login {
        /// Specify the username to create an account with
        #[arg(short, long)]
        username: String,
        /// Specify the password to create an account with
        #[arg(short, long)]
        password: String,
    },
    /// logout from the current user session
    Logout,
    /// check current authentication status (logged in/out, session info)
    Status,
}
