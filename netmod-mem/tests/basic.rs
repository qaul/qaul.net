use netmod_mem::MemMod;
use identity::Identity;
use netmod::{Endpoint, Frame, Payload};
use std::thread;

#[test]
fn ping_pong() {
    let mut a = MemMod::new();
    let mut b = MemMod::new();
    a.link(&mut b);

    thread::spawn(move || {
        a.send(Frame {
            sender: Identity::with_digest(&vec![0, 1, 2]),
            recipient: None,
            sequence: 0,
            signature: [0; 18],
            payload: Payload::pack(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        });
    });

    assert!(b.poll().is_ok());
}

#[test]
fn split() {
    let mut a = MemMod::new();
    let mut b = MemMod::new();
    a.link(&mut b);

    thread::spawn(move || {
        a.send(Frame {
            sender: Identity::with_digest(&vec![2, 1, 0]),
            recipient: None,
            sequence: 0,
            signature: [0; 18],
            payload: Payload::pack(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        });
    });
    // Disconnect the two interfaces, so the message sent by A will never be
    // received by B.
    b.split();
    assert!(b.poll().is_err());
}
