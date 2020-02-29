use crate::{
    api::StreamMetadata,
    state::{CallState, ConnectedState},
};
use libqaul::Identity;

pub(crate) struct IncomingState {
    /// The local user on this call
    pub(crate) local: Identity,
    /// The remote user on this call
    pub(crate) remote: Identity,
    /// Stream metadata for the remote user on this call
    pub(crate) remote_metadata: StreamMetadata,
}

impl IncomingState {
    /// Pick up the call and move to `Connected` state
    pub(crate) fn connected(self, local_metadata: StreamMetadata) -> CallState {
        CallState::Connected(ConnectedState::new(
            self.local,
            local_metadata,
            self.remote,
            self.remote_metadata,
        ))
    }
}
