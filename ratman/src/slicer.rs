//! Slices `Message` into a series of Frames

use crate::Message;
use netmod::{Endpoint, Frame};

pub(crate) struct Slicer;

impl Slicer {
    /// Take a `Message` and split it into a list of `Frames`
    pub(crate) fn slice(size_hint: usize, msg: Message) -> Vec<Frame> {
        vec![]
    }
}
