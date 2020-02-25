use {
    crate::Voices,
    failure::{Error, Fail},
    futures::stream::Stream,
    libqaul::{ 
        users::UserAuth,
        Identity 
    },
    serde::{Serialize, Deserialize},
    std::fmt::{Display, Formatter, Result as FmtResult},
};

pub type CallId = Identity;
pub type Result<T> = std::result::Result<T, Error>;

/// The number of channels in the incoming stream
#[derive(Serialize, Deserialize, Debug)]
pub enum Channels {
    Mono,
    Stereo,
}

/// The metadata needed to decode incoming packets
#[derive(Serialize, Deserialize, Debug)]
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
        unimplemented!();
    }

    /// Accept an incoming call
    ///
    /// `metadata` provides the stream settings this end of the conversation will use
    pub async fn accept(&self, auth: UserAuth, call: CallId, metadata: StreamMetadata) 
    -> Result<()> {
        unimplemented!();
    }

    /// Reject an incoming call, notifying the other party
    pub async fn reject(&self, auth: UserAuth, call: CallId) -> Result<()> {
        unimplemented!();
    }

    /// End a call, notifying the other party
    pub async fn hang_up(&self, auth: UserAuth, call: CallId) -> Result<()> {
        unimplemented!();
    }

    /// Wait for the next incoming call
    pub async fn next_incoming(&self, auth: UserAuth) -> Result<IncomingCall> {
        unimplemented!();
    }

    /// Get the metadata needed to decode incoming packets
    pub async fn get_metadata(&self, auth: UserAuth, call: CallId) -> Result<StreamMetadata> {
        unimplemented!();
    }
}
