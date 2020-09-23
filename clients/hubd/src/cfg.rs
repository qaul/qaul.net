use clap::{App, Arg};
use ratman::Router;
use ratman_configure::config::{Endpoint, Network, Params};
use std::collections::BTreeMap;
use std::{env, fs::File, io::Read, path::PathBuf, sync::Arc};

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

impl Config {
    /// Consume the application config into a fully initialised router
    pub(crate) fn into_router(self) -> Arc<Router> {
        let mut buf = String::new();
        let mut f = File::open(self.peers).expect("Peers configuration not found!");
        f.read_to_string(&mut buf).unwrap();

        let ep = Endpoint {
            id: 0,
            params: Params::Tcp {
                addr: self.addr,
                port: self.port,
                peers: buf
                    .split("\n")
                    .map(|s| s.parse().expect("Invalid peer port-address format!"))
                    .collect(),
                dynamic: false,
            },
        };

        Network {
            endpoints: {
                let mut map = BTreeMap::new();
                map.insert(0, ep);
                map
            },
            patches: Default::default(),
        }
        .into_router()
    }
}

pub(crate) fn cli<'a>() -> App<'a, 'a> {
    App::new("qaul-hubd")
        .about("Routing and state handling daemon for qaul.net and ratman networks")
        .version("0.1.0")
        .arg(
            Arg::with_name("PEERS_PATH")
                .short("p")
                .long("peers")
                .takes_value(true)
                .required(true)
                .value_name("PATH")
                .help("The path to a file, containing a list of newline separated initial peers"),
        )
        .arg(
            Arg::with_name("RUN_MODE")
                .short("m")
                .long("mode")
                .takes_value(true)
                .value_name("MODE")
                .default_value("static")
                .possible_values(&["static", "dynamic"])
                .help("The hub's run mode"),
        )
        .arg(
            Arg::with_name("SOCKET_ADDR")
                .short("a")
                .long("addr")
                .takes_value(true)
                .value_name("ADDR")
                .default_value("0.0.0.0")
                .help("The hub's bound socket address"),
        )
        .arg(
            Arg::with_name("SOCKET_PORT")
                .short("o")
                .long("port")
                .takes_value(true)
                .required(true)
                .value_name("PORT")
                .help("The hub's bound socket port"),
        )
}

/// Generate an application config from arguments and env vars
///
/// Environment variables are available for all parameters and will
/// override any default value.  Program arguments (commandline
/// parameters) will override env variable settings.  Any setting that
/// _must_ be present will be enforced in this function.
pub(crate) fn match_fold<'a>(app: App<'a, 'a>) -> Config {
    let m = app.get_matches();

    Config {
        peers: {
            let p = m.value_of("PEERS_PATH").map(|s| s.to_owned()).unwrap();
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
            .or(env::var("QAUL_HUBD_ADDR").ok())
            .unwrap_or("0.0.0.0".into()),
        port: m
            .value_of("SOCKET_PORT")
            .map(|s| str::parse(s).unwrap())
            .or(env::var("QAUL_HUBD_PORThat")
                .ok()
                .map(|s| str::parse(&s).unwrap()))
            .unwrap_or(9001),
    }
}
