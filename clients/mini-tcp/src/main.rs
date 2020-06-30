use async_std::task;
use netmod_tcp::Endpoint;
use ratman::Router;
use std::{
    env,
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

#[async_std::main]
async fn main() {
    let mut args: Vec<String> = env::args().into_iter().collect();
    let port = str::parse(&args.remove(1)).unwrap();
    let peer_port = str::parse(&args.remove(1)).unwrap();

    let mut ep = Endpoint::new("0.0.0.0", port, "").await.unwrap();
    ep.load_peers(vec![SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        peer_port,
    )])
    .await
    .unwrap();

    let r = Router::new();
    r.add_endpoint(ep).await;

    task::sleep(Duration::from_secs(120)).await;
}
