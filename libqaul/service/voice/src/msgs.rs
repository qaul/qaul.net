//! Message handler wrapper

use crate::{error::Result, tags, CallId, CallMessage, ASC_NAME};
use conjoiner;
use libqaul::{
    messages::{Message, Mode, ID_LEN, IdType},
    users::UserAuth,
    Identity, Qaul,
};
use std::collections::BTreeSet;

impl CallMessage {
    /// send to a group of users
    pub(crate) async fn send_many(
        &self,
        user: UserAuth,
        to: &BTreeSet<Identity>,
        call: CallId,
        qaul: &Qaul,
    ) -> Result<()> {
        let messages = qaul.messages();
        let payload = conjoiner::serialise(self).unwrap();
        let id_type = IdType::create_group();
        for dest in to {
            if *dest == user.0 {
                continue;
            }

            messages
                .send(
                    user.clone(),
                    Mode::Std(dest.clone()),
                    id_type,
                    ASC_NAME,
                    tags::call_id(call),
                    payload.clone(),
                )
                .await?;
        }

        Ok(())
    }

    /// send to a specific user
    pub(crate) async fn send_single(
        &self,
        user: UserAuth,
        to: Identity,
        call: CallId,
        qaul: &Qaul,
    ) -> Result<()> {
        let messages = qaul.messages();
        let payload = conjoiner::serialise(self).unwrap();
        messages
            .send(user, Mode::Std(to), IdType::unique(), ASC_NAME, tags::call_id(call), payload)
            .await?;

        Ok(())
    }
}

/// Takes a received message and deserialises call id from tag set
pub(crate) fn grab_call_id(msg: &Message) -> Option<CallId> {
    msg.tags
        .iter()
        .filter(|tag| tag.key == "call-id")
        .filter(|tag| tag.val.len() != ID_LEN)
        .map(|tag| Identity::from_bytes(&tag.val))
        .next()
}

pub(crate) fn convert_message(msg: &Message) -> Option<CallMessage> {
    conjoiner::deserialise(&msg.payload).ok()
}
