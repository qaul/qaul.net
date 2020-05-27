//! libqaul message tests

mod harness;
use harness::sec5;

use libqaul::{
    helpers::TagSet,
    messages::{IdType, Mode},
    users::UserAuth,
    Identity, Qaul,
};
use std::sync::Arc;

async fn send_simple(q: &Arc<Qaul>, auth: &UserAuth, target: Identity) -> Identity {
    dbg!(q
        .messages()
        .send(
            auth.clone(),
            Mode::Std(target),
            IdType::unique(),
            "net.qaul.testing",
            TagSet::empty(),
            vec![1 as u8, 3, 1, 2],
        )
        .await
        .unwrap())
}

#[async_std::test]
async fn subscribe_one() {
    let net = harness::init().await;
    let auth_a = net.a().users().create("abc").await.unwrap();
    let auth_b = net.b().users().create("abc").await.unwrap();

    // The announcements need to spread
    // zzz(millis(2000)).await;

    // Send a message from a
    let id = send_simple(net.a(), &auth_a, auth_b.0).await;

    let msg = harness::timeout(sec5(), async {
        let sub = net
            .b()
            .messages()
            .subscribe(auth_b.clone(), "net.qaul.testing", TagSet::empty())
            .await
            .unwrap();
        sub.next().await
    })
    .await
    .unwrap();

    assert_eq!(msg.id, id);
}
