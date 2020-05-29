#[macro_use]
extern crate tracing;

use async_std::future::timeout;
use futures::stream::StreamExt;
use libqaul::Qaul;
use qaul_voice::{Result, Voice};
use ratman_harness::{Initialize, ThreePoint};
use std::{collections::VecDeque, sync::Arc, io::{BufWriter, Write}, fs::File};
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

/// This test takes a really long time so we ignore it in our usual
/// test run (but we do run ignored tests in CI).
//#[ignore]
//#[async_std::test]
async fn voice_call() -> Result<()> {
    let net = init().await;

    let alice = net.a().qaul.users().create("abc").await?;
    let bob = net.b().qaul.users().create("acab").await?;

    warn!("Alice: {}", alice.0);
    warn!("Bob: {}", bob.0);

    // await user propagation
    zzz().await;

    let call_id = net.a().voice.start_call(alice.clone()).await?;
    warn!("Call: {}", call_id);

    let mut inv_sub_b = net.b().voice.subscribe_invites(bob.clone()).await?;
    net.a().voice.join_call(alice.clone(), call_id).await?;
    net.a()
        .voice
        .invite_to_call(alice.clone(), bob.0, call_id)
        .await?;
    try_wait!(inv_sub_b.next()).unwrap();

    let mut event_sub_a = net
        .a()
        .voice
        .subscribe_call_events(alice.clone(), call_id)
        .await?;
    net.b().voice.join_call(bob.clone(), call_id).await?;
    try_wait!(event_sub_a.next()).unwrap();

    let mut data = include_bytes!("test.raw")
        .chunks(4)
        .map(|chunk| {
            let mut bytes = [0; 4];
            for i in 0..4 {
                bytes[i] = chunk[i];
            }
            f32::from_be_bytes(bytes)
        })
        .collect::<VecDeque<_>>();

    let mut audio_sub_b = net
        .b()
        .voice
        .subscribe_call_audio(bob.clone(), call_id)
        .await?;
    let stream_id = net
        .a()
        .voice
        .create_stream(alice.clone(), call_id, 44100)
        .await?;

    let mut audio_sub_b = net.b().voice.subscribe_call_audio(bob.clone(), call_id).await?;
    let stream_id = net.a().voice.create_stream(alice.clone(), call_id, 44100).await?;

    //let mut out_file = BufWriter::new(File::create("/tmp/out.raw")?); 

    let samples_per_frame = 44100 / 50;
    let mut frame = Vec::with_capacity(samples_per_frame);
    while data.len() > 0 {
        frame.truncate(0);

        while frame.len() < samples_per_frame && data.len() > 0 {
            frame.push(data.pop_front().unwrap());
        }

        while frame.len() < samples_per_frame {
            frame.push(0.0);
        }

        net.a()
            .voice
            .push_samples(alice.clone(), stream_id, &frame)
            .await?;

        let mut recvd_frame = try_wait!(audio_sub_b.next()).unwrap();
        let recvd_frame = match recvd_frame.remove(&stream_id) {
            Some(rf) => rf,
            None => { continue; }, 
        };
        let samples = recvd_frame.samples;

        //for sample in samples {
        //    out_file.write_all(&sample.to_be_bytes());
        //}
    }

    Ok(())
}
