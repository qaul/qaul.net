use libqaul::{error::Result, Qaul};
use qaul_chat::Chat;
use ratman_harness::{temp, Initialize, ThreePoint};
use std::sync::Arc;

pub use async_std::future::timeout;
pub use ratman_harness::{millis, sec10, sec5};

/// We are expecting to be running on a large network and things will
/// take time to move around.  We're using hardcoded events here and
/// not reacing so sometimes we just gotta sleep
async fn zzz() {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
}

struct ChatPair {
    qaul: Arc<Qaul>,
    chat: Arc<Chat>,
}

async fn init() -> ThreePoint<ChatPair> {
    let mut tp = ThreePoint::new().await;
    tp.init_with(|_, arc| {
        let qaul = Qaul::new(arc, temp().path());
        let chat = async_std::task::block_on(async { Chat::new(Arc::clone(&qaul)).await }).unwrap();
        ChatPair { qaul, chat }
    });
    tp
}


#[async_std::test]
async fn rooms_for_different_people() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    // Wait for user propagations
    zzz().await;

    let room_1 = net.a().chat.start_chat(alice.clone(), vec![bob.0]).await?;

    zzz().await;

    let mut rooms = net.b().chat.rooms(bob.clone()).await?;
    assert!(rooms.len() == 1);
    assert_eq!(rooms.remove(0).id, room_1);

    ///// And do it again
    
    let charlie = net.a().qaul.users().create("abc").await?;
    let david = net.b().qaul.users().create("acab").await?;

    // Wait for user propagations
    zzz().await;

    let room_2 = net.a().chat.start_chat(charlie.clone(), vec![david.0]).await?;

    zzz().await;

    let mut rooms = net.b().chat.rooms(david.clone()).await?;
    assert!(rooms.len() == 1);
    assert_eq!(rooms.remove(0).id, room_2);
    
    Ok(())
}
