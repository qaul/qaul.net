use netmod_mem::media::BroadcastMedium;
use ratman_netmod::{Endpoint, Frame};
#[test]
fn broadcast_medium_ping_pong() {
    let mut medium = BroadcastMedium::with_latency(1);
    let mut a = medium.make_netmod();
    let mut b = medium.make_netmod();
    a.send(Frame::dummy(), 0)
        .expect("Failed to send message from a. Error");
    medium.tick();
    b.poll()
        .expect("Failed to get message at b. Error")
        .expect("No message available.");
    b.send(Frame::dummy(), 0)
        .expect("Failed to send message from b. Error");
    medium.tick();
    a.poll()
        .expect("Failed to get message at a. Error")
        .expect("No message available.");
}

#[test]
fn broadcast_medium_ping_broadcast() {
    let mut medium = BroadcastMedium::with_latency(1);
    let mut a = medium.make_netmod();
    let mut b = medium.make_netmod();
    let mut c = medium.make_netmod();
    let mut d = medium.make_netmod();
    a.send(Frame::dummy(), 0)
        .expect("Failed to send message from a. Error");
    medium.tick();
    b.poll()
        .expect("Failed to get message at b. Error")
        .expect("No message available.");
    c.poll()
        .expect("Failed to get message at c. Error")
        .expect("No message available.");
    d.poll()
        .expect("Failed to get message at d. Error")
        .expect("No message available.");
}
