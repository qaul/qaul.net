// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! qaul Services
//!
//! Contains all service module of qaul
//!
//! At the moment there are the following services supported
//!
//! * Feed messenger: sends public messages to all users in the network.
//! * Messaging: sends and receives direct messages in qaul network.
//!   It can be accessed by other services.
//! * Chat: Sends and receives direct chat messages via messaging.

pub mod chat;
pub mod crypto;
pub mod dtn;
pub mod feed;
pub mod group;
pub mod messaging;
pub mod rtc;

/// qaul Services
pub struct Services {}

impl Services {
    /// Initializes all qaul services
    ///
    /// This function needs to be called at startup
    pub fn init() {
        crypto::Crypto::init();
        feed::Feed::init();
        messaging::Messaging::init();
        chat::Chat::init();
        group::Group::init();
        rtc::Rtc::init();
        dtn::Dtn::init();
    }
}
