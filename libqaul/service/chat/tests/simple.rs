//! Some simple integration tests for the qaul.net chat service

use async_std::sync::Arc;
use libqaul::{error::Result, harness::ThreePoint};
use qaul_chat::Chat;

#[async_std::test]
async fn create_room() -> Result<()> {
    let net = ThreePoint::new().await;
    let ua = net.a.users().create("abc").await?;
    let ub = net.a.users().create("acab").await?;

    let chat_a = Chat::new(Arc::clone(&net.a)).await?;
    let chat_b = Chat::new(Arc::clone(&net.b)).await?;

    chat_a.start_chat(ua.clone(), vec![ub.0]).await?;

    assert_eq!(chat_b.rooms(ub).await.unwrap().len(), 1);
    Ok(())
}
