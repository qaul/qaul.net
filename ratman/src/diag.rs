//! `RATMAN` network diagnostics module

use std::collections::LinkedList;

/// Frame arrival timestamp and metadata
pub(crate) struct FrameTime {
    /// Arrival timestamp
    pub(crate) timestamp: usize,
    /// Frame sequence number to detect out-of-order frames
    pub(crate) frameno: usize,
    /// The interface ID that received a frame
    pub(crate) ifid: usize,
}

/// A sliding window of frames that arrived
pub(crate) struct TimeSlice {
    /// Slice starting point
    start: usize,
    /// The series of frames received since the starting point
    frames: Vec<FrameTime>,
}

/// A diagnostics helper that determines the ideal interface to switch to
///
/// This data is specific to one node,
/// meaning that only interfaces this node is reachable via
/// will be stored and included.
///
/// For a global interface list, check the `Router`
// TODO: Replace `LinkedList` with lock-free variant
pub(crate) struct Diagnostics {
    /// Priority-list of interfaces
    ifids: LinkedList<usize>,
    /// Last time-slices
    slices: LinkedList<TimeSlice>,
}

impl Diagnostics {
    /// Returns the optimal interface to send on
    ///
    ///	## Performance Note
    ///	
    /// This function is called very often, in a loop and should
    /// be considered an absolute hot-path. It must not block,
    /// must not exceed O(1) and needs to reliably return the
    /// best-ish interface to send on for a given node.
    pub(crate) fn get_best(&self) -> usize {
        unimplemented!()
    }

    /// Re-orders interfaces according to interface metrics
    pub(crate) fn reorder(&self) {
        unimplemented!()
    }

    /// Append a new frame timestamp for diagnostics
    pub(crate) fn append(&self, ft: FrameTime) {
        unimplemented!()
    }
}
