//! Manage the libqaul, service and ratman states

use crate::cfg::Config;
use directories::ProjectDirs;
use libqaul::Qaul;
use netmod_tcp::{Endpoint, Mode};
use ratman::Router;
use std::collections::HashSet;
use std::{fs::File, io::Read, net::SocketAddr, str::FromStr, sync::Arc};

#[allow(unused)]
pub(crate) struct State {
    qaul: Arc<Qaul>,
    router: Arc<Router>,
}

impl State {
    /// Create a new run state
    pub(crate) async fn new(cfg: &Config) -> State {
        let mut ep = Endpoint::new(&cfg.addr, cfg.port, "qaul-hubd")
            .await
            .unwrap();
        ep.mode(match cfg.mode.as_str() {
            "dynamic" => Mode::Dynamic,
            _ => Mode::Static,
        })
        .await;

        let mut buf = String::new();
        let mut peersfd = File::open(&cfg.peers).unwrap();
        peersfd.read_to_string(&mut buf).unwrap();

        let peers = buf
            .split("\n")
            .into_iter()
            .fold(HashSet::new(), |mut set, peer| {
                if peer != "" {
                    set.insert(
                        SocketAddr::from_str(&peer)
                            .expect(&format!("Peer `{}` had a bad format!", &peer)),
                    );
                }
                set
            });

        ep.load_peers(peers.into_iter().collect()).await.unwrap();
        ep.start().await;

        let router = Router::new();
        router.add_endpoint(ep).await;

        let dirs = ProjectDirs::from("net", "qaul", "hubd").unwrap();
        let qaul = Qaul::new(Arc::clone(&router), dirs.data_dir());

        Self { qaul, router }
    }
}
