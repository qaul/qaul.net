//! Slices `Message` into a series of Frames
#![allow(unused)]

use crate::{Message, MsgId};
use netmod::{Frame, Sequence};

pub(crate) struct Slicer;

impl Slicer {
    /// Take a `Message` and split it into a list of `Frames`
    // TODO: Implement this
    pub(crate) fn slice(_: usize, msg: Message) -> Vec<Frame> {
        Sequence::new(msg.sender, msg.recipient, msg.id.0)
            .add(msg.payload)
            .build()
    }

    /// Takes a set of `Frames` and turns it into a `Message`
    pub(crate) fn unslice(frames: Vec<Frame>) -> Message {
        assert!(frames.len() == 1);
        Sequence::restore(frames).into()
    }
}


impl From<Sequence> for Message {
    fn from(seq: Sequence) -> Self {
        let Sequence {
            seqid,
            sender,
            recp,
            mut data
        } = seq;

        Self {
            id: MsgId(seqid),
            sender: sender,
            recipient: recp,
            payload: data.remove(0),
            signature: vec![],
        }
    }
}
