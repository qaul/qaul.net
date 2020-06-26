use super::*;
use libqaul::{
    contacts::ContactEntry,
    messages::{Message, MsgId, MsgRef},
    users::{UserAuth, UserProfile},
    Identity,
};

#[cfg(feature = "chat")]
use qaul_chat::{Chat, ChatMessage, RoomId, RoomMeta};

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
    // =^-^= generic message results =^-^=
    Success,
    Error(String),

    // =^-^= authentication data =^-^=
    Auth(UserAuth),

    // =^-^= additional user contact data =^-^=
    Contact(ContactEntry),
    Contacts(Vec<ContactEntry>),

    // =^-^= binary payload messages =^-^=
    Message(Message),
    Messages(Vec<Message>),

    // =^-^= a single message id =^-^=
    MsgId(MsgId),

    // =^-^= one or many chat messages =^-^=
    #[cfg(feature = "chat")]
    ChatMessage(ChatMessage),
    #[cfg(feature = "chat")]
    ChatMessages(Vec<ChatMessage>),

    // =^-^= one or many chat rooms =^-^=
    #[cfg(feature = "chat")]
    ChatRoom(RoomMeta),
    #[cfg(feature = "chat")]
    ChatRooms(Vec<RoomMeta>),

    // =^-^= one or many chat room ids =^-^=
    #[cfg(feature = "chat")]
    RoomId(RoomId),
    #[cfg(feature = "chat")]
    RoomIds(Vec<RoomId>),

    // =^-^= a single subscription id =^-^=
    Subscription(SubId),

    // =^-^= one or mary user profiles =^-^=
    User(UserProfile),
    Users(Vec<UserProfile>),

    // =^-^= one or many user ids =^-^=
    UserId(Identity),
    UserIds(Vec<Identity>),

    // =^-^= a single call id =^-^=
    #[cfg(feature = "voice")]
    CallId(CallId),

    // =^-^= one or many voice callls =^-^=
    #[cfg(feature = "voice")]
    Call(Call),
    #[cfg(feature = "voice")]
    Calls(Vec<Call>),

    // =^-^= a single voice call event =^-^=
    #[cfg(feature = "voice")]
    CallEvent(CallEvent),
}

////////////////////////////////////////////////////////////////////////////////////
//////////////// Mapping libqaul responses to the Response envelope ////////////////
//
//
// This is essentially the primary mapper between the libqaul types
// and the fascade of types that is exposed.  For any type there
// should be a singular and a plural mapping, where the plural is
// simply a Vec<T> of whatever the main type is.
//
// When editing this file, try to cluster use cases together and
// insert comments between sections to make it clear where things
// start and end.

////
//// =^-^= Handle nested responses with errors and generic auth
////

impl From<()> for Response {
    fn from(_: ()) -> Self {
        Self::Success
    }
}

impl From<UserAuth> for Response {
    fn from(auth: UserAuth) -> Self {
        Response::Auth(auth)
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

////
//// =^-^= Contact entry responses
////

impl From<ContactEntry> for Response {
    fn from(contact: ContactEntry) -> Self {
        Response::Contact(contact)
    }
}

impl From<Vec<ContactEntry>> for Response {
    fn from(contacts: Vec<ContactEntry>) -> Self {
        Response::Contacts(contacts)
    }
}

////
//// =^-^= Binary message responses
////

impl From<MsgRef> for Response {
    fn from(msg: MsgRef) -> Self {
        Response::Message(msg.as_ref().clone())
    }
}

impl From<Vec<MsgRef>> for Response {
    fn from(msgs: Vec<MsgRef>) -> Self {
        Response::Messages(msgs.into_iter().map(|msg| msg.as_ref().clone()).collect())
    }
}

////
//// =^-^= User profile data and id responses
////

impl From<UserProfile> for Response {
    fn from(user: UserProfile) -> Self {
        Response::User(user)
    }
}

impl From<Vec<UserProfile>> for Response {
    fn from(users: Vec<UserProfile>) -> Self {
        Response::Users(users)
    }
}

impl From<Identity> for Response {
    fn from(id: Identity) -> Self {
        Self::UserId(id)
    }
}

impl From<Vec<Identity>> for Response {
    fn from(ids: Vec<Identity>) -> Self {
        Self::UserIds(ids)
    }
}

////
//// =^-^= Chat service respenses
////

#[cfg(feature = "chat")]
impl From<RoomMeta> for Response {
    fn from(room: RoomMeta) -> Self {
        Response::ChatRoom(room)
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
        Response::ChatMessage(msg)
    }
}

#[cfg(feature = "chat")]
impl From<Vec<ChatMessage>> for Response {
    fn from(msgs: Vec<ChatMessage>) -> Self {
        Response::ChatMessages(msgs)
    }
}

////
//// =^-^= Voice service responses
////

#[cfg(feature = "voice")]
impl From<Call> for Response {
    fn from(call: Call) -> Self {
        Response::Call(call)
    }
}

#[cfg(feature = "voice")]
impl From<Vec<Call>> for Response {
    fn from(calls: Vec<Call>) -> Self {
        Response::Calls(calls)
    }
}

#[cfg(feature = "voice")]
impl From<CallEvent> for Response {
    fn from(call_event: CallEvent) -> Self {
        Response::CallEvent(call_event)
    }
}
