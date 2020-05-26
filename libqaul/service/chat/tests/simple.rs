use libqaul::Qaul;
use qaul_chat::{Chat, Result};
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
        let qaul = Qaul::new(arc);
        let chat = async_std::task::block_on(async { Chat::new(Arc::clone(&qaul)).await }).unwrap();
        ChatPair { qaul, chat }
    });
    tp
}

#[async_std::test]
async fn create_room() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    // Wait for user propagations
    zzz().await;

    let room_id = net
        .a()
        .chat
        .start_chat(alice.clone(), vec![bob.0], None)
        .await?;

    zzz().await;

    let mut rooms = net.b().chat.rooms(bob.clone()).await?;
    assert!(rooms.len() == 1);
    assert_eq!(rooms.remove(0).id, room_id);
    Ok(())
}

#[async_std::test]
async fn send_message() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    println!("ALICE = {}", alice.0);
    println!("BOB   = {}", bob.0);

    // Wait for user propagations
    zzz().await;

    let room_id = net
        .a()
        .chat
        .start_chat(alice.clone(), vec![bob.0], None)
        .await?;
    println!("ROOM ID = {}", room_id);

    zzz().await;

    let room = net.b().chat.get_room(bob.clone(), room_id).await.unwrap();
    assert_eq!(room.users, vec![alice.0, bob.0].into_iter().collect());

    net.b()
        .chat
        .send_message(bob.clone(), room_id, "Hello Alice, how are you?".into())
        .await
        .unwrap();

    zzz().await;

    let msg = net
        .a()
        .chat
        .load_messages(alice.clone(), room_id)
        .await
        .unwrap()
        .into_iter()
        .find(|msg| msg.content.as_str() != "")
        .unwrap();
    assert_eq!(msg.content, String::from("Hello Alice, how are you?"));
    Ok(())
}

#[async_std::test]
async fn send_message_subscribe() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    println!("ALICE = {}", alice.0);
    println!("BOB   = {}", bob.0);

    // Wait for user propagations
    zzz().await;

    let room_id = net
        .a()
        .chat
        .start_chat(alice.clone(), vec![bob.0], None)
        .await?;

    zzz().await;

    let room = net.b().chat.get_room(bob.clone(), room_id).await.unwrap();
    assert_eq!(room.users, vec![alice.0, bob.0].into_iter().collect());

    net.b()
        .chat
        .send_message(bob.clone(), room_id, "Hello Alice, how are you?".into())
        .await
        .unwrap();

    let msg = timeout(sec5(), async {
        let sub = net
            .a()
            .chat
            .subscribe(alice.clone(), room_id)
            .await
            .unwrap();
        sub.next().await
    })
    .await
    .unwrap();

    assert_eq!(msg.content, String::from("Hello Alice, how are you?"));
    Ok(())
}

#[async_std::test]
async fn change_room_name() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    println!("ALICE = {}", alice.0);
    println!("BOB   = {}", bob.0);

    let room_name = "Super Fun Chat".to_owned();

    zzz().await;

    let room_id = net
        .a()
        .chat
        .start_chat(alice.clone(), vec![bob.0], None)
        .await?;

    zzz().await;

    net.b()
        .chat
        .set_name(bob.clone(), room_id, room_name.clone())
        .await?;

    zzz().await;

    let room = net.a().chat.get_room(alice, room_id).await?;
    assert_eq!(room.name, Some(room_name));
    Ok(())
}


#[async_std::test]
async fn create_room_with_name() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    println!("ALICE = {}", alice.0);
    println!("BOB   = {}", bob.0);

    let room_name = "Super Fun Chat".to_owned();

    zzz().await;

    let room_id = net
        .a()
        .chat
        .start_chat(alice.clone(), vec![bob.0], Some(room_name.clone()))
        .await?;

    zzz().await;

    let room = net.b().chat.get_room(bob, room_id).await?;
    assert_eq!(room.name, Some(room_name));
    Ok(())
}
