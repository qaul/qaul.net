use {
    libqaul::Qaul,
    netmod_mem::MemMod,
    ratman::{netmod::Endpoint, Router},
};

fn main() {
    // Create virtual network with two devices
    let mut mm1 = MemMod::new();
    let mut mm2 = MemMod::new();
    mm1.link(&mut mm2);

    println!("mm1.sizehint() = {} bytes", mm1.size_hint());

    let r1 = Router::new();
    let r2 = Router::new();

    r1.modify().add_ep(mm1);
    r2.modify().add_ep(mm2);

    let q1 = Qaul::new(r1);
    let q2 = Qaul::new(r2);

    // dbg!(&mm1.size_hint());

    // let f = Frame {
    //     sequence: 0,
    //     sender: Identity::with_digest(&vec![1, 2, 3]),
    //     recipient: Some(Identity::with_digest(&vec![3, 2, 1])),
    //     signature: [0; 18],
    //     payload: Payload::pack(vec![ 0, 1, 2, 3 ]),
    // };

    // // We want to send this frame!
    // dbg!(&f);

    // let j = thread::spawn(move || {
    //     println!("Spawning listener!");
    //     mm2.listen(Box::new(|f| {
    //         println!("Received frame!!!");
    //         dbg!(&f.sender);
    //         Ok(())
    //     }));
    // });

    // mm1.send(f);
    // j.join();

    // println!("{}", Identity::with_digest(&vec![]));
    //let ds = DataStore::new("/home/".into());
}
