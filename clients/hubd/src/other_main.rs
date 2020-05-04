use async_std::net::UdpSocket;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ep1 = netmod_overlay::Endpoint::spawn("0.0.0.0:20122");
    println!("Bound 20122");
    let r1 = ratman::Router::new();
    r1.add_endpoint(ep1).await;
    println!("Added endpoint to router");

    loop {
        println!("{:?}", r1.next().await);
    }
}
