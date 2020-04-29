//! libqaul user tests

mod harness;

#[async_std::test]
async fn user_create() {
    let net = harness::init().await;

    // There should be 0 users
    assert_eq!(net.a().users().list().await.len(), 0);

    // Create a user
    let _ = net.a().users().create("abcdefg").await.unwrap();
    assert_eq!(net.a().users().list().await.len(), 1);
}

#[async_std::test]
async fn user_delete() {
    let net = harness::init().await;

    // Create a user
    let auth = net.a().users().create("abcdefg").await.unwrap();
    assert_eq!(net.a().users().list().await.len(), 1);

    net.a().users().delete(auth).await.unwrap();

    // There should be 0 users
    assert_eq!(net.a().users().list().await.len(), 0);
}

#[async_std::test]
async fn fail_delete() {
    use libqaul::{users::UserAuth, Identity};

    let net = harness::init().await;

    // Delete a user but it fails
    assert!(net
        .a()
        .users()
        .delete(UserAuth(Identity::random(), "<fake-taken>".into()))
        .await
        .is_err());
}

#[async_std::test]
async fn change_pw() {
    let net = harness::init().await;

    // Create a user
    let auth = net.a().users().create("abcdefg").await.unwrap();
    assert_eq!(net.a().users().list().await.len(), 1);

    net.a()
        .users()
        .change_pw(auth, "new and better password")
        .unwrap();
}
