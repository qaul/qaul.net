use netmod_mem::MemMod;
use ratman_netmod::{Endpoint, Frame};
use async_std;
#[async_std::test]
async fn ping_pong() {
    let mut a = MemMod::new();
    let mut b = MemMod::new();
    a.link(&mut b);
    a.send(Frame::dummy(), 0).await
        .expect("Failed to send message from a. Error");
    b.next().await
        .expect("Failed to get message at b. Error");
    b.send(Frame::dummy(), 0).await
        .expect("Failed to send message from b. Error");
    a.next().await
        .expect("Failed to get message at a. Error");
}

#[async_std::test]
async fn split() {
    let mut a = MemMod::new();
    let mut b = MemMod::new();
    a.link(&mut b);
    a.send(Frame::dummy(), 0).await
        .expect("Failed to send message from a. Error");
    // Disconnect the two interfaces, so the message sent by A will never be
    // received by B.
    b.split();
    assert!(b.next().await.is_err());
}
