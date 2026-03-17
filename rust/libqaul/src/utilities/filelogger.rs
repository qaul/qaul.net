// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul File Logger
//!
//! Configurable file logger for libqaul, which can dynamically
//! enable and disable logging to file during runtime.

use std::fs::File;
use std::sync::{Arc, RwLock};

extern crate log;

/// File Logger Configuration
pub struct FileLoggerConfig {
    pub enable: bool,
}

impl FileLoggerConfig {
    /// Set enable/disable state (shared inner logic).
    fn set_enable(&mut self, enable: bool) {
        self.enable = enable;
    }
}

/// Instance-based file logger configuration state.
///
/// Holds an `Arc<RwLock<FileLoggerConfig>>` that is shared with the
/// `FileLogger` (which implements `log::Log`). When `enable()` is
/// called here, the `FileLogger` sees the change immediately because
/// both sides share the same Arc.
pub struct FileLoggerState {
    /// Shared file logger configuration handle.
    pub inner: Arc<RwLock<FileLoggerConfig>>,
}

impl FileLoggerState {
    /// Create a new FileLoggerState (logging disabled by default).
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(FileLoggerConfig { enable: false })),
        }
    }

    /// Create from an existing config value.
    pub fn from_config(enable: bool) -> Self {
        Self {
            inner: Arc::new(RwLock::new(FileLoggerConfig { enable })),
        }
    }

    /// Return a clone of the inner Arc so that `FileLogger` can share
    /// the same configuration at runtime.
    pub fn config_handle(&self) -> Arc<RwLock<FileLoggerConfig>> {
        Arc::clone(&self.inner)
    }

    /// Enable or disable file logging (instance method).
    pub fn enable(&self, enable: bool) {
        let mut config = self.inner.write().unwrap();
        config.set_enable(enable);
    }
}

/// Logger that writes log messages to all the loggers it encapsulates.
pub struct FileLogger {
    logger: simplelog::WriteLogger<File>,
    config: Arc<RwLock<FileLoggerConfig>>,
}

impl FileLogger {
    /// Create new file logger.
    ///
    /// `log_config` is a shared handle obtained from
    /// `FileLoggerState::config_handle()` so that runtime enable/disable
    /// toggles are visible to the `log::Log` implementation.
    pub fn new(logger: simplelog::WriteLogger<File>, log_config: Arc<RwLock<FileLoggerConfig>>) -> Self {
        FileLogger {
            logger,
            config: log_config,
        }
    }
}

impl log::Log for FileLogger {
    /// Check if file logger is enabled
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let config = self.config.read().unwrap();
        config.enable && self.logger.enabled(metadata)
    }

    /// log to file logger
    fn log(&self, record: &log::Record) {
        let config = self.config.read().unwrap();
        if config.enable {
            self.logger.log(record);
        }
    }

    /// flush logs to file
    fn flush(&self) {
        let config = self.config.read().unwrap();
        if config.enable {
            self.logger.flush();
        }
    }
}
