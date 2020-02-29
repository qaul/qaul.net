use crate::{
    api::StreamMetadata,
    state::{ReceivingState, SendingState},
};
use libqaul::Identity;
use opus::{Application, Decoder, Encoder};
use std::collections::{BTreeMap, VecDeque};

/// Call connection state
pub(crate) struct ConnectedState {
    /// The local user on this call
    pub(crate) local: Identity,
    /// Stream metadata for the local user on this call
    #[allow(unused)]
    pub(crate) local_metadata: StreamMetadata,
    /// The remote user on this call
    pub(crate) remote: Identity,
    /// Stream metadata for the remote user on this call
    pub(crate) remote_metadata: StreamMetadata,
    /// The state of the outgoing end of the call
    pub(crate) sending_state: SendingState,
    /// The state of the incoming end of the call
    pub(crate) receiving_state: ReceivingState,
}

impl ConnectedState {
    pub(crate) fn new(
        local: Identity,
        local_metadata: StreamMetadata,
        remote: Identity,
        remote_metadata: StreamMetadata,
    ) -> Self {
        let sending_samples = local_metadata.calc_samples();

        // TODO: make this return errors
        let encoder = Encoder::new(
            local_metadata.sample_rate,
            local_metadata.channels.clone().into(),
            Application::Voip,
        )
        .unwrap();

        let receiving_samples = remote_metadata.calc_samples();
        let decoder = Decoder::new(
            remote_metadata.sample_rate,
            remote_metadata.channels.clone().into(),
        )
        .unwrap();

        Self {
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
