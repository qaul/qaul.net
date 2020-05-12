use {
    crate::{
        api::{CallId, CallNotFound},
        wire::{VoiceMessage, VoiceMessageKind},
    },
    async_std::task,
    failure::Error,
    futures::{lock::Mutex, stream::StreamExt},
    libqaul::{helpers::Tag, users::UserAuth, Qaul},
    std::{
        collections::BTreeMap,
        sync::Arc,
        time::{Duration, Instant},
    },
};

pub(crate) mod state;
use state::CallState;

pub mod api;
mod wire;

pub type Result<T> = std::result::Result<T, Error>;

/// Broadcasted service name
pub(crate) const ASC_NAME: &'static str = "net.qaul.voices";

/// The duration of each packet in milliseconds
pub(crate) const PACKET_DURATION: usize = 20;

/// The maximum size in bytes of each packet
pub(crate) const PACKET_SIZE: usize = 256;

/// The packet jitter delay
///
/// The delay between the first recieved packet and the first decoded
/// packet in milliseconds. This exists to account for misordering of
/// packets and variable latency
pub(crate) const JITTER_DELAY: usize = 250;

#[derive(Clone)]
pub struct Voices {
    calls: Arc<Mutex<BTreeMap<CallId, CallState>>>,
    qaul: Arc<Qaul>,
}

impl Voices {
    pub async fn new(qaul: Arc<Qaul>) -> Result<Arc<Self>> {
        qaul.services().register(ASC_NAME, |_| {}).await?;
        Ok(Arc::new(Self {
            calls: Arc::new(Mutex::new(BTreeMap::new())),
            qaul,
        }))
    }

    /// Mutate an owned call state, potentially moving the state machine between
    /// states
    async fn modify_call_state<F, T>(&self, id: CallId, f: F) -> Result<T>
    where
        F: FnOnce(CallState) -> (CallState, Result<T>),
    {
        let mut calls = self.calls.lock().await;
        // yeah this is terrible and we should change it after the alpha
        //
        // i intend to change _all_ of this after the alpha so,
        let call = calls.remove(&id).ok_or(CallNotFound(id.clone()))?;
        let (call, res) = f(call);
        calls.insert(id, call);
        res
    }

    async fn start_call(&self, id: CallId, auth: UserAuth) -> Result<()> {
        let mut subscription = self
            .qaul
            .messages()
            .subscribe(
                auth.clone(),
                ASC_NAME,
                vec![
                    Tag::new("call_id", id.clone()),
                    Tag::new("kind", b"packet".to_vec()),
                ],
            )
            .await?;
        let voices = self.clone();
        // the connector taking incoming messages and turning them into packets
        task::spawn(async move {
            let mut task_spawned = false;
            // TODO: currently when the call drops this will leave the task dangling
            // forever
            //
            // it should do not that
            // while let msg = subscription.next().await {
            //     if !task_spawned {
            //         // the decoder heartbeat, decoding a packet every 20 ms
            //         let voices = voices.clone();
            //         task::spawn(async move {
            //             task::sleep(Duration::from_millis(JITTER_DELAY as u64)).await;

            //             let mut next_tick = Instant::now();
            //             loop {
            //                 {
            //                     let mut calls = voices.calls.lock().await;
            //                     let call = if let Some(call) = calls.get_mut(&id) {
            //                         call
            //                     } else {
            //                         break;
            //                     };

            //                     if call.decode_packet().is_err() {
            //                         break;
            //                     }
            //                 }

            //                 // this looks a little silly but it helps prevent errors
            //                 // from accumulating and causing us to needlessly miss packets
            //                 next_tick += Duration::from_millis(PACKET_DURATION as u64);
            //                 task::sleep(next_tick.duration_since(Instant::now())).await;
            //             }
            //         });
            //         task_spawned = true;
            //     }
            //     let msg: VoiceMessage = match conjoiner::deserialise(&msg.payload) {
            //         Ok(msg) => msg,
            //         Err(_) => {
            //             break;
            //         }
            //     };
            //     let packet = match msg.kind {
            //         VoiceMessageKind::Packet(p) => p,
            //         _ => {
            //             break;
            //         }
            //     };

            //     let mut calls = voices.calls.lock().await;
            //     let call = if let Some(call) = calls.get_mut(&id) {
            //         call
            //     } else {
            //         break;
            //     };

            //     match call.push_packet(packet) {
            //         Ok(_) => {}
            //         Err(_) => {
            //             break;
            //         }
            //     };
            // }
        });

        let voices = self.clone();
        // the encoder heartbeat, sending out a packet every 20 ms
        task::spawn(async move {
            let mut next_tick = Instant::now();
            loop {
                let (packet, dest) = {
                    let mut calls = voices.calls.lock().await;
                    let call = if let Some(call) = calls.get_mut(&id) {
                        call
                    } else {
                        break;
                    };

                    match call.encode_packet() {
                        Ok(p) => (p, call.remote()),
                        Err(_) => {
                            break;
                        }
                    }
                };

                let send = VoiceMessage {
                    call: id.clone(),
                    kind: VoiceMessageKind::Packet(packet),
                }
                .send(&voices, auth.clone(), dest);

                if send.await.is_err() {
                    break;
                }

                // this looks a little silly but it helps prevent errors
                // from accumulating and causing us to needlessly miss packets
                next_tick += Duration::from_millis(PACKET_DURATION as u64);
                task::sleep(next_tick.duration_since(Instant::now())).await;
            }
        });

        Ok(())
    }
}
