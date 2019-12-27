use netmod_mem::MemMod;
use ratman_netmod::{Endpoint, Frame};
#[test]
fn ping_pong() {
    let mut a = MemMod::new();
    let mut b = MemMod::new();
    a.link(&mut b);
    a.send(Frame::dummy(), 0)
        .expect("Failed to send message from a. Error");
    b.poll()
        .expect("Failed to get message at b. Error")
        .expect("No message available.");
    b.send(Frame::dummy(), 0)
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
    a.send(Frame::dummy(), 0)
        .expect("Failed to send message from a. Error");
    // Disconnect the two interfaces, so the message sent by A will never be
    // received by B.
    b.split();
    assert!(b.poll().is_err());
}
