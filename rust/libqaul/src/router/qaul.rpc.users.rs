/// users rpc message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Users {
    #[prost(oneof="users::Message", tags="1, 2, 3")]
    pub message: ::core::option::Option<users::Message>,
}
/// Nested message and enum types in `Users`.
pub mod users {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        UserRequest(super::UserRequest),
        #[prost(message, tag="2")]
        UserList(super::UserList),
        #[prost(message, tag="3")]
        UserUpdate(super::UserEntry),
    }
}
/// UI request for some users
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRequest {
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
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub id_base58: ::prost::alloc::string::String,
    /// protobuf encoded public key
    #[prost(bytes="vec", tag="5")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub key_type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub key_base58: ::prost::alloc::string::String,
    #[prost(enumeration="Connectivity", tag="8")]
    pub connectivity: i32,
    #[prost(bool, tag="9")]
    pub verified: bool,
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
