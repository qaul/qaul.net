//! # Protobuf qaul RPC Reference
//! 
//! This file is only here for reference.
//! It is created automatically during the build process by `prost-build`.
//! You can find it in the build directory. e.g.:
//! `target/debug/build/libqaul-{BUILD_ID}/out/qaul_rpc.pb.rs`

/// rpc message container from libqaul
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FromLibqaul {
    /// kind of rpc message
    #[prost(oneof="from_libqaul::Module", tags="1, 2, 3")]
    pub module: ::core::option::Option<from_libqaul::Module>,
}
/// Nested message and enum types in `FromLibqaul`.
pub mod from_libqaul {
    /// kind of rpc message
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Module {
        #[prost(message, tag="1")]
        Node(super::FromNode),
        #[prost(message, tag="2")]
        Router(super::FromRouter),
        #[prost(message, tag="3")]
        Feed(super::FromFeed),
    }
}
/// node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FromNode {
    /// message type
    #[prost(oneof="from_node::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<from_node::Type>,
}
/// Nested message and enum types in `FromNode`.
pub mod from_node {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        Session(super::SessionInformation),
        #[prost(message, tag="2")]
        MyUser(super::MyUser),
    }
}
/// Session Information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionInformation {
    #[prost(bool, tag="1")]
    pub user_exists: bool,
    #[prost(message, optional, tag="2")]
    pub my_user: ::core::option::Option<MyUser>,
}
/// Information about my user
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyUser {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
/// router info
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FromRouter {
    /// message type
    #[prost(oneof="from_router::Type", tags="1")]
    pub r#type: ::core::option::Option<from_router::Type>,
}
/// Nested message and enum types in `FromRouter`.
pub mod from_router {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        UserList(super::UserList),
    }
}
/// User List
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
    #[prost(bytes="vec", tag="3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `UserEntry`.
pub mod user_entry {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Connectivity {
        Online = 0,
        Reachable = 1,
        Offline = 2,
    }
}
/// feed service
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FromFeed {
    /// message type
    #[prost(oneof="from_feed::Type", tags="1")]
    pub r#type: ::core::option::Option<from_feed::Type>,
}
/// Nested message and enum types in `FromFeed`.
pub mod from_feed {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        Message(super::FeedMessage),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessage {
    #[prost(bytes="vec", tag="1")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub time_sent: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub time_received: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub content: ::prost::alloc::string::String,
}
/// rpc message container to libqaul
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToLibqaul {
    /// kind of rpc message
    #[prost(oneof="to_libqaul::Module", tags="1, 2, 3")]
    pub module: ::core::option::Option<to_libqaul::Module>,
}
/// Nested message and enum types in `ToLibqaul`.
pub mod to_libqaul {
    /// kind of rpc message
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Module {
        #[prost(message, tag="1")]
        Node(super::ToNode),
        #[prost(message, tag="2")]
        Router(super::ToRouter),
        #[prost(message, tag="3")]
        Feed(super::ToFeed),
    }
}
/// node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToNode {
    /// message type
    #[prost(oneof="to_node::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<to_node::Type>,
}
/// Nested message and enum types in `ToNode`.
pub mod to_node {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(bool, tag="1")]
        StartSession(bool),
        #[prost(message, tag="2")]
        CreateUser(super::CreateUser),
    }
}
/// create a new user on this node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUser {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// router info
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToRouter {
    #[prost(oneof="to_router::Type", tags="1")]
    pub r#type: ::core::option::Option<to_router::Type>,
}
/// Nested message and enum types in `ToRouter`.
pub mod to_router {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        RequestUsers(super::RequestUsers),
    }
}
/// request users
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestUsers {
}
/// feed service
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToFeed {
    #[prost(oneof="to_feed::Type", tags="1")]
    pub r#type: ::core::option::Option<to_feed::Type>,
}
/// Nested message and enum types in `ToFeed`.
pub mod to_feed {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        SendFeed(super::SendFeed),
    }
}
/// send feed message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendFeed {
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
}
