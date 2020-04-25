use {
    crate::{
        api::{CallId, StreamMetadata},
        Voices, ASC_NAME,
    },
    conjoiner,
    failure::Error,
    libqaul::{
        helpers::Tag,
        messages::{Mode, MsgId},
        users::UserAuth,
        Identity,
    },
    serde::{Deserialize, Serialize},
};

/// A packet of audio data
#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    /// A monotonically increasing sequence number used for ordering packets
    /// on arrival
    pub sequence_number: u32,
    /// The actual OPUS encoded audio data of the connection
    pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Differentiates the various kinds of voice messages
pub enum VoiceMessageKind {
    /// An incoming call
    Incoming(StreamMetadata),
    /// The call was accepted by the remote party
    Accept(StreamMetadata),
    /// The call was ended by the remote party
    HungUp,
    /// A packet of audio data
    Packet(Packet),
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
    pub async fn send(self, voices: &Voices, auth: UserAuth, to: Identity) -> Result<MsgId, Error> {
        let payload = conjoiner::serialise(&self).unwrap();
        let tags = vec![
            Tag::new("call_id", self.call),
            match self.kind {
                VoiceMessageKind::Incoming(_) => Tag::new("kind", b"incoming".to_vec()),
                VoiceMessageKind::Accept(_) | VoiceMessageKind::HungUp => {
                    Tag::new("kind", b"control".to_vec())
                }
                VoiceMessageKind::Packet(_) => Tag::new("kind", b"packet".to_vec()),
            },
        ];
        Ok(voices
            .qaul
            .messages()
            .send(auth, Mode::Std(to), ASC_NAME, tags, payload)
            .await?)
    }
}
