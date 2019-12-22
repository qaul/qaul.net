use netmod_mem::media::BroadcastMedium;
use ratman_netmod::{Endpoint, Frame};
use async_std;

#[async_std::test]
async fn broadcast_medium_ping_pong() {
    let mut medium = BroadcastMedium::with_latency(1);
    let mut a = medium.make_netmod();
    let mut b = medium.make_netmod();
    a.send(Frame::dummy(), 0).await
        .expect("Failed to send message from a. Error");
    medium.tick();
    b.next().await
        .expect("Failed to get message at b. Error");
    b.send(Frame::dummy(), 0).await
        .expect("Failed to send message from b. Error");
    medium.tick();
    a.next().await
        .expect("Failed to get message at a. Error");
}

#[async_std::test]
async fn broadcast_medium_ping_broadcast() {
    let mut medium = BroadcastMedium::with_latency(1);
    let mut a = medium.make_netmod();
    let mut b = medium.make_netmod();
    let mut c = medium.make_netmod();
    let mut d = medium.make_netmod();

    a.send(Frame::dummy(), 0).await
        .expect("Failed to send message from a. Error");
    medium.tick();
    b.next().await
        .expect("Failed to get message at b. Error");
    c.next().await
        .expect("Failed to get message at c. Error");
    d.next().await
        .expect("Failed to get message at d. Error");
}
