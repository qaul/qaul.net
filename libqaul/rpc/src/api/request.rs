use {
    serde::{Serialize, Deserialize},
    super::*,
};
#[feature(chat)]
use qaul_chat::Chat;

/// In some RPC systems requests will be processed in a non-deterministic
/// order, making it hard to associtate requests with responses. In such
/// systems clients are recommended to provide a transaction id with their
/// requests. This id will be returned along with the response to help 
/// correlation.
#[derive(Serialize, Deserialize)]
pub struct TransactionRequest {
    #[serde(default)]
    pub transaction_id: Option<String>,
    pub request: Request,
}

impl TransactionRequest {
    /// Split this incoming request into a `Request` and the required
    /// context to add on to the outgoing `Response`
    pub fn split(self) -> (Request, response::ResponseContext) {
        let TransactionRequest { transaction_id, request } = self;
        let transaction_request = match transaction_id {
            Some(_) => None,
            None => Some(request.clone()),
        };
        (
            request,
            response::ResponseContext {
                transaction_id,
                request: transaction_request,
            }, 
        )
    }
}

/// This absolutely massive enum is for protocols like `ws` and `ipc`
/// that will tunnel all kinds of messages through the same channel.
/// Things like `http` should use the direct RPC structs instead.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Request {
    #[feature(chat)]
    ChatMessageNext(chat::messages::Next),
    #[feature(chat)]
    ChatMessageSubscribe(chat::messages::Subscribe),
    #[feature(chat)]
    ChatMessageSend(chat::messages::Send),

    #[feature(chat)]
    ChatRoomList(chat::rooms::List),
    #[feature(chat)]
    ChatRoomGet(chat::rooms::Get),
    #[feature(chat)]
    ChatRoomCreate(chat::rooms::Create),
    #[feature(chat)]
    ChatRoomModify(chat::rooms::Modify),
    #[feature(chat)]
    ChatRoomDelete(chat::rooms::Delete),

    ContactModify(contacts::Modify),
    ContactGet(contacts::Get),
    ContactQuery(contacts::Query),
    ContactAll(contacts::All),

    MessageSend(messages::Send),
    MessagePoll(messages::Poll),
    MessageSubscribe(messages::Subscribe),
    MessageQuery(messages::Query),

    UserList(users::List),
    UserCreate(users::Create),
    UserDelete(users::Delete),
    UserChangePw(users::ChangePw),
    UserLogin(users::Login),
    UserLogout(users::Logout),
    UserGet(users::Get),
    UserUpdate(users::Update),
}
