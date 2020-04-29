//! Some simple integration tests for the qaul.net chat service

// use async_std::sync::Arc;
// use libqaul::{error::Result, harness::ThreePoint};
// use qaul_chat::Chat;

// /// We are expecting to be running on a large network and things will
// /// take time to move around.  We're using hardcoded events here and
// /// not reacing so sometimes we just gotta sleep
// async fn zzz() {
//     async_std::task::sleep(std::time::Duration::from_secs(1)).await;
// }

// #[async_std::test]
// #[ignore]
// async fn create_room() -> Result<()> {
//     let net = ThreePoint::new().await;
//     let ua = net.a.users().create("abc").await?;
//     let ub = net.b.users().create("acab").await?;

//     zzz().await;

//     let chat_a = Chat::new(Arc::clone(&net.a)).await?;
//     let chat_b = Chat::new(Arc::clone(&net.b)).await?;

//     chat_a.start_chat(ua.clone(), vec![ub.0]).await?;
//     zzz().await;

//     assert_eq!(chat_b.rooms(ub).await.unwrap().len(), 1);
//     Ok(())
// }
