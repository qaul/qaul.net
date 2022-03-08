// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Logger
use log::{info, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};

static LOGGER_HANDLE: state::Storage<Handle> = state::Storage::new();
const LOG_CONFIG_FILE_NAME: &str = "log4rs.yaml";
const DEFAULT_LOG_FILE_FORMAT: &str = "{m}\n";
const LOG_FILE_TAG: &str = "log_file";
const ROLL_PATTERN: &str = "./logs/data.log.{}.gz";
const LOG_FILE_PATH: &str = "./logs/data.log";
const LOG_FILE_SIZE_IN_B: u64 = 1024 * 1024 * 1024 * 10; //10Mb

//TODO
//
//due to lack of examples, there may be code here which will overwritte the config values from "log4rs.yaml", 
//and vise versa;
//find out how to use rust code and "log4rs.yaml" together
//

/// Initialize logger
/// may be called it only once, otherwise there may be an exception
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file(LOG_CONFIG_FILE_NAME, Default::default()).unwrap();
    info!("[logger]: (re)booting up...");
    configure(false, None)
}

/// (re)configure the logger
///
/// save state locally
/// @log_into_file whether use logging into a file or to stdout/stderr
pub fn configure(
    log_into_file: bool,
    level: Option<LevelFilter>,
) -> Result<(), Box<dyn std::error::Error>> {
    let level2 = level.unwrap_or(LevelFilter::Debug);
    let config = if log_into_file {
        let fixed_window_roller = FixedWindowRoller::builder().build(ROLL_PATTERN, 100)?;
        let size_trigger = SizeTrigger::new(LOG_FILE_SIZE_IN_B);
        let compound_policy =
            CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));
        let encoder = PatternEncoder::new(DEFAULT_LOG_FILE_FORMAT);
        let appender = RollingFileAppender::builder()
            .encoder(Box::new(encoder))
            .append(true)
            .build(LOG_FILE_PATH, Box::new(compound_policy))?;

        let file = Appender::builder().build(LOG_FILE_TAG, Box::new(appender));
        let root = Root::builder().appender(LOG_FILE_TAG).build(level2);
        Config::builder().appender(file).build(root)
    } else {
        let stdout = ConsoleAppender::builder().target(Target::Stdout).build();
        let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

        //TODO - how to merge "stdout" and "stderr" into one?
        //or, will they get merged automatically somehow?
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("stderr", Box::new(stderr)))
            .build(Root::builder().appender("stdout").build(level2))
    }
    .unwrap();

    if LOGGER_HANDLE.try_get().is_none() {
        let hdl = log4rs::init_config(config)?;
        LOGGER_HANDLE.set(hdl);
    } else {
        LOGGER_HANDLE.get().set_config(config);
    }

    info!("[logger]: (re)booting up finished");
    Ok(())
}
