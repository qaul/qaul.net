use libqaul::*; // ALL YOUR IMPORTS ARE BELONG TO US
use netmod_mem::MemMod;
use ratman_netmod::{Frame, Payload, Endpoint};
use std::thread;

fn main() {

    // Create virtual network with two devices
    let mut mm1 = MemMod::new();
    let mut mm2 = MemMod::new();
    mm1.link(&mut mm2);

    let f = Frame {
        sequence: 0,
        sender: Identity::with_digest(&vec![1, 2, 3]),
        recipient: Some(Identity::with_digest(&vec![3, 2, 1])),
        signature: [0; 18],
        payload: Payload::pack(vec![ 0, 1, 2, 3 ]),
    };

    // We want to send this frame!
    // dbg!(&f);

    let j = thread::spawn(move || {
        println!("Spawning listener!");
        mm2.listen(|f| {
            println!("Received frame!!!");
            dbg!(&f.sender);
            Ok(())
        });
    });

    mm1.send(f);
    j.join();
    
    // println!("{}", Identity::with_digest(&vec![]));
    //let ds = DataStore::new("/home/".into());
}
