#[macro_use] extern crate tracing;

use {
    async_std::future::timeout,
    libqaul::Qaul,
    qaul_voice::{Result, Voice},
    ratman_harness::{temp, Initialize, ThreePoint, millis, sec10, sec5},
    std::{
        collections::BTreeSet,
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
    tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    let mut tp = ThreePoint::new().await;
    tp.init_with(|_, arc| {
        let qaul = Qaul::new(arc);
        let voice = async_std::task::block_on(async { Voice::new(Arc::clone(&qaul)).await }).unwrap();
        VoicePair { qaul, voice }
    });
    tp
}

#[async_std::test]
async fn test() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    // first alice starts a call
    let call_id = net.a().voice.start_call(alice.clone()).await?;
    let mut participants = BTreeSet::new();
    let mut invitees = Some(alice.0).into_iter().collect();

    let mut calls = net.a().voice.get_calls(alice.clone()).await?;
    assert_eq!(calls.len(), 1);
    assert_eq!(calls.remove(0).id, call_id);

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);

    zzz().await;
    warn!("Call Started");

    // then ze invites bob to the call 
    net.a().voice.invite_to_call(alice.clone(), bob.0, call_id).await?;
    invitees.insert(bob.0);

    zzz().await;
    warn!("Bob Invited");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    let call_b = net.b().voice.get_call(bob.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);
    assert_eq!(call_b.participants, participants);
    assert_eq!(call_b.invitees, invitees);

    // bob, seeing the call and being an enby of action, decides to accept the invitation
    // and join 
    net.b().voice.join_call(bob.clone(), call_id).await?;
    participants.insert(bob.0);

    zzz().await;
    warn!("Bob Joined");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    let call_b = net.b().voice.get_call(bob.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);
    assert_eq!(call_b.participants, participants);
    assert_eq!(call_b.invitees, invitees);

    // alice sees that bob has joined and realizes that in hir enthusiasm ze has forgotten to 
    // join the call

    net.a().voice.join_call(alice.clone(), call_id).await?;
    participants.insert(alice.0);

    zzz().await;
    warn!("Alice Joined");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    let call_b = net.b().voice.get_call(bob.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);
    assert_eq!(call_b.participants, participants);
    assert_eq!(call_b.invitees, invitees);

    // they talk for a while and bob decides to leave the call
    net.b().voice.leave_call(bob.clone(), call_id).await?;
    participants.remove(&bob.0);
    invitees.remove(&bob.0);

    zzz().await;
    warn!("Bob Left");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    let call_b = net.b().voice.get_call(bob.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);

    Ok(())
}
