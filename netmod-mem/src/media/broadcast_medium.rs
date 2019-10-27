use std::collections::{BTreeMap, VecDeque};
use crate::io::Io;
use crate::MemMod;
use std::sync::mpsc::TryRecvError;
use crate::media::TaggedFrame;
use ratman_netmod::Endpoint;

/// A `BroadcastMedium` permits up to 2^32 `MemMod` interfaces to connect and
/// always sends all messages to all connected interfaces except for the sender.
// MemMods are assigned a 4-byte tag, and Frames are tagged with the internal ID
// of their sender, so they can be broadcast to only other connected interfaces,
// and a TTL value, which is decremented on each tick. Once that value reaches zero,
// the Frame can be transmitted.
// This is effective but remains lightweight, adding only a 8 bytes to each
// Frame while in "transit".
#[derive(Default)]
pub struct BroadcastMedium {
    /// The number of system ticks a frame spends in transmission; that is, the time
    /// between a frame being sent and the frame being available to other modules.
    latency: u32,
    /// The last tag used for a MemMod.
    last_tag: u32,
    /// The frames currently in transmission.
    buffer: VecDeque<TaggedFrame>,
    /// The raw `mpsc` transmission interfaces, with their associated tags.
    interfaces: BTreeMap<u32, Io>,
}

impl BroadcastMedium {
    pub fn new(latency: u32) -> Self {
        assert_ne!(
            latency, 0,
            "Cannot create a BroadcastMedium with latency == 0."
        );
        Self {
            latency,
            ..Default::default()
        }
    }

    pub fn make_netmod(&mut self) -> impl Endpoint {
        let mut mm = MemMod::new();
        let (mm_io, my_io) = Io::make_pair();
        mm.link_raw(mm_io);
        self.interfaces.insert(self.last_tag, my_io);
        self.last_tag += 1;
        mm
    }

    pub fn tick(mut self) -> Self {
        let mut disconnected: Vec<u32> = Vec::new();
        for (tag, io) in &mut self.interfaces {
            match io.inc.try_recv() {
                Ok(frame) => {
                    self.buffer
                        .push_back(TaggedFrame::new(*tag, self.latency, frame));
                }
                Err(e) => match e {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => disconnected.push(*tag),
                },
            }
        }

        let mut to_send = 0;
        self.buffer
            .iter_mut()
            .enumerate()
            .for_each(|(index, mut frame)| {
                frame.ttl -= 1;
                if frame.ttl == 0 {
                    to_send += 1;
                }
            });

        while to_send > 0 {
            to_send -= 1;
            let frame = self.buffer.pop_front().expect(
                "No frames in buffer despite having recorded a frame as requiring send this tick.",
            );
            self.interfaces
                .iter()
                .for_each(|(tag, io)| match io.out.send(frame.frame.clone()) {
                    Ok(_) => (),
                    Err(_) => disconnected.push(*tag),
                });
        }

        disconnected.sort_unstable();
        disconnected.dedup();
        disconnected.iter().for_each(|i| {
            self.interfaces.remove(i);
        });

        return self;
    }
}
