use state::Storage;
use std::{sync::RwLock};
use std::fs::File;
use crate::storage::configuration::Configuration;

extern crate log;

static FILELOGGERCONFIG: Storage<RwLock<FileLoggerConfig>> = Storage::new();
pub struct FileLoggerConfig{
    pub enable: bool,
}

/// Logger that writes log messages to all the loggers it encapsulates.
pub struct FileLogger {
    logger: simplelog::WriteLogger<File>,
}

impl FileLogger{ 
    pub fn new(logger: simplelog::WriteLogger<File>) -> Self {
        let cfg = Configuration::get();
        let config = FileLoggerConfig {
            enable: cfg.debug.log,
        };
        FILELOGGERCONFIG.set(RwLock::new(config));
        FileLogger { logger }
    }
    pub fn enable(enable: bool) {
        let mut config = FILELOGGERCONFIG.get().write().unwrap();
        config.enable = enable;
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let config = FILELOGGERCONFIG.get().read().unwrap();
        config.enable && self.logger.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        let config = FILELOGGERCONFIG.get().read().unwrap();
        if config.enable{
            self.logger.log(record);
        }        
    }
    fn flush(&self) {
        let config = FILELOGGERCONFIG.get().read().unwrap();
        if config.enable{
            self.logger.flush();
        }
    }
}
