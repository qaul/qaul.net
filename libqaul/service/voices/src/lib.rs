use {
    async_std::task,
    crate::{
        api::{CallId, StreamMetadata, CallNotFound},
        wire::{Packet, VoiceMessage, VoiceMessageKind},
    },
    failure::{Error, Fail},
    futures::{
        channel::mpsc,
        stream::StreamExt,
        lock::Mutex,
    },
    libqaul::{
        users::UserAuth,
        Identity, Tag, Qaul,
    },
    std::{
        collections::{BTreeMap, VecDeque},
        fmt::{Display, Formatter, Result as FmtResult},
        sync::Arc,
        time::{Instant, Duration},
    },
    opus::{Encoder, Application, Decoder},
};

pub mod api;
mod wire;

pub type Result<T> = std::result::Result<T, Error>;

const ASC_NAME: &'static str = "net.qaul.voices";
/// The duration of each packet in milliseconds
const PACKET_DURATION: usize = 20;
/// The maximum size in bytes of each packet
const PACKET_SIZE: usize = 256;
/// The delay between the first recieved packet and the first decoded packet in milliseconds
///
/// This exists to account for misordering of packets and variable latency
const JITTER_DELAY: usize = 250;

#[derive(Clone)]
pub struct Voices {
    calls: Arc<Mutex<BTreeMap<CallId, CallState>>>,
    qaul: Arc<Qaul>,
}

impl Voices {
    pub fn new(qaul: Arc<Qaul>) -> Result<Self> {
        qaul.services().register(ASC_NAME)?;
        Ok(Self { 
            calls: Arc::new(Mutex::new(BTreeMap::new())),
            qaul 
        })
    }

    /// Mutate an owned call state, potentially moving the state machine between
    /// states
    async fn modify_call_state<F, T>(&self, id: CallId, f: F) -> Result<T> 
    where
        F: FnOnce(CallState) -> (CallState, Result<T>)
    {
        let mut calls = self.calls.lock().await;
        // yeah this is terrible and we should change it after the alpha
        //
        // i intend to change _all_ of this after the alpha so,
        let mut call = calls.remove(&id).ok_or(CallNotFound(id.clone()))?;
        let (call, res) = f(call); 
        calls.insert(id, call);
        res
    }

