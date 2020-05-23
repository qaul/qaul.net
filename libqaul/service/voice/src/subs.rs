use {
    crate::Call,
    futures::channel::mpsc::Receiver,
};

pub type InvitationSubscription = Receiver<Call>;
