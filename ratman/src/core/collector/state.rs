use super::Locked;
use crate::Message;

use async_std::{
    sync::{Arc, Sender},
    task::{self, Poll},
};
use netmod::{Frame, SeqId};
use std::{collections::{BTreeMap, VecDeque}, time::Duration};

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
        loop {
            let mut vec = done.lock().await;
            match vec.pop_front() {
                Some(msg) => return msg,
                None => {}
            }
            drop(vec);
            task::sleep(Duration::from_millis(20)).await;
        }
    }

    /// Poll for new work on a particular frame sequence
    pub(super) async fn get(&self, seq: &SeqId) -> Frame {
        let incoming = Arc::clone(&self.incoming);
        loop {
            let mut map = incoming.lock().await;
            match map.get_mut(seq) {
                Some(ref mut vec) => match vec.pop_front() {
                    Some(msg) => return msg,
                    None => {}
                },
                _ => {}
            }
            drop(map);
            task::sleep(Duration::from_millis(20)).await;
        }
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
