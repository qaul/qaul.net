#[macro_use]
extern crate log;
extern crate simplelog;

mod ble;
mod rpc;


use filetime::FileTime;
use rpc::{msg_loop::listen_for_sys_msgs};
use simplelog::*;
use std::{
    collections::BTreeMap,
    fs::File,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::ble::ble_service::IdleBleService;

/// initialize and start the ble_module
///
#[async_std::main]
async fn main() {
    // --- initialize logger ---
    // prepare logger path
    // the path of the log file follows libqaul's naming convention
    let log_path = std::env::current_dir().unwrap().as_path().join("logs");

    // create log directory if missing
    std::fs::create_dir_all(&log_path).unwrap();

    // find rust env var
    let mut env_log_level = String::from("error");
    for (key, value) in std::env::vars() {
        if key == "RUST_LOG" {
            env_log_level = value;
            break;
        }
    }

    // define log level
    let mut level_filter = log::LevelFilter::Error;
    if env_log_level == "warn" {
        level_filter = log::LevelFilter::Warn;
    } else if env_log_level == "debug" {
        level_filter = log::LevelFilter::Debug;
    } else if env_log_level == "info" {
        level_filter = log::LevelFilter::Info;
    } else if env_log_level == "trace" {
        level_filter = log::LevelFilter::Trace;
    }

    // create log file name
    let log_file_name: String = format!(
        "{}_{}.log",
        env_log_level,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let log_file_path = log_path.join(log_file_name);

    // maintain log files
    let paths = std::fs::read_dir(log_path).unwrap();

    let mut logfiles: BTreeMap<i64, String> = BTreeMap::new();
    let mut logfile_times: Vec<i64> = vec![];
    for path in paths {
        let filename = String::from(path.as_ref().unwrap().path().to_str().unwrap());
        let metadata = std::fs::metadata(filename.clone()).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        logfile_times.push(mtime.seconds());
        logfiles.insert(mtime.seconds(), filename);
    }
    logfile_times.sort();

    if logfile_times.len() > 2 {
        for i in 0..(logfile_times.len() - 2) {
            if let Some(filename) = logfiles.get(&logfile_times[i]) {
                std::fs::remove_file(std::path::Path::new(filename)).unwrap();
            }
        }
    }

    CombinedLogger::init(vec![
        SimpleLogger::new(level_filter, Config::default()),
        WriteLogger::new(
            level_filter,
            Config::default(),
            File::create(log_file_path).unwrap(),
        ),
    ])
    .unwrap();

    let rpc_receiver = rpc::init();
    let ble_service = IdleBleService::new().await.unwrap_or_else(|err| {
        error!("{:#?}", err);
        std::process::exit(1);
    });

    listen_for_sys_msgs(rpc_receiver, ble_service).await.unwrap_or_else(|err| {
        error!("{:#?}", err);
        std::process::exit(1);
    });
}
