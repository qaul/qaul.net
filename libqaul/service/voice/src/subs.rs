use {
    crate::{Call, CallEvent, VoiceData},
    futures::channel::mpsc::Receiver,
};

pub type InvitationSubscription = Receiver<Call>;

pub type CallEventSubscription = Receiver<CallEvent>;

pub type VoiceSubscription = Receiver<VoiceData>;
