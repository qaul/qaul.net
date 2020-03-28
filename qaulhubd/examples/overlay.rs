use ratman_netmod::Endpoint;

#[async_std::main]
async fn main() {
    let ep1 = netmod_overlay::Endpoint::spawn("0.0.0.0:31337", "127.0.0.1:20122");
    let ep2 = netmod_overlay::Endpoint::spawn("0.0.0.0:31338", "127.0.0.1:20122");
    let r1 = ratman::Router::new();
    let r2 = ratman::Router::new();
    println!("Routers spawned.");
    r1.add_endpoint(ep1).await;
    r2.add_endpoint(ep2).await;
    let user1 = ratman::Identity::random();
    let user2 = ratman::Identity::random();
    r1.add_user(user1);
    r2.add_user(user2);
    r1.online(user1).await;
    r2.online(user2).await;
    println!("Users online.");

    r1.send(ratman::Message {
        id: ratman::Identity::random(),
        sender: user1,
        recipient: ratman::Recipient::Flood,
        payload: Vec::from("Hello, world!".as_bytes()),
        sign: vec![]
    }).await;
    println!("Message sent.");
}
