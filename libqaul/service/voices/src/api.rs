use {
    crate::{
        wire::{VoiceMessage, VoiceMessageKind},
        ASC_NAME, CallState, Voices,
    },
    failure::{Error, Fail},
    futures::stream::Stream,
    libqaul::{ 
        users::UserAuth,
        Identity, Tag, 
    },
    serde::{Serialize, Deserialize},
    std::fmt::{Display, Formatter, Result as FmtResult},
};

pub type CallId = Identity;
pub type Result<T> = std::result::Result<T, Error>;

/// The number of channels in the incoming stream
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Channels {
    Mono,
    Stereo,
}

/// The metadata needed to decode incoming packets
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamMetadata {
    /// The sample rate of the stream 
    sample_rate: u32,
    /// The number of channels in the stream
    channels: Channels,
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
pub struct CallNotFound(CallId);

impl Fail for CallNotFound {}

impl Display for CallNotFound {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "There are no active calls with id {}", self.0)
    }
}

/// An incoming call
#[derive(Serialize, Deserialize)]
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
    pub async fn call(&self, auth: UserAuth, user: Identity, metadata: StreamMetadata) 
    -> Result<CallId> {
        let id = CallId::random();
        VoiceMessage {
            call: id.clone(),
            kind: VoiceMessageKind::Incoming(metadata),
        }.send(self, auth.clone(), user).await?;
        let tags = vec![
            Tag::new("call_id", id.clone()),
            Tag::new("kind", b"control".to_vec()),
        ];
        let msg = self.qaul.messages()
            .next(auth.clone(), ASC_NAME, tags)
            .await?;
        let remote = msg.sender.clone();
        let msg : VoiceMessage = conjoiner::deserialise(&msg.payload)?;
        match msg.kind {
            VoiceMessageKind::Accept(remote_metadata) => {
                self.calls.lock().await.insert(id.clone(), CallState {
                    local: auth.0,
                    remote,
                    remote_metadata,
                });
                Ok(id)
            },
            VoiceMessageKind::HungUp => Err(CallRejected.into()),
            VoiceMessageKind::Incoming(_) => 
                Err(BadMessageKind::new("control", "Incoming").into()),
            VoiceMessageKind::Packet(_) => 
                Err(BadMessageKind::new("control", "Packet").into()),
        }
    }

    /// Accept an incoming call
    ///
    /// `metadata` provides the stream settings this end of the conversation will use
    pub async fn accept(&self, auth: UserAuth, call: CallId, metadata: StreamMetadata) 
    -> Result<()> {
        let other = self.calls.lock().await.get(&call).ok_or(CallNotFound(call))?.remote.clone();
        VoiceMessage {
            call,
            kind: VoiceMessageKind::Accept(metadata),
        }.send(self, auth, other).await?;
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
        let other = self.calls.lock().await.remove(&call).ok_or(CallNotFound(call))?.remote.clone();
        VoiceMessage {
            call,
            kind: VoiceMessageKind::HungUp,
        }.send(self, auth, other).await?;
        Ok(())
    }

    /// Wait for the next incoming call
    pub async fn next_incoming(&self, auth: UserAuth) -> Result<IncomingCall> {
        let msg = self.qaul.messages()
            .next(auth.clone(), ASC_NAME, Some(Tag::new("kind", b"incoming".to_vec())))
            .await?;
        let user = msg.sender.clone();
        let msg : VoiceMessage = conjoiner::deserialise(&msg.payload)?;
        match msg.kind {
            VoiceMessageKind::Incoming(remote_metadata) => {
                self.calls.lock()
                    .await
                    .insert(msg.call.clone(), CallState {
                        local: auth.0,
                        remote: user.clone(),
                        remote_metadata,
                    });
                Ok(IncomingCall {
                    id: msg.call,
                    user,
                })
            },
            VoiceMessageKind::Accept(_) =>
                Err(BadMessageKind::new("incoming", "Accept").into()),
            VoiceMessageKind::HungUp =>
                Err(BadMessageKind::new("incoming", "HungUp").into()),
            VoiceMessageKind::Packet(_) => 
                Err(BadMessageKind::new("incoming", "Packet").into()),
        }
    }

    /// Get the metadata needed to decode incoming packets
    pub async fn get_metadata(&self, auth: UserAuth, call: CallId) -> Result<StreamMetadata> {
        self.qaul.users().ok(auth)?;
        self.calls.lock().await
            .get(&call)
            .map(|v| v.remote_metadata.clone())
            .ok_or(CallNotFound(call).into())
    }
}
