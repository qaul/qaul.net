use super::Locked;
use crate::Message;

use async_std::{
    future::{self, Future},
    pin::Pin,
    sync::{Arc, Sender},
    task::Poll,
};
use netmod::{SeqId, Frame};
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
}

// #[test]
// fn join_frames_in_order_async() {
//     println!(
//         "Running async frame test. Here, `p` represents the number of frames not yet processed."
//     );
//     let (sender, recipient, seqid) = (Identity::random(), Identity::random(), Identity::random());
//     let mut seq = SeqBuilder::new(sender, Recipient::User(recipient), seqid)
//         .add(vec![0, 1, 2, 3])
//         .add(vec![4, 5, 6, 7])
//         .add(vec![8, 9, 10, 11])
//         .add(vec![12, 13, 14, 15])
//         .build();

//     task::block_on(async {
//         let collector = Collector::new();
//         let (_, incoming) = channel(1);
//         let (messages, _) = channel(1);
//         collector.clone().run(incoming, messages);

//         for frame in seq {
//             collector.queue(frame).await;
//             println!("Queued frame. p: {}", collector.in_progress());
//         }

//         println!("All frames queued. p: {}", collector.in_progress());

//         collector
//             .finish_ingestion()
//             .timeout(std::time::Duration::from_millis(100))
//             .await
//             .expect("Failed to complete ingestion.");

//         println!("Processed frames. p: {}", collector.in_progress());

//         assert!(collector.has_completed_messages());
//     });
// }
