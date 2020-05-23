#[macro_use] extern crate tracing;

use {
    async_std::future::timeout,
    futures::stream::StreamExt,
    libqaul::Qaul,
    qaul_voice::{Result, Voice},
    ratman_harness::{temp, Initialize, ThreePoint, millis, sec10, sec5},
    std::{
        collections::BTreeSet,
        time::Duration,
        sync::Arc,
    },
    tracing::Level,
    tracing_subscriber,
};

async fn zzz() {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
}

struct VoicePair {
    qaul: Arc<Qaul>,
    voice: Arc<Voice>,
}

async fn init() -> ThreePoint<VoicePair> {
    //tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    let mut tp = ThreePoint::new().await;
    tp.init_with(|_, arc| {
        let qaul = Qaul::new(arc);
        let voice = async_std::task::block_on(async { Voice::new(Arc::clone(&qaul)).await }).unwrap();
        VoicePair { qaul, voice }
    });
    tp
}

#[async_std::test]
async fn subscribe_invite() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    let mut subscription = net.a().voice.subscribe_invites(alice.clone()).await?;
    let call_id = net.b().voice.start_call(bob.clone()).await?;

    zzz().await;

    net.b().voice.invite_to_call(bob.clone(), alice.0, call_id).await?; 
    assert_eq!(
        timeout(Duration::from_secs(1), subscription.next()).await.unwrap().unwrap().id, 
        call_id
    );

    Ok(())
}
