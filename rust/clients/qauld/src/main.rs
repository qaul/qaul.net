// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld - qaul Daemon
//!
//! qaul daemon is running headless in the background.
//! It can be used to run on an embedded device, such as a raspberry Pi,
//! or as a static node on a server in the Internet.

use clap::Parser;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::{thread, time::Duration};

use libqaul;
mod socket;

/// qauld - qaul daemon : CLI Arguments
#[derive(Parser)]
struct CliArguments {
    /// User Name
    #[arg(short, long)]
    name: Option<String>,
    /// Port Number
    #[arg(short, long)]
    port: Option<u16>,
}

/// create a default user account for zero configuration Community Node startups
/// without providing a user name
pub fn create_default_named() -> String {
    let mut user_name: String;
    user_name = "Community Node ".to_string();
    user_name.push_str(
        libqaul::utilities::timestamp::Timestamp::get_timestamp()
            .to_string()
            .as_str(),
    );
    user_name
}

#[tokio::main]
async fn main() {
    // get current working directory
    let path = std::env::current_dir().unwrap();
    let storage_path = path.as_path().to_str().unwrap().to_string();

    // parse parameters and create default config
    let cli_arguments = CliArguments::parse();
    let mut def_config: BTreeMap<String, String> = BTreeMap::new();
    {
        if let Some(v) = cli_arguments.name.as_deref() {
            def_config.insert("name".to_string(), v.to_string());
        }
        if let Some(v) = cli_arguments.port {
            def_config.insert("port".to_string(), v.to_string());
        }
    }

    // start libqaul in new thread and save configuration file to current working path
    libqaul::api::start_with_config(storage_path.clone(), Some(def_config.clone()));

    // wait until libqaul finished initializing
    while libqaul::api::initialization_finished() == false {
        // wait a little while
        std::thread::sleep(Duration::from_millis(10));
    }

    // if no account, creating new accounts
    if libqaul::node::user_accounts::UserAccounts::len() == 0 {
        let user_name: String;
        if let Some(usr_name) = cli_arguments.name.as_deref() {
            user_name = usr_name.to_string();
        } else {
            user_name = create_default_named();
        }
        libqaul::node::user_accounts::UserAccounts::create(user_name.clone(), None);
    }

    // since we're now starting a socket server, the main thread now has work to do,
    // so, we can get rid of the loop
    if let Err(err) = socket::start_server(PathBuf::from(storage_path)).await {
        eprintln!("Err: {err}")
    }
}