    fn start_call(&self, id: CallId, auth: UserAuth) -> Result<()> {
        let mut subscription = self.qaul
            .messages()
            .subscribe(auth.clone(), ASC_NAME, vec![
                Tag::new("call_id", id.clone()),
                Tag::new("kind", b"packet".to_vec()),
            ])?;
        let voices = self.clone();
        // the connector taking incoming messages and turning them into packets
        task::spawn(async move {
            // TODO: currently when the call drops this will leave the task dangling
            // forever
            //
            // it should do not that
            while let Some(msg) = subscription.next().await {
                let msg: VoiceMessage = match conjoiner::deserialise(&msg.payload) {
                    Ok(msg) => msg,
                    Err(_) => { break; },
                };
                let packet = match msg.kind {
                    VoiceMessageKind::Packet(p) => p,
                    _ => { break; }
                };

                let mut calls = voices.calls.lock().await;
                let mut call = if let Some(call) = calls.get_mut(&id) {
                    call
                } else {
                    break;
                };

                match call.push_packet(packet) {
                    Ok(_) => {},
                    Err(_) => { break; },
                };
            }
        });

        let voices = self.clone();
        // the encoder heartbeat, sending out a packet every 20 ms
        task::spawn(async move {
            let mut next_tick = Instant::now();
            loop {
                let (packet, dest) = {
                    let mut calls = voices.calls.lock().await;
                    let mut call = if let Some(call) = calls.get_mut(&id) {
                        call
                    } else {
                        break;
                    };

                    match call.encode_packet() {
                        Ok(p) => (p, call.remote()),
                        Err(_) => { break; },
                    }
                };

                let send = VoiceMessage {
                    call: id.clone(),
                    kind: VoiceMessageKind::Packet(packet),
                }.send(&voices, auth.clone(), dest);

                if send.await.is_err() {
                    break;
                }

                // this looks a little silly but it helps prevent errors
                // from accumulating and causing us to needlessly miss packets
                next_tick += Duration::from_millis(PACKET_DURATION as u64);
                task::sleep(next_tick.duration_since(Instant::now())).await;
            }
        });

        let voices = self.clone();
        // the decoder heartbeat, decoding a packet every 20 ms
        task::spawn(async move {
            task::sleep(Duration::from_millis(JITTER_DELAY as u64)).await;

            let mut next_tick = Instant::now();
            loop {
                {
                    let mut calls = voices.calls.lock().await;
                    let mut call = if let Some(call) = calls.get_mut(&id) {
                        call
                    } else {
                        break;
                    };

                    if call.decode_packet().is_err() {
                        break;
                    }
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

/// The state machine tried to move between states in an invalid way
#[derive(Debug)]
pub struct InvalidStateTransition {
    from: String,
    to: String,
}

impl InvalidStateTransition {
    pub fn new<A: Into<String>, B: Into<String>>(from: A, to: B) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl Fail for InvalidStateTransition {}

impl Display for InvalidStateTransition {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Tried to transistion from call state {} to {}", self.from, self.to)
    }
}

/// The method called is invalid for the current state
#[derive(Debug)]
pub struct InvalidState(String);

impl InvalidState {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl Fail for InvalidState {}

impl Display for InvalidState {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "The requested operation is invalid with the call in state {}", self.0)
    }
}

struct RingingState {
    /// The local user on this call
    local: Identity,
    /// Stream metadata for the local user on this call
    local_metadata: StreamMetadata,
    /// The remote user on this call
    remote: Identity,
}

impl RingingState {
    /// The remote party has picked up the call, move to `Connected` state
    fn connected(self, remote_metadata: StreamMetadata) -> CallState {
        CallState::Connected(ConnectedState::new(
            self.local,
            self.local_metadata,
            self.remote,
            remote_metadata,
        ))
    }
}

struct IncomingState {
    /// The local user on this call
    local: Identity,
    /// The remote user on this call
    remote: Identity,
    /// Stream metadata for the remote user on this call
    remote_metadata: StreamMetadata,
}

impl IncomingState {
    /// Pick up the call and move to `Connected` state
    fn connected(self, local_metadata: StreamMetadata) -> CallState {
        CallState::Connected(ConnectedState::new(
            self.local,
            local_metadata,
            self.remote,
            self.remote_metadata,
        ))
    }
}

struct ConnectedState {
    /// The local user on this call
    local: Identity,
    /// Stream metadata for the local user on this call
    local_metadata: StreamMetadata,
    /// The remote user on this call
    remote: Identity,
    /// Stream metadata for the remote user on this call
    remote_metadata: StreamMetadata,
    /// The state of the outgoing end of the call
    sending_state: SendingState,
    /// The state of the incoming end of the call
    receiving_state: ReceivingState,
}

impl ConnectedState {
    fn new(
        local: Identity, 
        local_metadata: StreamMetadata, 
        remote: Identity, 
        remote_metadata: StreamMetadata
    ) -> Self {
        let sending_samples = local_metadata.calc_samples();
        // TODO: make this return errors
        let encoder = Encoder::new(
            local_metadata.sample_rate,
            local_metadata.channels.clone().into(),
            Application::Voip,
        ).unwrap();

        let receiving_samples = remote_metadata.calc_samples();
        let decoder = Decoder::new(
            remote_metadata.sample_rate,
            remote_metadata.channels.clone().into(),
        ).unwrap();

        ConnectedState {
            local,
            local_metadata,
            remote,
            remote_metadata,
            sending_state: SendingState {
                next_sequence_number: 0,
                outgoing_samples: VecDeque::new(),
                encoder,
                samples: sending_samples,
            },
            receiving_state: ReceivingState {
                next_sequence_number: 0,
                incoming_packets: BTreeMap::new(),
                decoder,
                samples: receiving_samples,
                senders: Vec::new(),
            },
        }
    }
}

struct SendingState {
    /// The sequence number that will be given to the next outgoing packet
    next_sequence_number: u32,
    /// The set of samples queued up to be sent 
    outgoing_samples: VecDeque<i16>,
    /// The Opus Encoder that will be used to encode the next samples
    encoder: Encoder,
    /// The number of samples in each packet
    samples: usize,
}

struct ReceivingState {
    /// The sequence number of the next packet that will be deocded
    next_sequence_number: u32,
    /// All currently unprocessed packets ordered by sequence number 
    incoming_packets: BTreeMap<u32, Vec<u8>>,
    /// The Opus Decoder that will be used to decode the next packet
    decoder: Decoder,
    /// The number of samples in each packet
    samples: usize,
    /// The senders samples will be pushed to
    senders: Vec<mpsc::UnboundedSender<Vec<i16>>>,
}

/// A small state machine tracking the status of calls
enum CallState {
    /// We have sent the call to the other party but no response has been made
    Ringing(RingingState),
    /// This is an incoming call from a remote party
    Incoming(IncomingState),
    /// The call is connected and ready to move data
    Connected(ConnectedState),
}

impl CallState {
    /// Construct a new call in the `Ringing` state
    pub fn ringing(local: Identity, local_metadata: StreamMetadata, remote: Identity) -> Self {
        CallState::Ringing(RingingState {
            local,
            local_metadata,
            remote,
        })
    }

    /// Construct a new call in the `Incoming` state
    pub fn incoming(local: Identity, remote: Identity, remote_metadata: StreamMetadata) -> Self {
        CallState::Incoming(IncomingState {
            local,
            remote,
            remote_metadata,
        })
    }

    /// Connect a call in the `Ringing` or `Incoming` state
    pub fn connect(self, other_metadata: StreamMetadata) -> (Self, Result<()>) {
        let state = match self {
            CallState::Ringing(state) => state.connected(other_metadata),
            CallState::Incoming(state) => state.connected(other_metadata),
            CallState::Connected(state) => {
                return (
                    CallState::Connected(state),
                    Err(InvalidStateTransition::new("Connected", "Connected").into())
                );
            },
        };

        (state, Ok(()))
    }


    /// Get the remote party of the call
    pub fn remote(&self) -> Identity {
        match self {
            CallState::Ringing(state) => state.remote.clone(),
            CallState::Incoming(state) => state.remote.clone(),
            CallState::Connected(state) => state.remote.clone(),
        }
    }

    /// Get the local party of the call
    pub fn local(&self) -> Identity {
        match self {
            CallState::Ringing(state) => state.local.clone(),
            CallState::Incoming(state) => state.local.clone(),
            CallState::Connected(state) => state.local.clone(),
        }
    }

    /// Get the metadata of the remote party on this call
    pub fn remote_metadata(&self) -> Result<StreamMetadata> {
        let rm = match self {
            CallState::Ringing(_) => {
                Err(InvalidState::new("Ringing"))?
            },
            CallState::Incoming(state) => state.remote_metadata.clone(),
            CallState::Connected(state) => state.remote_metadata.clone(),
        };
        Ok(rm)
    }

    /// Encode the next packet of voice data for sending
    ///
    /// If there is not enough voice data in the output buffer this will encode
    /// a packet of silence
    pub fn encode_packet(&mut self) -> Result<Packet> {
        let mut sending_state = match self {
            CallState::Ringing(_) => Err(InvalidState::new("Ringing")),
            CallState::Incoming(_) => Err(InvalidState::new("Incoming")),
            CallState::Connected(state) => Ok(&mut state.sending_state),
        }?;

        let packet_contents = if sending_state.outgoing_samples.len() < sending_state.samples {
            (0..sending_state.samples)
                .map(|_| sending_state.outgoing_samples.pop_front().unwrap())
                .collect()
        } else {
            Vec::new()
        };

        let encoded_packet = sending_state.encoder.encode_vec(&packet_contents, PACKET_SIZE)?;

        let packet = Packet {
            sequence_number: sending_state.next_sequence_number,
            payload: encoded_packet,
        };
        sending_state.next_sequence_number += 1;

        Ok(packet)
    }

    /// Push some samples of voice data onto the outgoing voice queue
    pub fn push_data<V: IntoIterator<Item = i16>>(&mut self, data: V) -> Result<()> {
        match self {
            CallState::Ringing(_) => Err(InvalidState::new("Ringing").into()),
            CallState::Incoming(_) => Err(InvalidState::new("Incoming").into()),
            CallState::Connected(state) => {
                state.sending_state.outgoing_samples.extend(data);
                Ok(())
            },
        }
    }

    /// Decode the next packet of voice data and return the contained audio samples
    pub fn decode_packet(&mut self) -> Result<()> {
        let mut receiving_state = match self {
            CallState::Ringing(_) => Err(InvalidState::new("Ringing")),
            CallState::Incoming(_) => Err(InvalidState::new("Incoming")),
            CallState::Connected(state) => Ok(&mut state.receiving_state),
        }?;
        
        // get the next packet or an empty packet if it hasn't come in yet
        let packet = receiving_state.incoming_packets
            .remove(&receiving_state.next_sequence_number)
            .unwrap_or(Vec::new());
        receiving_state.next_sequence_number += 1;

        // decode the packet into a sample set
        let mut samples = vec![0; receiving_state.samples];
        let length = receiving_state.decoder.decode(&packet[..], &mut samples[..], false)?;
        samples.truncate(length);

        // send the samples down each open channel, marking the closed channels for removal
        let to_remove = receiving_state.senders
            .iter_mut()
            .enumerate()
            .filter_map(|(i, sender)| sender
                 .unbounded_send(samples.clone())
                 .map(|_| None)
                 .unwrap_or(Some(i))
            )
            .collect::<Vec<_>>();
        
        // remove them starting from the back so the indicies don't change
        to_remove
            .into_iter()
            .rev()
            .for_each(|i| { receiving_state.senders.remove(i); }); 

        Ok(())
    }

    /// Push an incoming packet on to the queue of packets to be decoded
    pub fn push_packet(&mut self, packet: Packet) -> Result<()> {
        let mut receiving_state = match self {
            CallState::Ringing(_) => Err(InvalidState::new("Ringing")),
            CallState::Incoming(_) => Err(InvalidState::new("Incoming")),
            CallState::Connected(state) => Ok(&mut state.receiving_state),
        }?;

        if receiving_state.next_sequence_number <= packet.sequence_number {
            receiving_state.incoming_packets.insert(
                packet.sequence_number, 
                packet.payload,
            );
        }

        Ok(())
    }

    /// Add a listener for incoming voice samples
    pub fn add_voice_listener(&mut self) -> Result<mpsc::UnboundedReceiver<Vec<i16>>> {
        let mut receiving_state = match self {
            CallState::Ringing(_) => Err(InvalidState::new("Ringing")),
            CallState::Incoming(_) => Err(InvalidState::new("Incoming")),
            CallState::Connected(state) => Ok(&mut state.receiving_state),
        }?;

        let (sender, receiver) = mpsc::unbounded();
        receiving_state.senders.push(sender);
        Ok(receiver)
    }
}
