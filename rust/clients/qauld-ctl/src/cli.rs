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
