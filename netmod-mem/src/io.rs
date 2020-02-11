use ratman_netmod::Frame;
use crossbeam_channel::{bounded, Sender, Receiver};

/// A simple I/O wrapper around channels
pub(crate) struct Io {
    pub out: Sender<Frame>,
    pub inc: Receiver<Frame>,
}

impl Io {
    pub(crate) fn make_pair() -> (Io, Io) {
        // On order to handle backpressure on the runtime we use
        // bounded channels here via crossbeam.  This way a channel
        // will be able to hold 2 frames before it will be woken to
        // deliver them (if it was parked).  Potentially we might want
        // to make this a configurable.
        let (a_to_b, b_from_a) = bounded(2);
        let (b_to_a, a_from_b) = bounded(2);
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
