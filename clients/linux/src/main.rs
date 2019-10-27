use {
    libqaul::{messages::Recipient, Qaul},
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
    r1.modify().add_ep(mm1);
    r2.modify().add_ep(mm2);
    r2.modify().add_ep(mm3);
    r3.modify().add_ep(mm4);

    // While `libqaul` can't add users to the routing scope yet, we
    // need to now create Qaul structures so we can create users
    let q1 = Qaul::new(r1);
    let q2 = Qaul::new(r2);
    let q3 = Qaul::new(r3);

    // Generate two user profiles on node 1 and 3
    let u1 = q1.users().create("abc").unwrap();
    let u2 = q3.users().create("abc").unwrap();

    // Manually make Routers discover each other
    #[allow(deprecated)]
    {
        q1.router().discover(u2.0, 0);
        q2.router().discover(u1.0, 0);
        q2.router().discover(u2.0, 1);
        q3.router().discover(u1.0, 0);
    }

    // At this point all `Qaul` stacks are sufficiently initialised to
    // use the actual `message` API to send a message.
    q1.messages()
        .send(u1, Recipient::User(u2.0), "test".into(), vec![1, 2, 3, 4])
        .unwrap();

    // This delay is required to make the main thread wait enough time
    // for the exchange to complete. In a real app this is not a
    // problem because the main thread might be running UI or whatever
    #[allow(deprecated)]
    std::thread::sleep_ms(5000);
}
