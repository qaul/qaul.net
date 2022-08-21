/// users rpc message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Users {
    #[prost(oneof="users::Message", tags="1, 2, 3, 4")]
    pub message: ::core::option::Option<users::Message>,
}
/// Nested message and enum types in `Users`.
pub mod users {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        UserRequest(super::UserRequest),
        #[prost(message, tag="2")]
        UserOnlineRequest(super::UserOnlineRequest),
        #[prost(message, tag="3")]
        UserList(super::UserList),
        #[prost(message, tag="4")]
        UserUpdate(super::UserEntry),
    }
}
/// UI request for some users
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRequest {
}
/// UI request for some online users
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOnlineRequest {
}
/// user list
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserList {
    #[prost(message, repeated, tag="1")]
    pub user: ::prost::alloc::vec::Vec<UserEntry>,
}
/// user entry
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEntry {
    /// user name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// user ID (38 Byte PeerID)
    #[prost(bytes="vec", tag="2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// direct chat conversation ID
    ///
    /// this is a predictable 16 bytes UUID
    #[prost(bytes="vec", tag="3")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// base58 string of public key
    #[prost(string, tag="7")]
    pub key_base58: ::prost::alloc::string::String,
    /// reachability of the user: online | reachable | offline
    #[prost(enumeration="Connectivity", tag="8")]
    pub connectivity: i32,
    /// user has been verified
    #[prost(bool, tag="9")]
    pub verified: bool,
    /// user is blocked
    #[prost(bool, tag="10")]
    pub blocked: bool,
}
/// how is the user connected
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Connectivity {
    /// The user is actively connected to the node
    /// and reachable for synchronous communication.
    Online = 0,
    /// The node which hosts the user account is online
    /// but the user is not actively connected to it.
    /// Messages can sent and will reach the node.
    Reachable = 1,
    /// The user is currently not reachable.
    Offline = 2,
}
impl Connectivity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Connectivity::Online => "Online",
            Connectivity::Reachable => "Reachable",
            Connectivity::Offline => "Offline",
        }
    }
}
