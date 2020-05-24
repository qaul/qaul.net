#[macro_use] extern crate tracing;

use {
    async_std::future::timeout,
    futures::stream::StreamExt,
    libqaul::Qaul,
    qaul_voice::{Result, Voice, CallEvent},
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
    tracing_subscriber::fmt().with_env_filter("qaul_voice=trace,[]=warn").init();
    let mut tp = ThreePoint::new().await;
    tp.init_with(|_, arc| {
        let qaul = Qaul::new(arc);
        let voice = async_std::task::block_on(async { Voice::new(Arc::clone(&qaul)).await }).unwrap();
        VoicePair { qaul, voice }
    });
    tp
}

macro_rules! try_wait {
    ($f: expr) => {
        timeout(std::time::Duration::from_secs(1), $f).await.unwrap()
    };
}

#[async_std::test]
async fn call_state() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    warn!("Alice: {}", alice.0);
    warn!("Bob: {}", bob.0);

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

    let mut inv_sub_b = net.b().voice.subscribe_invites(bob.clone()).await?;

    // then ze invites bob to the call 
    net.a().voice.invite_to_call(alice.clone(), bob.0, call_id).await?;
    invitees.insert(bob.0);

    
    assert_eq!(try_wait!(inv_sub_b.next()).unwrap().id, call_id);
    warn!("Bob Invited");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    let call_b = net.b().voice.get_call(bob.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);
    assert_eq!(call_b.participants, participants);
    assert_eq!(call_b.invitees, invitees);

    let mut event_sub_a = net.a().voice.subscribe_call_events(alice.clone(), call_id).await?;

    // bob, seeing the call and being an enby of action, decides to accept the invitation
    // and join 
    net.b().voice.join_call(bob.clone(), call_id).await?;
    participants.insert(bob.0);

    assert_eq!(try_wait!(event_sub_a.next()).unwrap(), CallEvent::UserJoined(bob.0));
    warn!("Bob Joined");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    let call_b = net.b().voice.get_call(bob.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);
    assert_eq!(call_b.participants, participants);
    assert_eq!(call_b.invitees, invitees);

    let mut event_sub_b = net.b().voice.subscribe_call_events(bob.clone(), call_id).await?;

    // alice sees that bob has joined and realizes that in hir enthusiasm ze has forgotten to 
    // join the call

    net.a().voice.join_call(alice.clone(), call_id).await?;
    participants.insert(alice.0);

    assert_eq!(try_wait!(event_sub_b.next()).unwrap(), CallEvent::UserJoined(alice.0));
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

    assert_eq!(try_wait!(event_sub_a.next()).unwrap(), CallEvent::UserParted(bob.0));
    warn!("Bob Left");

    let call_a = net.a().voice.get_call(alice.clone(), call_id).await?;
    assert_eq!(call_a.participants, participants);
    assert_eq!(call_a.invitees, invitees);

    Ok(())
}
