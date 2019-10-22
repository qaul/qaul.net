//! Slices `Message` into a series of Frames
#![allow(unused)]

use crate::{Message, Payload};
use netmod::{Frame, Sequence};

pub(crate) struct Slicer;

impl Slicer {
    /// Take a `Message` and split it into a list of `Frames`
    // TODO: Implement this
    pub(crate) fn slice(_: usize, msg: Message) -> Vec<Frame> {
        Sequence::new(msg.sender, msg.recipient)
            .add(msg.payload.data)
            .build()
    }

    /// Takes a set of `Frames` and turns it into a `Message`
    // TODO: Implement this
    pub(crate) fn unslice(mut frames: Vec<Frame>) -> Message {
        assert!(frames.len() == 1);
        let frame: Frame = frames.remove(0);
        Message {
            sender: frame.sender,
            recipient: frame.recipient,
            associator: String::new(), // TODO!
            payload: Payload {
                length: frame.payload.len() as u64,
                data: frame.payload,
            },
            signature: 0,
        }
    }
}
