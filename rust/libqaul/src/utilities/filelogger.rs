// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul File Logger
//!
//! Configurable file logger for libqaul, which can dynamically
//! enable and disable logging to file during runtime.

use crate::storage::configuration::Configuration;
use state::InitCell;
use std::fs::File;
use std::sync::RwLock;

extern crate log;

/// mutable state of file logger configuration
static FILELOGGERCONFIG: InitCell<RwLock<FileLoggerConfig>> = InitCell::new();

/// File Logger Configuration
pub struct FileLoggerConfig {
    pub enable: bool,
}

/// Logger that writes log messages to all the loggers it encapsulates.
pub struct FileLogger {
    logger: simplelog::WriteLogger<File>,
}

impl FileLogger {
    /// Create new file logger
    pub fn new(logger: simplelog::WriteLogger<File>) -> Self {
        let cfg = Configuration::get();
        let config = FileLoggerConfig {
            enable: cfg.debug.log,
        };
        FILELOGGERCONFIG.set(RwLock::new(config));
        FileLogger { logger }
    }

    /// Enable / disable file logger
    pub fn enable(enable: bool) {
        let mut config = FILELOGGERCONFIG.get().write().unwrap();
        config.enable = enable;
    }
}

impl log::Log for FileLogger {
    /// Check if file logger is enabled
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let config = FILELOGGERCONFIG.get().read().unwrap();
        config.enable && self.logger.enabled(metadata)
    }

    /// log to file logger
    fn log(&self, record: &log::Record) {
        let config = FILELOGGERCONFIG.get().read().unwrap();
        if config.enable {
            self.logger.log(record);
        }
    }

    /// flush logs to file
    fn flush(&self) {
        let config = FILELOGGERCONFIG.get().read().unwrap();
        if config.enable {
            self.logger.flush();
        }
    }
}
