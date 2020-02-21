use super::Locked;
use crate::Message;

use async_std::{
    future::{self, Future},
    pin::Pin,
    sync::{Arc, Sender},
    task::Poll,
};
use netmod::{Frame, SeqId};
use std::collections::{BTreeMap, VecDeque};

type FrameMap = Locked<BTreeMap<SeqId, Sender<Frame>>>;

/// Local frame collector state holder
#[derive(Default)]
pub(super) struct State {
    incoming: Locked<BTreeMap<SeqId, VecDeque<Frame>>>,
    done: Locked<VecDeque<Message>>,
    tasks: FrameMap,
}

impl State {
    /// Create a new state (oh no)
    pub(crate) fn new() -> Self {
        Default::default()
    }

    /// Poll for completed messages from teh outside world
    pub(super) async fn completed(&self) -> Message {
        let done = Arc::clone(&self.done);
        future::poll_fn(|ctx| {
            let lock = &mut done.lock();
            match unsafe { Pin::new_unchecked(lock).poll(ctx) } {
                Poll::Ready(ref mut vec) if vec.len() > 0 => match vec.pop_front() {
                    Some(f) => Poll::Ready(f),
                    None => Poll::Pending,
                },
                _ => Poll::Pending,
            }
        })
        .await
    }

    /// Poll for new work on a particular frame sequence
    pub(super) async fn get(&self, seq: &SeqId) -> Frame {
        let incoming = Arc::clone(&self.incoming);
        future::poll_fn(|ctx| {
            let lock = &mut incoming.lock();
            match unsafe { Pin::new_unchecked(lock).poll(ctx) } {
                Poll::Ready(ref mut map) => match map.get_mut(seq) {
                    Some(ref mut vec) if vec.len() > 0 => match vec.pop_front() {
                        Some(f) => Poll::Ready(f),
                        None => Poll::Pending,
                    },
                    _ => Poll::Pending,
                },
                _ => Poll::Pending,
            }
        })
        .await
    }

    /// Yield a finished message to the state
    pub(super) async fn finish(&self, msg: Message) {
        self.done.lock().await.push_back(msg);
    }

    /// Queue a new frame to the state
    pub(super) async fn queue(&self, seq: SeqId, frame: Frame) {
        self.incoming
            .lock()
            .await
            .entry(seq)
            .or_default()
            .push_back(frame);
    }

    /// Get the current number of queued frames for diagnostic and testing
    #[cfg(test)]
    pub(crate) async fn num_queued(&self) -> usize {
        self.incoming
            .lock()
            .await
            .iter()
            .fold(0, |acc, (_, vec)| acc + vec.len())
    }

    /// Get the current number of completed messages
    #[cfg(test)]
    pub(crate) async fn num_completed(&self) -> usize {
        self.done.lock().await.len()
    }
}
