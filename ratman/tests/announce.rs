use async_std::task;
use netmod_mem::MemMod;
use ratman::{Identity, Result, Router};

async fn testing() -> Result<()> {
    // Build a simple channel in memory
    let mm1 = MemMod::new();
    let mm2 = MemMod::new();
    mm1.link(&mm2);

    // Initialise two routers, one for each device
    let r1 = Router::new();
    let r2 = Router::new();

    // Add channel endpoints to routers
    r1.add_endpoint(mm1).await;
    r2.add_endpoint(mm2).await;

    // Create some users and add them to the routers
    let u1 = Identity::random();
    r1.add_user(u1).await?;

    let u2 = Identity::random();
    r2.add_user(u2).await?;

    // And mark them "online"
    r1.online(u1).await?;
    r2.online(u2).await?;

    // The routers will now start announcing their new users on the
    // micro-network.  You can now poll for new user discoveries.
    assert_eq!(r1.discover().await, u2);

    Ok(())
}

#[test]
fn announce_and_discover() {
    task::block_on(testing()).unwrap();
}
