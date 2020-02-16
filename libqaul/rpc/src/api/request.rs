use {
    failure::{format_err, Fail},
    libqaul::Qaul,
    serde::{Serialize, Deserialize},
    std::sync::Arc,
    super::*,
};
#[feature(chat)]
use qaul_chat::Chat;

/// This absolutely massive enum is for protocols like `ws` and `ipc`
/// that will tunnel all kinds of messages through the same channel.
/// Things like `http` should use the direct RPC structs instead.
#[derive(Serialize, Deserialize)]
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

/// Due to the way we are locking services behind features we can't
/// actually implement `QaulRPC` on `Request`, or at least it'd
/// be a bit painful to use. This struct wraps a series of services
/// and allows `Request` objects to be executed on them.
#[derive(Clone)]
pub struct RequestExecutor {
    #[feature(chat)]
    pub chat: Option<Arc<Chat>>,
    pub qaul: Arc<Qaul>,
}

impl RequestExecutor {
    pub fn new(qaul: Arc<Qaul>) -> Self {
        Self {
            #[feature(chat)]
            chat: None,
            qaul,
        }
    }

    #[feature(chat)]
    pub fn chat(&mut self, chat: Arc<Chat>) {
        self.chat = Some(chat);
    }

    /// Execute the request on the appropriate service
    ///
    /// This returns an opaque type, if you need a concrete type please
    /// pull out the `Request` variant you care about and handle that one
    /// specially
    pub fn execute(&self, req: Request) {
        unimplemented!();
    }
}
