use {
    conjoiner,
    crate::{ASC_NAME, Result, tags},
    libqaul::{
        messages::{Mode, Message},
        users::UserAuth,
        Identity, Qaul,
    },
    serde::{Serialize, Deserialize},
    std::collections::BTreeSet,
};

pub type CallId = Identity;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Call {
    pub id: CallId,
    pub participants: BTreeSet<Identity>, 
    pub invitees: BTreeSet<Identity>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) enum CallMessage {
    Invitation(CallInvitation),
    InvitationSent(Identity),
    Join,
    Part,
    Data(CallData),
}

impl CallMessage {
    pub(crate) async fn send_to(
        &self, 
        user: UserAuth, 
        to: &BTreeSet<Identity>,
        call: CallId,
        qaul: &Qaul,
    ) -> Result<()> {
        let messages = qaul.messages();
        let payload = conjoiner::serialise(self).unwrap(); 
        for dest in to {
            if *dest == user.0 {
                continue;
            }
            
            messages
                .send(
                    user.clone(),
                    Mode::Std(dest.clone()),
                    ASC_NAME,
                    tags::call_id(call),
                    payload.clone(),
                )
                .await?;
        }

        Ok(())
    }

    pub(crate) async fn send(
        &self, 
        user: UserAuth, 
        to: Identity,
        call: CallId,
        qaul: &Qaul,
    ) -> Result<()> {
        let messages = qaul.messages();
        let payload = conjoiner::serialise(self).unwrap(); 
        messages
            .send(
                user,
                Mode::Std(to),
                ASC_NAME,
                tags::call_id(call),
                payload,
            )
            .await?;

        Ok(())
    }

    pub(crate) fn from_payload(message: &Message) -> Result<Self> {
        let this = conjoiner::deserialise(&message.payload)?;
        Ok(this)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct CallInvitation {
    pub(crate) participants: BTreeSet<Identity>,
    pub(crate) invitees: BTreeSet<Identity>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct CallData {
    pub(crate) data: Vec<u8>,
    pub(crate) sequence_number: u64,
}
