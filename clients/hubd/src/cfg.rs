use clap::{App, Arg};
use std::{env, path::PathBuf};

/// The hub configuration
pub(crate) struct Config {
    /// Path to initial peer set
    pub(crate) peers: PathBuf,
    /// Runtime mode (in netmod-tcp)
    pub(crate) mode: String,
    /// What interface to bind on
    pub(crate) addr: String,
    /// What port to bind on
    pub(crate) port: u16,
}

pub(crate) fn cli<'a>() -> App<'a, 'a> {
    App::new("hubd")
        .arg(
            Arg::with_name("PEERS_PATH")
                .short("p")
                .long("peers")
                .help("The path to initial peers"),
        )
        .arg(
            Arg::with_name("RUN_MODE")
                .short("m")
                .long("mode")
                .help("The hub's run mode"),
        )
        .arg(
            Arg::with_name("SOCKET_ADDR")
                .short("a")
                .long("addr")
                .help("The hub's bound socket address"),
        )
        .arg(
            Arg::with_name("SOCKET_PORT")
                .short("p")
                .long("port")
                .help("The hub's bound socket port"),
        )
}

pub(crate) fn match_fold<'a>(app: App<'a, 'a>) -> Config {
    let m = app.get_matches();

    Config {
        peers: {
            let p = m
                .value_of("PEERS_PATH")
                .map(|s| s.to_owned())
                .or(env::var("QAUL_HUBD_PEERS").ok())
                .expect("No initial peer set provided!");
            let mut buf = PathBuf::new();
            buf.push(p);
            buf
        },
        mode: m
            .value_of("RUN_MODE")
            .map(|s| s.to_owned())
            .or(env::var("QAUL_HUBD_MODE").ok())
            .unwrap_or("static".into()),
        addr: m
            .value_of("SOCKET_ADDR")
            .map(|s| s.to_owned())
            .or(env::var("QAUL_HUBD_MODE").ok())
            .unwrap_or("0.0.0.0".into()),
        port: m
            .value_of("SOCKET_PORT")
            .map(|s| str::parse(s).unwrap())
            .or(env::var("QAUL_HUBD_MODE")
                .ok()
                .map(|s| str::parse(&s).unwrap()))
            .unwrap_or(9001),
    }
}

// use crate::config::Config;
// use async_std::path::PathBuf;
// use dirs::config_dir;

// use quicli::prelude::*;
// use serde_json;
// use structopt::StructOpt;

// /// Serde default functions
// mod defaults {
//     pub const DEFAULT_SYSTEM_CFG_PATH: &'static str = "/etc/qauld/config.json";
// }

// #[derive(Debug, StructOpt)]
// #[structopt(name = "qauld", about = "daemon service for qaul.net")]
// struct CommandLineOptions {
//     #[structopt(short = "c", long = "config", parse(from_os_str))]
//     configuration_file_path: Option<PathBuf>,
// }

// pub async fn inflate_options() -> Config {
//     let cli_opts = CommandLineOptions::from_args();
//     use async_std::fs::File;
//     use async_std::prelude::*;
//     let system_path = PathBuf::from(defaults::DEFAULT_SYSTEM_CFG_PATH);
//     let user_path = config_dir().map(|d| {
//         let mut p = PathBuf::from(d);
//         p.push("qauld");
//         p.push("config.json");
//         p
//     });

//     let cfg_file_path = {
//         if let Some(cli_path) = cli_opts.configuration_file_path {
//             Some(cli_path)
//         } else if user_path.is_some() && user_path.clone().unwrap().as_path().is_file().await {
//             Some(user_path.clone().unwrap())
//         } else if system_path.as_path().is_file().await {
//             Some(system_path.clone())
//         } else {
//             None
//         }
//     };

//     let cfg_file_contents = match cfg_file_path {
//         Some(pathbuf) => {
//             let mut contents = Vec::new();
//             let mut file = File::open(&pathbuf).await.expect(&format!(
//                 "Could not open configuration file at '{}'. Error",
//                 pathbuf.display()
//             ));
//             file.read_to_end(&mut contents).await.expect(&format!(
//                 "Could not read configuration file at '{}'. Error",
//                 pathbuf.display()
//             ));
//             contents
//         }
//         None => {
//             warn!("No configuration file found; both system path {} and user path {:?} are not readable or do not exist.",
//                 system_path.display(),
//                 user_path.map(|p| p.display().to_string()));
//             vec![]
//         }
//     };

//     serde_json::from_slice(&cfg_file_contents[..])
//         .expect("Could not understand configuration file. Error")
// }
