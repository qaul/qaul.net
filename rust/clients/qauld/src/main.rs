// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld - qaul Daemon
//!
//! qaul Daemon is running headless on servers in the background

use clap::{App, Arg};
use std::{thread, time::Duration};

use libqaul;

/// get command line arguments
pub fn get_argument(pattern: &str) -> Option<String> {
    let matches = App::new("")
        .arg(
            Arg::with_name("name")
                .short('n')
                .long("name")
                .takes_value(true)
                .help("user name"),
        )
        .arg(
            Arg::with_name("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .help("port number"),
        )
        .get_matches();

    if let Some(v) = matches.value_of(pattern) {
        Some(v.to_string())
    } else {
        None
    }
}

#[async_std::main]
async fn main() {
    // get current working directory
    let path = std::env::current_dir().unwrap();
    let storage_path = path.as_path().to_str().unwrap().to_string();

    // start libqaul in new thread and save configuration file to current working path
    libqaul::api::start(storage_path);

    // wait until libqaul finished initializing
    while libqaul::api::initialization_finished() == false {
        // wait a little while
        std::thread::sleep(Duration::from_millis(10));
    }

    // if no account, creating new accounts
    if libqaul::node::user_accounts::UserAccounts::len() == 0 {
        if let Some(user_name) = get_argument("name") {
            libqaul::node::user_accounts::UserAccounts::create(user_name.clone());
        } else {
            libqaul::node::user_accounts::UserAccounts::create_default_named();
        }
    }

    // loop
    loop {
        thread::sleep(Duration::from_millis(10));
    }
}
