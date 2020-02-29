use std::collections::BTreeMap;
use opus::Decoder;
use futures::channel::mpsc::UnboundedSender;


pub(crate) struct ReceivingState {
    /// The sequence number of the next packet that will be deocded
    pub(crate) next_sequence_number: u32,
    /// All currently unprocessed packets ordered by sequence number
    pub(crate) incoming_packets: BTreeMap<u32, Vec<u8>>,
    /// The Opus Decoder that will be used to decode the next packet
    pub(crate) decoder: Decoder,
    /// The number of samples in each packet
    pub(crate) samples: usize,
    /// The senders samples will be pushed to
    pub(crate) senders: Vec<UnboundedSender<Vec<i16>>>,
}

