use {
    async_std::sync::{Mutex, RwLock},
    conjoiner,
    crate::{ASC_NAME, Result, tags, InvitationSubscription},
    futures::{
        channel::mpsc::Sender,
        future::AbortHandle,
    },
    libqaul::{
        messages::{Mode, Message},
        users::UserAuth,
        Identity, Qaul,
    },
    opus::{Decoder, Encoder},
    rubato::SincFixedOut,
    serde::{Serialize, Deserialize},
    std::{
        collections::{BTreeSet, BTreeMap, VecDeque},
        time::Instant,
    },
};

pub type CallId = Identity;
pub type StreamId = Identity;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Call {
    pub id: CallId,
    /// Who has joined the call?
    pub participants: BTreeSet<Identity>, 
    /// Who has been invited to the call?
    pub invitees: BTreeSet<Identity>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) enum CallMessage {
    /// invite a user
    Invitation(CallInvitation),
    /// note that you have invited a user
    InvitationSent(Identity),
    /// join a call
    Join,
    /// leave a call
    Part,
    /// send some data to the call
    Data(CallData),
}

impl CallMessage {
    /// send to a group of users
    pub(crate) async fn send_to(
        &self, 
        user: UserAuth, 
        to: &BTreeSet<Identity>,
        call: CallId,
        qaul: &Qaul,
    ) -> Result<()> {
        let messages = qaul.messages();
        let payload = conjoiner::serialise(self).unwrap(); 
        for dest in to {
            if *dest == user.0 {
                continue;
            }
            
            messages
                .send(
                    user.clone(),
                    Mode::Std(dest.clone()),
                    ASC_NAME,
                    tags::call_id(call),
                    payload.clone(),
                )
                .await?;
        }

        Ok(())
    }

    /// send to a specific user
    pub(crate) async fn send(
        &self, 
        user: UserAuth, 
        to: Identity,
        call: CallId,
        qaul: &Qaul,
    ) -> Result<()> {
        let messages = qaul.messages();
        let payload = conjoiner::serialise(self).unwrap(); 
        messages
            .send(
                user,
                Mode::Std(to),
                ASC_NAME,
                tags::call_id(call),
                payload,
            )
            .await?;

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct CallInvitation {
    pub(crate) participants: BTreeSet<Identity>,
    pub(crate) invitees: BTreeSet<Identity>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct CallData {
    pub(crate) stream: StreamId,
    pub(crate) data: Vec<u8>,
    pub(crate) sequence_number: u32,
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CallEvent {
    UserInvited(Identity),
    UserJoined(Identity),
    UserParted(Identity),
}

/// 20ms of audio data for all incoming streams indexed by stream id
pub type VoiceData = BTreeMap<StreamId, VoiceDataPacket>;

/// An individal audio packet from a stream
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VoiceDataPacket {
    /// What user sent this packet?
    pub user: Identity,
    /// The audio samples
    pub samples: Vec<f32>,
}

pub(crate) struct CallUser {
    pub(crate) auth: UserAuth,
    pub(crate) invitation_subs: RwLock<Vec<Sender<Call>>>,
    pub(crate) call_event_subs: RwLock<BTreeMap<CallId, Vec<Sender<CallEvent>>>>,
    pub(crate) stream_subs: RwLock<BTreeMap<CallId, Vec<Sender<VoiceData>>>>,
    pub(crate) incoming_streams: RwLock<BTreeMap<StreamId, StreamState>>,
    pub(crate) outgoing_streams: RwLock<BTreeMap<StreamId, EncoderStreamState>>,
    pub(crate) abort_handles: Vec<AbortHandle>,
}

pub(crate) struct StreamState {
    /// what call does this stream belong to?
    pub(crate) call: CallId,
    /// and what user is sending it?
    pub(crate) user: Identity, 
    /// a buffer of packets indexed by sequence numbers
    ///
    /// this is where new incoming packets are stored to allow
    /// them to come in in a different order than they were sent
    pub(crate) jitter_buffer: BTreeMap<u32, Vec<u8>>,
    /// the sequence number of the next packet to be decoded
    pub(crate) next_sequence_number: u32,
    /// an instant representing when the stream will go live
    ///
    /// when the stream starts it delays before decoding packets
    /// to allow for the jitter buffer to fill up
    pub(crate) startup_timeout: Option<Instant>,
    /// an instant representing when the stream will shutdown
    ///
    /// this field will be set when the decoder tries to decode a 
    /// packet and can't find one. it will be cleared if a packet is
    /// found but if the timer is allowed to expire the stream will be
    /// flushed from memory.
    pub(crate) shutdown_timeout: Option<Instant>,
    /// the decoder that does the actual work of decoding packets
    ///
    /// behind a mutex because it is not `Send`
    pub(crate) decoder: Mutex<Decoder>,
}

pub(crate) struct EncoderStreamState {
    pub(crate) call: CallId,
    pub(crate) samples: VecDeque<f32>,
    pub(crate) next_sequence_number: u32,
    pub(crate) encoder: Mutex<Encoder>,
    pub(crate) resampler: SincFixedOut<f32>,
}
