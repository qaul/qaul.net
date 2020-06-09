use super::*;
use libqaul::{
    contacts::ContactEntry,
    messages::{Message, MsgId, MsgRef},
    users::{UserAuth, UserProfile},
    Identity,
};

#[cfg(feature = "chat")]
use qaul_chat::{Chat, ChatMessage, Room, RoomId, RoomMeta};

#[cfg(feature = "voice")]
use qaul_voice::{Call, CallEvent, CallId};

use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

/// Represents a libqaul RPC request envelope
///
/// Because in some rpc systems requests will be processed in a
/// non-knowable order, making it hard to associtate requests with
/// responses.  This is what the request ID is for, and should be set,
/// even on systems that don't have this problem.
#[derive(Clone, Serialize, Deserialize)]
pub struct Envelope<D> {
    pub id: String,
    pub data: D,
}

/// A wrapper enum to disambiguate request types in the envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Request {
    // =^-^= Chat Messages =^-^=
    #[cfg(feature = "chat")]
    ChatMsgSub(chat::messages::Subscribe),
    #[cfg(feature = "chat")]
    ChatMsgCreate(chat::messages::Create),
    #[cfg(feature = "chat")]
    ChatLoadRoom(chat::messages::Query),

    // =^-^= Chat Rooms =^-^=
    #[cfg(feature = "chat")]
    ChatRoomList(chat::rooms::List),
    #[cfg(feature = "chat")]
    ChatRoomGet(chat::rooms::Get),
    #[cfg(feature = "chat")]
    ChatRoomCreate(chat::rooms::Create),
    #[cfg(feature = "chat")]
    ChatRoomModify(chat::rooms::Modify),

    // =^-^= Contacts =^-^=
    ContactModify(contacts::Modify),
    ContactGet(contacts::Get),
    ContactQuery(contacts::Query),
    ContactAll(contacts::All),

    // =^-^= Generic/ low level commands =^-^=
    MsgSend(messages::Send),
    CancelSub(streamer::CancelSub),

    // =^-^= Users =^-^=
    UserList(users::List),
    UserListRemote(users::ListRemote),
    UserIsAuthenticated(users::IsAuthenticated),
    UserCreate(users::Create),
    UserDelete(users::Delete),
    UserChangePw(users::ChangePw),
    UserLogin(users::Login),
    UserLogout(users::Logout),
    UserGet(users::Get),
    UserUpdate(users::Update),

    // =^-^= Voice calls =^-^=
    #[cfg(feature = "voice")]
    VoiceStartCall(voice::call_state::StartCall),
    #[cfg(feature = "voice")]
    VoiceGetCalls(voice::call_state::GetCalls),
    #[cfg(feature = "voice")]
    VoiceGetCall(voice::call_state::GetCall),
    #[cfg(feature = "voice")]
    VoiceInviteToCall(voice::call_state::InviteToCall),
    #[cfg(feature = "voice")]
    VoiceJoinCall(voice::call_state::JoinCall),
    #[cfg(feature = "voice")]
    VoiceLeaveCall(voice::call_state::LeaveCall),
    #[cfg(feature = "voice")]
    VoiceSubscribeInvites(voice::call_state::SubscribeInvites),
    #[cfg(feature = "voice")]
    VoiceSubscribeCallEvents(voice::call_state::SubscribeCallEvents),
}

/// Wrap around all possible response values for piped Rpc protocols
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Response {
    /// Return an auth object
    Auth(UserAuth),

    /// Return a set of chat messages
    #[cfg(feature = "chat")]
    ChatMessage(Vec<ChatMessage>),

    /// Return a set of contact entries
    Contact(Vec<ContactEntry>),

    /// Return an error type
    Error(String),

    /// Return a set of message
    Message(Vec<Message>),

    /// Return a message ID
    MsgId(MsgId),

    /// Return chat room data
    #[cfg(feature = "chat")]
    ChatRoom(Room),

    /// Get a set of chat room IDs
    #[cfg(feature = "chat")]
    RoomId(Vec<RoomId>),

    /// Get a list of all chat rooms with read / unread messages
    #[cfg(feature = "chat")]
    ChatRooms(Vec<RoomMeta>),

    /// Confirmation for a new subscription
    Subscription(SubId),

    /// A generic success message
    Success,

    /// Return a set of user profiles
    User(Vec<UserProfile>),

    /// Return available user IDs
    UserId(Vec<Identity>),

    #[cfg(feature = "voice")]
    CallId(CallId),

    #[cfg(feature = "voice")]
    Call(Vec<Call>),

    #[cfg(feature = "voice")]
    CallEvent(CallEvent),
}

impl From<UserAuth> for Response {
    fn from(auth: UserAuth) -> Self {
        Response::Auth(auth)
    }
}

#[cfg(feature = "chat")]
impl From<RoomMeta> for Response {
    fn from(room: RoomMeta) -> Self {
        Response::ChatRooms(vec![room])
    }
}

#[cfg(feature = "chat")]
impl From<Vec<RoomMeta>> for Response {
    fn from(rooms: Vec<RoomMeta>) -> Self {
        Response::ChatRooms(rooms)
    }
}

#[cfg(feature = "chat")]
impl From<ChatMessage> for Response {
    fn from(msg: ChatMessage) -> Self {
        Response::ChatMessage(vec![msg])
    }
}

#[cfg(feature = "chat")]
impl From<Vec<ChatMessage>> for Response {
    fn from(msgs: Vec<ChatMessage>) -> Self {
        Response::ChatMessage(msgs)
    }
}

impl From<ContactEntry> for Response {
    fn from(contact: ContactEntry) -> Self {
        Response::Contact(vec![contact])
    }
}

impl From<Vec<ContactEntry>> for Response {
    fn from(contacts: Vec<ContactEntry>) -> Self {
        Response::Contact(contacts)
    }
}

impl<T: Into<Response>, E: Display> From<Result<T, E>> for Response {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(t) => t.into(),
            Err(e) => Response::Error(e.to_string()),
        }
    }
}

impl From<MsgRef> for Response {
    fn from(msg: MsgRef) -> Self {
        Response::Message(vec![msg.as_ref().clone()])
    }
}

impl From<Vec<MsgRef>> for Response {
    fn from(msgs: Vec<MsgRef>) -> Self {
        Response::Message(msgs.into_iter().map(|msg| msg.as_ref().clone()).collect())
    }
}

#[cfg(feature = "chat")]
impl From<Room> for Response {
    fn from(room: Room) -> Self {
        Response::ChatRoom(room)
    }
}

impl From<()> for Response {
    fn from(_: ()) -> Self {
        Self::Success
    }
}

impl From<UserProfile> for Response {
    fn from(user: UserProfile) -> Self {
        Response::User(vec![user])
    }
}

impl From<Vec<UserProfile>> for Response {
    fn from(users: Vec<UserProfile>) -> Self {
        Response::User(users)
    }
}

#[cfg(feature = "voice")]
impl From<Call> for Response {
    fn from(call: Call) -> Self {
        Response::Call(vec![call])
    }
}

#[cfg(feature = "voice")]
impl From<Vec<Call>> for Response {
    fn from(calls: Vec<Call>) -> Self {
        Response::Call(calls)
    }
}

#[cfg(feature = "voice")]
impl From<CallEvent> for Response {
    fn from(call_event: CallEvent) -> Self {
        Response::CallEvent(call_event)
    }
}
