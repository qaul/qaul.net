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
pub mod radio_streamer;

#[cfg(feature = "rtc")]
pub mod rtc;

use std::sync::{Arc, Mutex};
use crate::node::user_accounts::UserAccounts;
use base64::{Engine as _, engine::general_purpose};

/// Services Module - holds all services state for a single instance
///
/// This struct wraps the services and provides instance-based access.
pub struct ServicesModule {
    /// Whether services have been initialized
    initialized: bool,
    /// Handle to the radio streamer task if running
    radio_handle: Arc<Mutex<Option<tokio::task::AbortHandle>>>,
}

impl ServicesModule {
    /// Create a new ServicesModule (instance-based)
    ///
    /// Note: This creates the instance but the actual service states
    /// are still initialized via global state for backward compatibility.
    pub fn new() -> Self {
        Self {
            initialized: false,
            radio_handle: Arc::new(Mutex::new(None)),
        }
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

    /// Start the radio streamer service
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the radio stream
    /// * `chunk_size` - The size of chunks to generate
    pub fn start_radio(&self, url: String, chunk_size: usize) {
        let mut handle_guard = self.radio_handle.lock().unwrap();

        // Stop existing radio if running
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }

        let task = tokio::spawn(async move {
            radio_streamer::RadioStreamer::process_stream(
                url,
                chunk_size,
                move |chunk, seq| {
                    async move {
                        // Get active user account
                        if let Some(user_account) = UserAccounts::get_default_user() {
                            // Encode chunk to base64
                            let encoded = general_purpose::STANDARD.encode(&chunk);
                            // Format: radio_chunk:<seq>:<base64>
                            let content = format!("radio_chunk:{}:{}", seq, encoded);

                            // Send via Feed service (using None for transports to use async flooding)
                            feed::Feed::send(&user_account, content, None, None);
                        } else {
                            log::warn!("No active user account found, cannot send radio chunk");
                        }
                    }
                }
            ).await;
        });

        *handle_guard = Some(task.abort_handle());
    }

    /// Stop the radio streamer service
    pub fn stop_radio(&self) {
        let mut handle_guard = self.radio_handle.lock().unwrap();
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }
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
