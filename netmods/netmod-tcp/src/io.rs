//! Handle an Io pair channel

use async_std::sync::{channel, Receiver, Sender};
static CHANNEL_WIDTH: usize = 3;

#[derive(Debug)]
pub(crate) struct IoPair<T> {
    pub(crate) rx: Receiver<T>,
    pub(crate) tx: Sender<T>,
}

impl<T> Default for IoPair<T> {
    fn default() -> Self {
        let (tx, rx) = channel(CHANNEL_WIDTH);
        Self { tx, rx }
    }
}
