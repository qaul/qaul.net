//! A set of very simple (linear tests) for libqaul

use libqaul::{
    error::Result,
    helpers::Tag,
    messages::{Mode, MsgQuery},
    Qaul,
};

fn harness() -> std::sync::Arc<Qaul> {
    let dir = tempfile::tempdir().unwrap();
    let r = ratman::Router::new();
    Qaul::new(r, dir.path())
}

#[async_std::test]
async fn user_insert_delete() -> Result<()> {
    let q = harness();

    let auth = q.users().create("car horse battery staple").await?;
    assert_eq!(q.users().list().await.len(), 1);

    q.users().delete(auth).await?;
    assert_eq!(q.users().list().await.len(), 0);
    Ok(())
}

#[async_std::test]
async fn send_message_query() -> Result<()> {
    let q = harness();
    let auth = dbg!(q.users().create("car horse battery staple").await?);

    let msgid = q
        .messages()
        .send(
            auth.clone(),
            Mode::Flood,
            "net.qaul.testing",
            Tag::empty("test-tag"),
            vec![1, 3, 1, 2],
        )
        .await?;

    let res = q.messages()
        .query(
            auth,
            "net.qaul.testing",
            MsgQuery::new().tag(Tag::empty("test-tag")),
        )
        .await?;
    assert_eq!(res.take(1).await?[0].id, msgid);
    Ok(())
}
