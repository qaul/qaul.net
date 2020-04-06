use ratman_netmod::Endpoint;

#[async_std::main]
async fn main() {
    let mut ep1 = netmod_overlay::Endpoint::spawn("0.0.0.0:31337");
    let mut ep2 = netmod_overlay::Endpoint::spawn("0.0.0.0:31338");
    ep1.set_server("127.0.0.1:20122");
    ep2.set_server("127.0.0.1:20122");
    let r1 = ratman::Router::new();
    let r2 = ratman::Router::new();
    println!("Routers spawned.");
    r1.add_endpoint(ep1).await;
    r2.add_endpoint(ep2).await;
    let user1 = ratman::Identity::random();
    let user2 = ratman::Identity::random();
    r1.add_user(user1).await.unwrap();
    r2.add_user(user2).await.unwrap();
    r1.online(user1).await.unwrap();
    r2.online(user2).await.unwrap();
    println!("Users online.");

    r1.send(ratman::Message {
        id: ratman::Identity::random(),
        sender: user1,
        recipient: ratman::Recipient::Flood,
        payload: Vec::from("Hello, world!".as_bytes()),
        sign: vec![]
    }).await.expect("Could not send message!");

    println!("First message sent!");
    
    r2.send(ratman::Message {
        id: ratman::Identity::random(),
        sender: user1,
        recipient: ratman::Recipient::Flood,
        payload: Vec::from("Hello, nerd!".as_bytes()),
        sign: vec![]
    }).await.expect("Could not send message!");

    println!("Messages sent.");

    println!("{:?}", r2.next().await);
}
