//! Handling message interaction with Alexandria

use super::Conv;
use crate::messages::Message;
use alexandria::{record::RecordRef, utils::Diff};

const MID: &'static str = "id";
const SENDER: &'static str = "sender";
const ASSOC: &'static str = "associate";
const SIGN: &'static str = "sign";
const PLOAD: &'static str = "payload";

// impl TakeMessage for RecordRef {
//     fn take_mesage(self) -> Option<Message> {
//         let kv = self.kv();

//         Some(Self {
//             id: Conv::id(kv.get(MID)?),
//             sender: Conv::id(kv.get(SENDER)?),
//             associator: Conv::string(kv.get(ASSOC)?),
//             tags: rec.header.tags.clone(),
//             payload: Conv::binvec(kv.get(PLOAD)?),
//         })
//     }
// }

impl From<RecordRef> for Message {
    fn from(rec: RecordRef) -> Self {
        let kv = rec.kv();

        Self {
            id: Conv::id(kv.get(MID).unwrap()),
            sender: Conv::id(kv.get(SENDER).unwrap()),
            associator: Conv::string(kv.get(ASSOC).unwrap()),
            tags: rec.header.tags.clone(),
            payload: Conv::binvec(kv.get(PLOAD).unwrap()),
        }
    }
}

impl Message {
    /// Generate a set of diffs to insert into alexandria
    pub(crate) fn diff(&self) -> Vec<Diff> {
        vec![
            Diff::map().insert(MID, self.id.as_bytes().to_vec()),
            Diff::map().insert(SENDER, self.sender.as_bytes().to_vec()),
            Diff::map().insert(ASSOC, self.associator.as_str()),
            Diff::map().insert(PLOAD, self.payload.clone()),
        ]
    }
}
