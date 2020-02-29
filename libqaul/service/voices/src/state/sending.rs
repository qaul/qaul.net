use std::collections::VecDeque;
use opus::Encoder;

/// Voice service sending state
pub(crate) struct SendingState {
    /// Sequence number that will be given to the next outgoing packet
    pub(crate) next_sequence_number: u32,
    /// Set of samples queued up to be sent
    pub(crate) outgoing_samples: VecDeque<i16>,
    /// Opus Encoder that will be used to encode the next samples
    pub(crate) encoder: Encoder,
    /// Number of samples in each packet
    pub(crate) samples: usize,
}
