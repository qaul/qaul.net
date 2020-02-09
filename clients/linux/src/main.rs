use async_std::{sync::Arc, task};
use {
    libqaul::{Qaul, messages::Mode},
    messaging::{Messaging, TextPayload},
    netmod_mem::MemMod,
    ratman::{netmod::Endpoint, Router},
};

// This function implements a very simple message send and
// bootstrapping procedure. It is heavily documented to be useful as
// an onboarding device.
fn main() {
    // Create virtual network with two devices
    let mut mm1 = MemMod::new();
    let mut mm2 = MemMod::new();
    let mut mm3 = MemMod::new();
    let mut mm4 = MemMod::new();

    // Link the two devices together
    mm1.link(&mut mm2);
    mm3.link(&mut mm4);

    // Print the sizehint for good measure to see how large a Message
    // we can send through this endpoint
    println!("mm1.sizehint() = {} bytes", mm1.size_hint());

    // Create three routers. These hold routing state and journals and
    // are responsible for routing packets through the network
    let r1 = Router::new();
    let r2 = Router::new();
    let r3 = Router::new();

    // Add the endpoints to their respective routers
    r1.add_endpoint(mm1).unwrap();
    r2.add_endpoint(mm2).unwrap();
    r2.add_endpoint(mm3).unwrap();
    r3.add_endpoint(mm4).unwrap();

    // While `libqaul` can't add users to the routing scope yet, we
    // need to now create Qaul structures so we can create users
    let q1 = Arc::new(Qaul::new(r1));
    let q2 = Qaul::new(r2);
    let q3 = Qaul::new(r3);

    // Generate two user profiles on node 1 and 3
    let u1 = q1.users().create("abc").unwrap();
    let u2 = q3.users().create("abc").unwrap();

    // Manually make Routers discover each other
    // #[allow(deprecated)]
    // {
    //     q1.router().discover(u2.0, 0);
    //     q2.router().discover(u1.0, 0);
    //     q2.router().discover(u2.0, 1);
    //     q3.router().discover(u1.0, 0);
    // }

    // We setup a messaging endpoint listener on node u2
    let recv = Messaging::new(Arc::clone(&q3));
    recv.listen(u2.clone(), |msg| {
        dbg!(msg);
        Ok(())
    })
    .unwrap();

    // Then we setup a messaging endponti on note u1 and send a message to u2
    let msg = Messaging::new(Arc::clone(&q1));
    task::block_on(async {
        msg.send(
            u1,
            Mode::Std(u2.0),
            TextPayload {
                text: "Hello, world!".into(),
            },
        )
        .await
    })
    .unwrap();

    // This delay is required to make the main thread wait enough time
    // for the exchange to complete. In a real app this is not a
    // problem because the main thread might be running UI or whatever
    #[allow(deprecated)]
    std::thread::sleep_ms(5000);
}
