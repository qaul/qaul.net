//! An announce test on a three-node network
//!
//! This test is similar to the one featured in the root doc page,
//! except that the network is a bit more complicated.  This test
//! implements a three-node network, meaning that messages _have_ to
//! be re-transmitted by a middle router.
//!
//! From experience, these kinds of low-level, distributed tests are
//! good at catching errors in the code, but also demonstrate how the
//! concepts in Ratman scale.
//!
//! Because a `MemMod` is always 1-to-1, we need to create two, and
//! then add two interfaces to the middle router.

use async_std::task;
use netmod_mem::MemMod;
use ratman::{Identity, Result, Router};

#[async_std::test]
async fn announce_and_discover() -> Result<()> {
    // Build two channels in memory
    let mm1 = MemMod::new();
    let mm2_1 = MemMod::new();
    let mm2_3 = MemMod::new();
    let mm3 = MemMod::new();
    mm1.link(&mm2_1);
    mm2_3.link(&mm3);

    // Initialise three empty routers
    let r1 = Router::new();
    let r2 = Router::new();
    let r3 = Router::new();

    // Attach endpoints so the topology is r1 - r2 - r3
    r1.add_endpoint(mm1).await;
    r2.add_endpoint(mm2_1).await;
    r2.add_endpoint(mm2_3).await;
    r3.add_endpoint(mm3).await;

    // Create two users and add them to the routers
    let u1 = Identity::random();
    r1.add_user(u1).await?;

    let u3 = Identity::random();
    r3.add_user(u3).await?;

    // And mark them "online"
    r1.online(u1).await?;
    r3.online(u3).await?;

    // The routers will now start announcing their new users on the
    // micro-network.  You can now poll for new user discoveries.
    assert_eq!(r1.discover().await, u3);
    Ok(())
}

#[async_std::test]
async fn message_id_check() -> Result<()> {
    use ratman::Message;
    
    // Build two channels in memory
    let mm1 = MemMod::new();
    let mm2_1 = MemMod::new();
    let mm2_3 = MemMod::new();
    let mm3 = MemMod::new();
    mm1.link(&mm2_1);
    mm2_3.link(&mm3);

    // Initialise three empty routers
    let r1 = Router::new();
    let r2 = Router::new();
    let r3 = Router::new();

    // Attach endpoints so the topology is r1 - r2 - r3
    r1.add_endpoint(mm1).await;
    r2.add_endpoint(mm2_1).await;
    r2.add_endpoint(mm2_3).await;
    r3.add_endpoint(mm3).await;

    // Create two users and add them to the routers
    let u1 = Identity::random();
    r1.add_user(u1).await?;

    let u3 = Identity::random();
    r3.add_user(u3).await?;

    // And mark them "online"
    r1.online(u1).await?;
    r3.online(u3).await?;

    Messa
        id: MsgId::random(),
        recipient,
        sender,
        payload,
        sign: vec![],
    }
}
