// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # bridge client for Matrix<-->Qaul Bridge
//!
//! bridge can be used to run on an embedded device, such as a raspberry Pi,
//! or as a static node on a server in the Internet.

use crate::relay_bot::MATRIX_CONFIG;
use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use futures_ticker::Ticker;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use libqaul;

mod chat;
mod chatfile;
mod configuration;
mod feed;
mod group;
mod relay_bot;
mod rpc;
mod user_accounts;
mod users;

use rpc::Rpc;

/// Events of the async loop
enum EventType {
    Rpc(bool),
}

#[async_std::main]
async fn main() {
    // get current working directory
    let path = std::env::current_dir().unwrap();
    let storage_path = path.as_path().to_str().unwrap().to_string();

    // start libqaul in new thread and save configuration file to current working path
    libqaul::api::start_with_config(storage_path, None);

    // wait until libqaul finished initializing
    while libqaul::api::initialization_finished() == false {
        // wait a little while
        std::thread::sleep(Duration::from_millis(10));
    }

    // Set user account
    // if no account, creating new account
    if libqaul::node::user_accounts::UserAccounts::len() == 0 {
        // TODO: the name of the user account should be configurable
        libqaul::node::user_accounts::UserAccounts::create("Qaul Matrix Bridge Bot".to_owned());
    }
    let default_user = libqaul::node::user_accounts::UserAccounts::get_default_user().unwrap();
    // initialize user account
    user_accounts::UserAccounts::init(default_user);
    println!("Matrix Bot has been initialized as a Qaul User");

    thread::spawn(|| {
        // connect the matrix bot with the qaul-cli
        match relay_bot::connect() {
            Ok(_) => {
                println!("Matrix-Bridge connecting");
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    });

    // check RPC once every 10 milliseconds
    let mut futures_ticker = Ticker::new(Duration::from_millis(10));
    // check arival of feed messages every 30 milliseconds
    let mut feed_ticker = Ticker::new(Duration::from_millis(30));
    // check arival of group messages every 30 milliseconds
    let mut group_ticker = Ticker::new(Duration::from_millis(30));
    // check for new users on the network every 50 milliseconds
    let mut user_ticker = Ticker::new(Duration::from_millis(50));
    // check for any invitations incoming for groups every 50 milliseconds
    let mut invited_ticker = Ticker::new(Duration::from_millis(50));

    // loop and poll CLI and RPC
    loop {
        let evt = {
            let rpc_fut = futures_ticker.next().fuse();
            let feed_fut = feed_ticker.next().fuse();
            let group_fut = group_ticker.next().fuse();
            let users_fut = user_ticker.next().fuse();
            let invited_fut = invited_ticker.next().fuse();

            pin_mut!(rpc_fut);
            pin_mut!(feed_fut);
            pin_mut!(group_fut);
            pin_mut!(users_fut);
            pin_mut!(invited_fut);

            select! {
               _rpc_ticker = rpc_fut => Some(EventType::Rpc(true)),
                _feed_ticker = feed_fut => {
                    if let Ok(config) = MATRIX_CONFIG.get().read() {
                        let last_index = &config.feed.last_index;
                        // Check unread messages from Libqaul
                        feed::Feed::request_feed_list(*last_index);
                    } else {
                        println!("Waiting for the configuration to Sync")
                    }
                    None
                },
                _group_ticker = group_fut => {
                    if let Ok(config) = MATRIX_CONFIG.get().read() {
                        group::Group::group_list();
                        let qaul_groups: Vec<Uuid> = config.room_map.keys().cloned().collect();

                        // Check unread messages from Libqaul groups
                        for group in qaul_groups {
                            let matrix_room = config.room_map.get(&group).unwrap();
                                let last_index_grp = matrix_room.last_index;
                            let group_id = group.as_bytes().to_vec();
                            chat::Chat::request_chat_conversation(group_id,last_index_grp);
                        }
                    } else {
                        println!("Waiting for the configuration to Sync")
                    }
                    None
                },
                _users_ticker = users_fut => {
                    users::Users::request_user_list("".to_string());
                    None
                },
                _invited_ticker = invited_fut => {
                    group::Group::group_invited();
                    None
                }
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Rpc(_) => match libqaul::api::receive_rpc() {
                    Ok(data) => {
                        Rpc::received_message(data);
                    }
                    _ => {}
                },
            }
        }
    }
}
