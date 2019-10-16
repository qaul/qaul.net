use netmod_mem::{media::BroadcastMedium, MemMod};
use ratman_netmod::{Endpoint, Frame};
#[test]
fn ping_pong() {
    let mut a = MemMod::new();
    let mut b = MemMod::new();
    a.link(&mut b);
    a.send(Frame::dummy())
        .expect("Failed to send message from a. Error");
    b.poll()
        .expect("Failed to get message at b. Error")
        .expect("No message available.");
    b.send(Frame::dummy())
        .expect("Failed to send message from b. Error");
    a.poll()
        .expect("Failed to get message at a. Error")
        .expect("No message available.");
}

#[test]
fn split() {
    let mut a = MemMod::new();
    let mut b = MemMod::new();
    a.link(&mut b);
    a.send(Frame::dummy())
        .expect("Failed to send message from a. Error");
    // Disconnect the two interfaces, so the message sent by A will never be
    // received by B.
    b.split();
    assert!(b.poll().is_err());
}

#[test]
fn broadcast_medium_ping_pong() {
    let mut medium = BroadcastMedium::new(1);
    let mut a = medium.make_netmod();
    let mut b = medium.make_netmod();
    a.send(Frame::dummy())
        .expect("Failed to send message from a. Error");
    medium = medium.tick();
    b.poll()
        .expect("Failed to get message at b. Error")
        .expect("No message available.");
    b.send(Frame::dummy())
        .expect("Failed to send message from b. Error");
    medium.tick();
    a.poll()
        .expect("Failed to get message at a. Error")
        .expect("No message available.");
}

#[test]
fn broadcast_medium_ping_broadcast() {
    let mut medium = BroadcastMedium::new(1);
    let mut a = medium.make_netmod();
    let mut b = medium.make_netmod();
    let mut c = medium.make_netmod();
    let mut d = medium.make_netmod();
    a.send(Frame::dummy())
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
