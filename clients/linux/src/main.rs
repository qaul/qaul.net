use {
    libqaul::Qaul,
    netmod_mem::MemMod,
    ratman::{netmod::Endpoint, Router, Identity},
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

    // Create two routers. These hold routing state and journals and
    // are responsible for routing packets through the network
    let r1 = Router::new();
    let r2 = Router::new();
    let r3 = Router::new();

    // Add the endpoints to their respective routers
    r1.modify().add_ep(mm1);
    r2.modify().add_ep(mm2);
    r2.modify().add_ep(mm3);
    r3.modify().add_ep(mm4);

    // Generate two network IDs we will use instead of real user profiles
    let id1 = Identity::with_digest(&vec![1]);
    let id2 = Identity::with_digest(&vec![2]);

    // This step is a hack because we don't have actual discovery
    // Messages yet. It will be replaced soon though!
    r1.modify().discover(id2.clone(), 0);
    r1.modify().local(id1.clone());

    // Router 2 is purely pass-through, no local users!
    r2.modify().discover(id1.clone(), 0);
    r2.modify().discover(id2.clone(), 1);

    // Router 3 has the second local user
    r3.modify().discover(id1.clone(), 0);
    r3.modify().local(id2.clone());

    // Initialise two Qaul instances with their respective Routers.
    // At this point it's important that the routers were previously
    // initialised. Changes _can_ be made, but only from inside libqaul,
    // i.e. via some configuration service NOT 
    let q1 = Qaul::new(r1);
    let q2 = Qaul::new(r2);
    let q3 = Qaul::new(r3);

    // Send a test message from id1 to id2 that says "hello world"
    q1.send_test_message(id1, id2);

    // This delay is required to make the main thread wait enough time
    // for the exchange to complete. In a real app this is not a
    // problem because the main thread might be running UI or whatever
    #[allow(deprecated)]
    std::thread::sleep_ms(5000);
}
