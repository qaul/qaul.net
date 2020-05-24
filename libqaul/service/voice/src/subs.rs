use {
    crate::{Call, CallEvent},
    futures::channel::mpsc::Receiver,
};

pub type InvitationSubscription = Receiver<Call>;

pub type CallEventSubscription = Receiver<CallEvent>;
