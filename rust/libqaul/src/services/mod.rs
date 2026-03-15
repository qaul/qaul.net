// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
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

#[cfg(feature = "rtc")]
pub mod rtc;

/// Instance-based services state owning all service sub-states.
/// Replaces the per-module global statics for multi-instance use.
pub struct ServicesState {
    /// Messaging state (scheduled messages + unconfirmed tracking).
    pub messaging: messaging::MessagingState,
    /// Feed state (feed messages + DB references).
    pub feed: feed::FeedState,
    /// Chat storage state (per-user chat DB references).
    pub chat: chat::storage::ChatState,
    /// Chat file state (per-user file history DB references).
    pub chat_files: chat::file::ChatFileState,
    /// Crypto session storage state (per-user crypto DB references).
    pub crypto: crypto::CryptoStorageState,
    /// Group storage state (per-user group DB references).
    pub groups: group::storage::GroupStorageState,
    /// DTN module state (DTN message tracking).
    pub dtn: dtn::DtnModuleState,
}

impl ServicesState {
    /// Create a new empty ServicesState with temporary in-memory databases.
    pub fn new() -> Self {
        Self {
            messaging: messaging::MessagingState::new(),
            feed: feed::FeedState::new(),
            chat: chat::storage::ChatState::new(),
            chat_files: chat::file::ChatFileState::new(),
            crypto: crypto::CryptoStorageState::new(),
            groups: group::storage::GroupStorageState::new(),
            dtn: dtn::DtnModuleState::new(),
        }
    }
}

/// Services Module - holds all services state for a single instance
///
/// This struct wraps the services and provides instance-based access.
pub struct ServicesModule {
    /// Whether services have been initialized
    initialized: bool,
}

impl ServicesModule {
    /// Create a new ServicesModule (instance-based)
    ///
    /// Note: This creates the instance but the actual service states
    /// are still initialized via global state for backward compatibility.
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// Initialize all services (instance method)
    ///
    /// This should be called after creating the instance to initialize
    /// all sub-services.
    pub fn initialize(&mut self) {
        if !self.initialized {
            crypto::Crypto::init();
            feed::Feed::init();
            messaging::Messaging::init();
            chat::Chat::init();
            group::Group::init();
            #[cfg(feature = "rtc")]
            rtc::Rtc::init();
            dtn::Dtn::init();
            self.initialized = true;
        }
    }

    /// Check if services are initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for ServicesModule {
    fn default() -> Self {
        Self::new()
    }
}

/// qaul Services (global state wrapper - for backward compatibility)
pub struct Services {}

impl Services {
    /// Initializes all qaul services
    ///
    /// This function needs to be called at startup
    ///
    /// Note: This uses global state. For new code, prefer using `ServicesModule`.
    pub fn init() {
        crypto::Crypto::init();
        feed::Feed::init();
        messaging::Messaging::init();
        chat::Chat::init();
        group::Group::init();
        dtn::Dtn::init();

        #[cfg(feature = "rtc")]
        rtc::Rtc::init();
    }
}
