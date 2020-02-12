//! Sequence handling module

use crate::{Frame, Recipient};
use identity::Identity;
use {
    std::hash::{BuildHasher, Hasher},
    twox_hash::RandomXxHashBuilder64 as RXHash64,
};

/// A unique identifier to represents a sequence of frames
pub type SeqId = [u8; 16];

/// An XxHash signature and initialisation seed
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct XxSignature {
    sig: u64,
    seed: u64,
}

/// Encoded signature information related to a data sequence
///
/// When a large chunk of data is split across a `Frame` set,
/// signature hashes are used to verify data integrity, as well as
/// sequence ordering.  The "Sequence ID" itself can be used to
/// re-order frames received out of order, as well as verifying that a
/// `Frame` was transmitted without error.
///
/// Check the crate documentation for more details.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeqData {
    /// Frame number in sequence
    pub num: u32,
    /// A hash signature of the payload
    pub sig: XxSignature,
    /// Global frame sequence ID
    pub seqid: SeqId,
    /// Next sequenced Frame SIG
    pub next: Option<u64>,
}

/// Utility wrapping around `Vec<Frame>` with `SeqId` initialisation.
///
/// This type implements a builder, which is initialised with header
/// data, then filled with various sliced payloads, and then made into
/// a frame sequence, as outlined in the root netmod docs.
pub struct SeqBuilder {
    #[doc(hidden)]
    pub seqid: [u8; 16],
    #[doc(hidden)]
    pub sender: Identity,
    #[doc(hidden)]
    pub recp: Recipient,
    #[doc(hidden)]
    pub data: Vec<Vec<u8>>,
}

impl SeqBuilder {
    /// Initialise a Sequence builder
    pub fn new(sender: Identity, recp: Recipient, seqid: [u8; 16]) -> Self {
        Self {
            sender,
            recp,
            seqid,
            data: vec![],
        }
    }

    /// Add a slice of payload to the sequence set
    pub fn add(mut self, data: Vec<u8>) -> Self {
        self.data.push(data);
        self
    }

    /// Consume the builder into a set of frames
    pub fn build(self) -> Vec<Frame> {
        let seqid = self.seqid;
        let sender = self.sender;
        let recipient = self.recp;
        let signed = self
            .data
            .into_iter()
            .map(|d| (hash_new(&d), d))
            .collect::<Vec<_>>();

        (0..signed.len())
            .enumerate()
            .map(|(num, i)| match (signed.get(i), signed.get(i + 1)) {
                (
                    Some((ref sig, data)),
                    Some((
                        XxSignature {
                            sig: ref next,
                            seed: _,
                        },
                        _,
                    )),
                ) => (
                    SeqData {
                        num: num as u32,
                        seqid,
                        sig: *sig,
                        next: Some(*next),
                    },
                    data,
                ),
                (Some((ref sig, data)), None) => (
                    SeqData {
                        num: num as u32,
                        seqid,
                        sig: *sig,
                        next: None,
                    },
                    data,
                ),
                _ => unreachable!(),
            })
            .map(|(seq, data)| Frame {
                sender,
                recipient,
                seq,
                payload: data.to_vec(),
            })
            .collect()
    }

    /// Read the sequence ID back from the builder
    pub fn seqid(&self) -> &[u8; 16] {
        &self.seqid
    }

    /// Read the sender back from the builder
    pub fn sender(&self) -> Identity {
        self.sender
    }

    /// Read the recipient back from the builder
    pub fn recp(&self) -> Recipient {
        self.recp
    }

    /// Read the payload data set back from the builder
    pub fn data(&self) -> Vec<u8> {
        self.data.get(0).unwrap().clone()
    }
}

fn hash_new(data: &Vec<u8>) -> XxSignature {
    let mut hasher = RXHash64::default().build_hasher();
    hasher.write(data);
    XxSignature {
        sig: hasher.finish(),
        seed: hasher.seed(),
    }
}

#[cfg(test)]
fn setup() -> Vec<Frame> {
    let sender = Identity::with_digest(&vec![1]);
    let recp = Identity::with_digest(&vec![2]);
    SeqBuilder::new(sender, Recipient::User(recp), [0; 16])
        .add(vec![42])
        .add(vec![13, 12])
        .add(vec![13, 37])
        .build()
}

#[test]
fn simple() {
    let seq = setup();
    assert!(seq.len() == 3);
    assert!(seq.get(0).unwrap().seq.next == Some(seq.get(1).unwrap().seq.sig.sig));
}

/// A simple test to see if the sequence numbers are ok
#[test]
fn seq_num() {
    let seq = setup();
    assert_eq!(seq[0].seq.num, 0);
    assert_eq!(seq[1].seq.num, 1);
    assert_eq!(seq[2].seq.num, 2);
}

/// Hash sequence test
#[test]
fn hash_seq() {
    let seq = setup();
    assert_eq!(seq[0].seq.next, Some(seq[1].seq.sig.sig));
    assert_eq!(seq[1].seq.next, Some(seq[2].seq.sig.sig));
    assert_eq!(seq[2].seq.next, None);
}
