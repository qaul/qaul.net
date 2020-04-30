use {
    crate::{
        state::InvalidState,
        wire::{VoiceMessage, VoiceMessageKind},
        CallState, Result, Voices, ASC_NAME, PACKET_DURATION,
    },
    failure::Fail,
    futures::channel::mpsc,
    libqaul::{error::Error as QaulError, users::UserAuth, Identity, helpers::Tag},
    opus,
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result as FmtResult},
};

pub type CallId = Identity;

/// The number of channels in the incoming stream
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Channels {
    Mono,
    Stereo,
}

impl Channels {
    pub fn num_channels(&self) -> usize {
        match self {
            Channels::Mono => 1,
            Channels::Stereo => 2,
        }
    }
}

impl From<Channels> for opus::Channels {
    fn from(c: Channels) -> Self {
        match c {
            Channels::Mono => opus::Channels::Mono,
            Channels::Stereo => opus::Channels::Stereo,
        }
    }
}

/// The metadata needed to decode incoming packets
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StreamMetadata {
    /// The sample rate of the stream
    pub sample_rate: u32,
    /// The number of channels in the stream
    pub channels: Channels,
}

impl StreamMetadata {
    pub(crate) fn calc_samples(&self) -> usize {
        self.sample_rate as usize * self.channels.num_channels() * PACKET_DURATION / 1000
    }
}

/// The call was rejected by the remote party
#[derive(Debug)]
pub struct CallRejected;

impl Fail for CallRejected {}

impl Display for CallRejected {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "The call was rejected by the remote party")
    }
}

/// A message was recieved with a payload that doesn't make sense
#[derive(Debug)]
pub struct BadMessageKind {
    kind_tag: String,
    actual_kind: String,
}

impl BadMessageKind {
    pub fn new<A: Into<String>, B: Into<String>>(kind_tag: A, actual_kind: B) -> Self {
        Self {
            kind_tag: kind_tag.into(),
            actual_kind: actual_kind.into(),
        }
    }
}

impl Fail for BadMessageKind {}

impl Display for BadMessageKind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Failed to parse a message from the remote party: kind tag was {} but the message was {}", self.kind_tag, self.actual_kind)
    }
}

/// No call was found with the provided id
#[derive(Debug)]
pub struct CallNotFound(pub CallId);

impl Fail for CallNotFound {}

impl Display for CallNotFound {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "There are no active calls with id {}", self.0)
    }
}

/// An incoming call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncomingCall {
    /// The `id` associated with this call
    id: CallId,
    /// The identity of the calling party
    user: Identity,
}

impl Voices {
    /// Make a call to a remote user. The future will return whenever the call is
    /// accepted/rejected.
    ///
    /// `metadata` provides the stream settings this end of the conversation will use
    pub async fn call(
        &self,
        auth: UserAuth,
        user: Identity,
        metadata: StreamMetadata,
    ) -> Result<CallId> {
        let id = CallId::random();
        self.calls.lock().await.insert(
            id.clone(),
            CallState::ringing(auth.0.clone(), metadata.clone(), user.clone()),
        );
        VoiceMessage {
            call: id.clone(),
            kind: VoiceMessageKind::Incoming(metadata),
        }
        .send(self, auth.clone(), user)
        .await?;
        let tags = vec![
            Tag::new("call_id", id.clone()),
            Tag::new("kind", b"control".to_vec()),
        ];
        // let msg = self
        //     .qaul
        //     .messages()
        //     .next(auth.clone(), ASC_NAME, tags)
        //     .await?;
        unimplemented!();

        
        // let _remote = msg.sender.clone();
        // let msg: VoiceMessage = conjoiner::deserialise(&msg.payload)?;
        // match msg.kind {
        //     VoiceMessageKind::Accept(remote_metadata) => self
        //         .modify_call_state(id.clone(), |call| call.connect(remote_metadata))
        //         .await
        //         .and_then(|_| {
        //             self.start_call(id.clone(), auth)?;
        //             Ok(id)
        //         }),
        //     VoiceMessageKind::HungUp => Err(CallRejected.into()),
        //     VoiceMessageKind::Incoming(_) => Err(BadMessageKind::new("control", "Incoming").into()),
        //     VoiceMessageKind::Packet(_) => Err(BadMessageKind::new("control", "Packet").into()),
        // }
    }

    /// Accept an incoming call
    ///
    /// `metadata` provides the stream settings this end of the conversation will use
    pub async fn accept(
        &self,
        auth: UserAuth,
        call: CallId,
        metadata: StreamMetadata,
    ) -> Result<()> {
        let other = self
            .modify_call_state(call.clone(), |call| {
                let (state, res) = call.connect(metadata.clone());
                let res = res.map(|_| state.remote());
                (state, res)
            })
            .await?;
        self.start_call(call.clone(), auth.clone()).await?;
        VoiceMessage {
            call,
            kind: VoiceMessageKind::Accept(metadata),
        }
        .send(self, auth, other)
        .await?;
        Ok(())
    }

    /// Reject an incoming call, notifying the other party
    ///
    /// On the wire this is the same as hanging up
    pub async fn reject(&self, auth: UserAuth, call: CallId) -> Result<()> {
        self.hang_up(auth, call).await
    }

