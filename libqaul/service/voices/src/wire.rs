use {
    crate::{ 
        api::{ CallId, Channels, StreamMetadata },
        ASC_NAME, Voices,
    },
    conjoiner,
    failure::{Error},
    libqaul::{ 
        messages::{MsgId, Mode},
        users::UserAuth,
        Identity, Tag, 
    },
    serde::{Serialize, Deserialize},
};

#[derive(Serialize, Deserialize, Debug)]
/// Differentiates the various kinds of voice messages
pub enum VoiceMessageKind {
    /// An incoming call
    Incoming(StreamMetadata),
    /// The call was accepted by the remote party
    Accept(StreamMetadata),
    /// The call was ended by the remote party 
    HungUp,
    /// An OPUS encoded voice packet
    Packet(Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug)]
/// The on-the-wire representation of a voice service message
pub struct VoiceMessage {
    /// A unique identifier for this call 
    pub call: CallId,
    /// The actually content of the message
    pub kind: VoiceMessageKind,
}

impl VoiceMessage {
    /// Serialize the message and send it down the wire with appropriate tags
    pub async fn send(self, voices: &Voices, auth: UserAuth, to: Identity)
    -> Result<MsgId, Error> {
        let payload = conjoiner::serialise(&self).unwrap(); 
        let tags = vec![ 
            Tag::new("call_id", self.call),
            match self.kind {
                VoiceMessageKind::Incoming(_) => Tag::new("kind", b"incoming".to_vec()),
                VoiceMessageKind::Accept(_) |
                    VoiceMessageKind::HungUp => Tag::new("kind", b"control".to_vec()),
                VoiceMessageKind::Packet(p) => Tag::new("kind", b"packet".to_vec()),
            },
        ];
        Ok(voices.qaul.messages().send(
            auth,
            Mode::Std(to),
            ASC_NAME,
            tags,
            payload,
        ).await?)
    }
}
