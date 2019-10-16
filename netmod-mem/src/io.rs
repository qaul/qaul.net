use ratman_netmod::Frame;
use std::sync::mpsc;
/// A simple I/O wrapper around channels
pub(crate) struct Io {
    pub out: mpsc::Sender<Frame>,
    pub inc: mpsc::Receiver<Frame>,
}

impl Io {
    pub(crate) fn make_pair() -> (Io, Io) {
        let (a_to_b, b_from_a) = mpsc::channel();
        let (b_to_a, a_from_b) = mpsc::channel();
        let a = Io {
            out: a_to_b,
            inc: a_from_b,
        };
        let b = Io {
            out: b_to_a,
            inc: b_from_a,
        };
        return (a, b);
    }
}
