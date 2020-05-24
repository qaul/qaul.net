use async_std::future::timeout;
use libqaul::Qaul;
use qaul_voice::{error::Result, CallEvent, Voice};
use ratman_harness::{Initialize, ThreePoint};
use std::{collections::BTreeSet, sync::Arc};
use tracing::warn;
use tracing_subscriber;

async fn zzz() {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
}

struct VoicePair {
    qaul: Arc<Qaul>,
    voice: Arc<Voice>,
}

async fn init() -> ThreePoint<VoicePair> {
    tracing_subscriber::fmt()
        .with_env_filter("qaul_voice=trace,[]=warn")
        .init();
    let mut tp = ThreePoint::new().await;
    tp.init_with(|_, arc| {
        let qaul = Qaul::new(arc);
        let voice =
            async_std::task::block_on(async { Voice::new(Arc::clone(&qaul)).await }).unwrap();
        VoicePair { qaul, voice }
    });
    tp
}

macro_rules! try_wait {
    ($f: expr) => {
        timeout(std::time::Duration::from_secs(1), $f)
            .await
            .unwrap()
    };
}

#[async_std::test]
async fn call_state() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    warn!("Alice: {}", alice.0);
    warn!("Bob: {}", bob.0);

    // Bob needs to be ready to be invited
    let inv_sub_b = net.b().voice.subscribe_invites(bob.clone()).await?;

    // Then alice starts a call and ze invites bob to the call
    let call_id = net.a().voice.start_call(alice.clone(), bob.0).await?;
    let mut participants = BTreeSet::new();
    let mut invitees = vec![alice.0, bob.0].into_iter().collect();

    let mut calls = net.a().voice.get_calls(alice.clone()).await?;
    assert_eq!(calls.len(), 1);
    assert_eq!(calls.remove(0).id, call_id);

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);

    zzz().await;
    warn!("Call Started");

    assert_eq!(try_wait!(inv_sub_b.next()).id, call_id);
    warn!("Bob Invited");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    let call_b = net.b().voice.get_call(bob.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);
    assert_eq!(call_b.participants, participants);
    assert_eq!(call_b.invitees, invitees);

    let event_sub_a = net
        .a()
        .voice
        .subscribe_call_events(alice.clone(), call_id)
        .await?;

    // bob, seeing the call and being an enby of action, decides to accept the invitation
    // and join
    net.b().voice.join_call(bob.clone(), call_id).await?;
    participants.insert(bob.0);

    assert_eq!(
        try_wait!(event_sub_a.next()),
        Some(CallEvent::UserJoined(bob.0))
    );
    warn!("Bob Joined");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    let call_b = net.b().voice.get_call(bob.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);
    assert_eq!(call_b.participants, participants);
    assert_eq!(call_b.invitees, invitees);

    let event_sub_b = net
        .b()
        .voice
        .subscribe_call_events(bob.clone(), call_id)
        .await?;

    // alice sees that bob has joined and realizes that in hir enthusiasm ze has forgotten to
    // join the call

    net.a().voice.join_call(alice.clone(), call_id).await?;
    participants.insert(alice.0);

    assert_eq!(
        try_wait!(event_sub_b.next()),
        Some(CallEvent::UserJoined(alice.0))
    );
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

    assert_eq!(
        try_wait!(event_sub_a.next()),
        Some(CallEvent::UserParted(bob.0))
    );
    warn!("Bob Left");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);

    Ok(())
}
