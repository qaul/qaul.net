use {
    libqaul::{
        api::{SubId},
        contacts::{ContactEntry},
        messages::{MsgId, MsgRef, Message},
        users::{UserAuth, UserProfile},
        Identity,
    },
    serde::{Serialize, Deserialize},
    std::fmt::Display,
    super::request::Request,
};
#[feature(chat)]
use qaul_chat::{ChatMessage, room::{RoomId, Room}};

/// A wrapped response object carrying with it additional
/// information to allow correlating the response with a sent
/// request.
///
/// If the client provided a transaction id with the request
/// the respose will carry that same id. If the client did not
/// the response will carry the request sent by the client. As such
/// it is **highly** recommended clients include a transaction id *even*
/// if they don't care about correlation.
///
/// If this message is a response to an existing subscription instead 
/// the subscription id field will be filled.
#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    #[serde(default)]
    pub subscription_id: Option<SubId>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub request: Option<Request>,
    pub response: Response,
}

impl TransactionResponse {
    pub fn subscription(response: Response, sub_id: SubId) -> Self {
        Self {
            subscription_id: Some(sub_id),
            transaction_id: None,
            request: None,
            response,
        }
    }
}

/// A struct holding the required context to construct a `TransactionResponse`
/// with a minimum of copying
pub struct ResponseContext {
    pub transaction_id: Option<String>,
    pub request: Option<Request>,
}

impl ResponseContext {
    /// Create a transaction with this context and the given response
    pub fn with_response(self, response: Response) -> TransactionResponse {
        TransactionResponse {
            subscription_id: None,
            transaction_id: self.transaction_id,
            request: self.request,
            response
        }
    }
}

/// In some systems all responses are channeled over a single pipe. In
/// such systems, this object is provided to contain all possible responses.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Response {
    Auth(UserAuth),
    #[feature(chat)]
    ChatMessage(Vec<ChatMessage>),
    Contact(Vec<ContactEntry>),
    Error(String),
    Message(Vec<Message>),
    MessageId(MsgId),
    #[feature(chat)]
    Room(Room),
    #[feature(chat)]
    RoomId(Vec<RoomId>),
    Subscription(SubId),
    Success,
    User(Vec<UserProfile>),
    UserId(Vec<Identity>),
}

impl From<UserAuth> for Response {
    fn from(auth: UserAuth) -> Response {
        Response::Auth(auth)
    }
}

#[feature(chat)]
impl From<ChatMessage> for Response {
    fn from(msg: ChatMessage) -> Response {
        Response::ChatMessage(vec![msg])
    }
}

#[feature(chat)]
impl From<Vec<ChatMessage>> for Response {
    fn from(msgs: Vec<ChatMessage>) -> Response {
        Response::ChatMessage(msgs)
    }
}

impl From<ContactEntry> for Response {
    fn from(contact: ContactEntry) -> Response {
        Response::Contact(vec![contact])
    }
}

impl From<Vec<ContactEntry>> for Response {
    fn from(contacts: Vec<ContactEntry>) -> Response {
        Response::Contact(contacts)
    }
}

impl<T: Into<Response>, E: Display> From<Result<T, E> > for Response {
    fn from(result: Result<T, E>) -> Response {
        match result {
            Ok(t) => t.into(),
            Err(e) => Response::Error(e.to_string()),
        }
    }
}

impl From<MsgRef> for Response {
    fn from(msg: MsgRef) -> Response {
        Response::Message(vec![msg.as_ref().clone()])
    }
}

impl From<Vec<MsgRef>> for Response {
    fn from(msgs: Vec<MsgRef>) -> Response {
        Response::Message(
            msgs.into_iter()
                .map(|msg| msg.as_ref().clone())
                .collect()
        )
    }
}

#[feature(chat)]
impl From<Room> for Response {
    fn from(room: Room) -> Response {
        Response::Room(room)
    }
}

impl From<()> for Response {
    fn from(_: ()) -> Response {
        Response::Success
    }
}

impl From<UserProfile> for Response {
    fn from(user: UserProfile) -> Response {
        Response::User(vec![user])
    }
}

impl From<Vec<UserProfile>> for Response {
    fn from(users: Vec<UserProfile>) -> Response {
        Response::User(users)
    }
}
