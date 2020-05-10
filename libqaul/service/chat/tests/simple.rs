//! Some simple integration tests for the qaul.net chat service

use libqaul::{error::Result, Qaul};
use qaul_chat::{Chat, RoomId};
use ratman_harness::{temp, Initialize, ThreePoint};
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber;

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

async fn room_setup() -> Result<RoomId> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    println!("ALICE = {}", alice.0);
    println!("BOB   = {}", bob.0);
    
    // Wait for user propagations
    zzz().await;
    zzz().await;
    zzz().await;

    let room_id = net.a().chat.start_chat(alice.clone(), vec![bob.0]).await?;

    zzz().await;
    println!("\n===============================================\n");

    let mut rooms = net.b().chat.rooms(bob.clone()).await?;
    assert!(rooms.len() == 1);
    assert_eq!(rooms.remove(0).id, room_id);
    Ok(room_id)
}

#[async_std::test]
async fn create_room() -> Result<()> {
    // let _sub = tracing_subscriber::fmt()
    //     .with_max_level(Level::TRACE)
    //     .init();
    let _ = room_setup().await?;
    Ok(())
}

// #[async_std::test]
// async fn send_message() -> Result<()> {
//     let _ = room_setup().await?;
//     Ok(())
// }
