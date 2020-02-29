use crate::{
    api::StreamMetadata,
    state::{ConnectedState, IncomingState, InvalidState, InvalidStateTransition, RingingState},
    wire::Packet,
    Result, PACKET_SIZE,
};
use futures::channel::mpsc;
use libqaul::Identity;

/// A small state machine tracking the status of calls
pub(crate) enum CallState {
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
                    Err(InvalidStateTransition::new("Connected", "Connected").into()),
                );
            }
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
            CallState::Ringing(_) => Err(InvalidState::new("Ringing"))?,
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

        let encoded_packet = sending_state
            .encoder
            .encode_vec(&packet_contents, PACKET_SIZE)?;

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
            }
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
        let packet = receiving_state
            .incoming_packets
            .remove(&receiving_state.next_sequence_number)
            .unwrap_or(Vec::new());
        receiving_state.next_sequence_number += 1;

        // decode the packet into a sample set
        let mut samples = vec![0; receiving_state.samples];
        let length = receiving_state
            .decoder
            .decode(&packet[..], &mut samples[..], false)?;
        samples.truncate(length);

        // send the samples down each open channel, marking the closed channels for removal
        let to_remove = receiving_state
            .senders
            .iter_mut()
            .enumerate()
            .filter_map(|(i, sender)| {
                sender
                    .unbounded_send(samples.clone())
                    .map(|_| None)
                    .unwrap_or(Some(i))
            })
            .collect::<Vec<_>>();

        // remove them starting from the back so the indicies don't change
        to_remove.into_iter().rev().for_each(|i| {
            receiving_state.senders.remove(i);
        });

        Ok(())
    }

    /// Push an incoming packet on to the queue of packets to be decoded
    pub fn push_packet(&mut self, packet: Packet) -> Result<()> {
        let receiving_state = match self {
            CallState::Ringing(_) => Err(InvalidState::new("Ringing")),
            CallState::Incoming(_) => Err(InvalidState::new("Incoming")),
            CallState::Connected(state) => Ok(&mut state.receiving_state),
        }?;

        if receiving_state.next_sequence_number <= packet.sequence_number {
            receiving_state
                .incoming_packets
                .insert(packet.sequence_number, packet.payload);
        }

        Ok(())
    }

    /// Add a listener for incoming voice samples
    pub fn add_voice_listener(&mut self) -> Result<mpsc::UnboundedReceiver<Vec<i16>>> {
        let receiving_state = match self {
            CallState::Ringing(_) => Err(InvalidState::new("Ringing")),
            CallState::Incoming(_) => Err(InvalidState::new("Incoming")),
            CallState::Connected(state) => Ok(&mut state.receiving_state),
        }?;

        let (sender, receiver) = mpsc::unbounded();
        receiving_state.senders.push(sender);
        Ok(receiver)
    }
}
