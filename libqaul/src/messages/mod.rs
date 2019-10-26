//! Internal `Message` handling module

use crate::{Message, QaulResult, Recipient};
use conjoiner;
use ratman::{Identity, Message as RatMessage};
use serde::{de::DeserializeOwned, Serialize};

trait SignUtil: Serialize + DeserializeOwned {
    /// Sign a `Message` into a raw payload.
    fn sign(self) -> QaulResult<RatMessage>;
    /// Verify an incoming `ratman::Message` signature stack
    fn verify(msg: RatMessage) -> QaulResult<Self>;
}

impl SignUtil for Message {
    fn sign(self) -> QaulResult<RatMessage> {
        let payload = conjoiner::serialise(&self);

        // let Message {
        //     sender,
        //     recipient,
        //     associator,
        //     payload,
        // } = self;

        // TODO: Build signature
        unimplemented!()
    }

    fn verify(msg: RatMessage) -> QaulResult<Self> {
        unimplemented!()
    }
}
