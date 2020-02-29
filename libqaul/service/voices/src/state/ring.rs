use crate::{
    api::StreamMetadata,
    state::{CallState, ConnectedState},
};
use libqaul::Identity;

pub(crate) struct RingingState {
    /// The local user on this call
    pub(crate) local: Identity,
    /// Stream metadata for the local user on this call
    pub(crate) local_metadata: StreamMetadata,
    /// The remote user on this call
    pub(crate) remote: Identity,
}

impl RingingState {
    /// The remote party has picked up the call, move to `Connected` state
    pub(crate) fn connected(self, remote_metadata: StreamMetadata) -> CallState {
        CallState::Connected(ConnectedState::new(
            self.local,
            self.local_metadata,
            self.remote,
            remote_metadata,
        ))
    }
}
