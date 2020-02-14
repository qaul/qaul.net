use {
    libqaul_rpc::{contacts, files, messages, users},
};

pub struct Transaction<T> {
    /// An optional client-provided ID
    ///
    /// This keeps track of which set of requests
    /// a given response is to. This does not have to be
    /// globally unique, it is solely for the client's reference
    id: Option<String>,

    /// The items that are a part of the transaction
    /// 
    /// These will either be the set of requests the client wishes to execute
    /// on `libqaul` or the set of responses to the requests sent by the client.
    ///
    /// Requests are executed asynchronously which is to say **order will not
    /// be preserved**. Responses will however be collected in the order the 
    /// requests were given in
    items: Vec<T>,
}

/// The various different kinds of requests a client could make
pub enum Request<'a> {
    MessageSend(messages::Send),
    MessagePoll(messages::Poll),
    MessageSubscribe(messages::Subscribe),
    MessageQuery(messages::Query),
    FileQuery(files::Query),
    FileList(files::List),
    ContactModify(contacts::Modify),
    ContactGet(contacts::Get),
    ContactQuery(contacts::Query<'a>),
    ContactAll(contacts::All),
    UserList,
    UserCreate(users::Create),
    UserDelete(users::Delete),
    UserChangePw(users::ChangePw),
    UserLogin(users::Login),
    UserLogout(users::Logout),
    UserGet(users::Get),
    UserUpdate(users::Update),
}
