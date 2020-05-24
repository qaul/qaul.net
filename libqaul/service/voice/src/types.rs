use async_std::sync::{Arc, RwLock, Sender};
use libqaul::{users::UserAuth, Identity};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::atomic::{AtomicUsize, Ordering},
};

pub type CallId = Identity;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Call {
    pub id: CallId,
    /// Who has joined the call?
    pub participants: BTreeSet<Identity>,
    /// Who has been invited to the call?
    pub invitees: BTreeSet<Identity>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) enum CallMessage {
    /// invite a user
    Invitation(CallInvitation),
    /// note that you have invited a user
    InvitationSent(Identity),
    /// join a call
    Join,
    /// leave a call
    Part,
    /// send some data to the call
    Data(CallData),
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

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CallEvent {
    UserInvited(Identity),
    UserJoined(Identity),
    UserParted(Identity),
}

pub(crate) struct CallUser {
    pub(crate) auth: UserAuth,
    pub(crate) invitation_subs: RwLock<BTreeMap<usize, Sender<Call>>>,
    pub(crate) call_event_subs: RwLock<BTreeMap<CallId, BTreeMap<usize, Sender<CallEvent>>>>,
    sub_id: AtomicUsize,
}

impl CallUser {
    pub(crate) fn new(auth: UserAuth) -> Arc<Self> {
        Arc::new(Self {
            auth,
            invitation_subs: Default::default(),
            call_event_subs: Default::default(),
            sub_id: 0.into(),
        })
    }

    pub(crate) async fn add_invitation_sub(&self, sender: Sender<Call>) -> usize {
        let id = self.sub_id.fetch_add(1, Ordering::Relaxed);
        self.invitation_subs.write().await.insert(id, sender);
        id
    }

    pub(crate) async fn add_event_sub(&self, call_id: CallId, sender: Sender<CallEvent>) -> usize {
        let id = self.sub_id.fetch_add(1, Ordering::Relaxed);
        self.call_event_subs
            .write()
            .await
            .entry(call_id)
            .or_default()
            .insert(id, sender);
        id
    }
}