    /// End a call, notifying the other party
    pub async fn hang_up(&self, auth: UserAuth, call: CallId) -> Result<()> {
        let other = self
            .calls
            .lock()
            .await
            .remove(&call)
            .ok_or(CallNotFound(call))?
            .remote();
        VoiceMessage {
            call,
            kind: VoiceMessageKind::HungUp,
        }
        .send(self, auth, other)
        .await?;
        Ok(())
    }

    /// Wait for the next incoming call
    pub async fn next_incoming(&self, auth: UserAuth) -> Result<IncomingCall> {
        unimplemented!()
        
        // let msg = self
        //     .qaul
        //     .messages()
        //     .next(
        //         auth.clone(),
        //         ASC_NAME,
        //         Some(Tag::new("kind", b"incoming".to_vec())),
        //     )
        //     .await?;
        // let user = msg.sender.clone();
        // let msg: VoiceMessage = conjoiner::deserialise(&msg.payload)?;
        // match msg.kind {
        //     VoiceMessageKind::Incoming(remote_metadata) => {
        //         self.calls.lock().await.insert(
        //             msg.call.clone(),
        //             CallState::incoming(auth.0, user.clone(), remote_metadata),
        //         );
        //         Ok(IncomingCall { id: msg.call, user })
        //     }
        //     VoiceMessageKind::Accept(_) => Err(BadMessageKind::new("incoming", "Accept").into()),
        //     VoiceMessageKind::HungUp => Err(BadMessageKind::new("incoming", "HungUp").into()),
        //     VoiceMessageKind::Packet(_) => Err(BadMessageKind::new("incoming", "Packet").into()),
        // }
    }

    /// Get the metadata needed to decode incoming packets
    pub async fn get_metadata(&self, auth: UserAuth, call: CallId) -> Result<StreamMetadata> {
        self.qaul.users().ok(auth.clone())?;
        let calls = self.calls.lock().await;
        let call = calls.get(&call).ok_or(CallNotFound(call))?;
        if call.local() != auth.0 {
            return Err(QaulError::NotAuthorised.into());
        }
        call.remote_metadata()
    }

    /// Push some samples of voice data on to the outgoing voice queue
    pub async fn push_voice<V>(&self, auth: UserAuth, call: CallId, data: V) -> Result<()>
    where
        V: IntoIterator<Item = i16>,
    {
        self.qaul.users().ok(auth.clone())?;
        let mut calls = self.calls.lock().await;
        let call = calls.get_mut(&call).ok_or(CallNotFound(call))?;
        if call.local() != auth.0 {
            return Err(QaulError::NotAuthorised.into());
        }
        call.push_data(data)
    }

    /// Get the current state of a call
    pub async fn get_status(&self, auth: UserAuth, call: CallId) -> Result<CallStatus> {
        self.qaul.users().ok(auth.clone())?;
        let mut calls = self.calls.lock().await;
        let call = calls.get_mut(&call).ok_or(CallNotFound(call))?;
        if call.local() != auth.0 {
            return Err(QaulError::NotAuthorised.into());
        }
        let status = match call {
            CallState::Ringing(_) => CallStatus::Ringing,
            CallState::Incoming(_) => CallStatus::Incoming,
            CallState::Connected(_) => CallStatus::Connected,
        };
        Ok(status)
    }

    /// Subscribe to incoming voice sample packets for a call
    pub async fn subscribe_to_voice(
        &self,
        auth: UserAuth,
        call: CallId,
    ) -> Result<mpsc::UnboundedReceiver<Vec<i16>>> {
        self.qaul.users().ok(auth.clone())?;
        let mut calls = self.calls.lock().await;
        let call = calls.get_mut(&call).ok_or(CallNotFound(call))?;
        if call.local() != auth.0 {
            return Err(QaulError::NotAuthorised.into());
        }
        call.add_voice_listener()
    }

    /// A future which completes when the other end of the call hangs up
    pub async fn on_hangup(&self, auth: UserAuth, call: CallId) -> Result<()> {
        match self.get_status(auth.clone(), call.clone()).await? {
            CallStatus::Ringing => Err(InvalidState::new("Ringing")),
            CallStatus::Incoming => Err(InvalidState::new("Incoming")),
            CallStatus::Connected => Ok(()),
        }?;

        // let msg = self
        //     .qaul
        //     .messages()
        //     .next(
        //         auth,
        //         ASC_NAME,
        //         vec![
        //             Tag::new("call_id", call),
        //             Tag::new("kind", b"control".to_vec()),
        //         ],
        //     )
        //     .await?;
        // let msg: VoiceMessage = conjoiner::deserialise(&msg.payload)?;
        // match msg.kind {
        //     VoiceMessageKind::HungUp => {}
        //     VoiceMessageKind::Incoming(_) => {
        //         Err(BadMessageKind::new("control", "Incoming"))?;
        //     }
        //     VoiceMessageKind::Accept(_) => {
        //         Err(InvalidState::new("Connected"))?;
        //     }
        //     VoiceMessageKind::Packet(_) => Err(BadMessageKind::new("control", "Packet"))?,
        // }

        // self.calls.lock().await.remove(&call);

        unimplemented!()
        // Ok(())
    }
}

/// The current state of the call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CallStatus {
    /// We're waiting for the other side to either accept or reject our call
    Ringing,
    /// We have an incoming call and we need to accept or reject it
    Incoming,
    /// We are connected and audio is flowing down the wire
    Connected,
}
