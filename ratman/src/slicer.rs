//! Slices `Message` into a series of Frames

use crate::Message;
use netmod::{Frame, SeqBuilder};

/// Slices messages into managable chunks
pub(crate) struct Slicer;

impl Slicer {
    /// Take a `Message` and split it into a list of `Frames`
    // FIXME: Don't assume infinite buffer sizes
    pub(crate) fn slice(_: usize, msg: Message) -> Vec<Frame> {
        SeqBuilder::new(msg.sender, msg.recipient, msg.id)
            .add(msg.payload)
            .build()
    }
}
