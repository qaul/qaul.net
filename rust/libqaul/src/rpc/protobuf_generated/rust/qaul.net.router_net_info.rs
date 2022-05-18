/// Router information Container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterInfoContainer {
    /// signature
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// message content
    #[prost(bytes="vec", tag="2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
/// Router information content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterInfoContent {
    /// node id
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// message content
    #[prost(bytes="vec", tag="2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// timestamp in milli seconds
    #[prost(uint64, tag="3")]
    pub time: u64,
}
/// Router information message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterInfoMessage {
    /// node id
    #[prost(bytes="vec", tag="1")]
    pub node: ::prost::alloc::vec::Vec<u8>,
    /// Routing information table
    #[prost(message, optional, tag="2")]
    pub routes: ::core::option::Option<RoutingInfoTable>,
    /// Users information table
    #[prost(message, optional, tag="3")]
    pub users: ::core::option::Option<UserInfoTable>,
    ///Latest Feed ids table
    #[prost(message, optional, tag="4")]
    pub feeds: ::core::option::Option<FeedIdsTable>,
    /// timestamp
    #[prost(uint64, tag="5")]
    pub timestamp: u64,
}
/// Routing information to send to neighbours
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInfoTable {
    #[prost(message, repeated, tag="1")]
    pub entry: ::prost::alloc::vec::Vec<RoutingInfoEntry>,
}
/// Routing structures to send over the network
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInfoEntry {
    //// user id
    #[prost(bytes="vec", tag="1")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    //// round trip time
    #[prost(uint32, tag="2")]
    pub rtt: u32,
    //// hop count
    #[prost(bytes="vec", tag="3")]
    pub hc: ::prost::alloc::vec::Vec<u8>,
    //// propagation id
    #[prost(uint32, tag="5")]
    pub pgid: u32,
}
/// User information table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoTable {
    #[prost(message, repeated, tag="1")]
    pub info: ::prost::alloc::vec::Vec<UserInfo>,
}
/// User info structure for sending to the neighbours
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// user id
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// public key of the user
    #[prost(bytes="vec", tag="2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// user name
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
///Feed ids table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedIdsTable {
    #[prost(bytes="vec", repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Router information message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedRequstMessage {
    ///Feed ids table
    #[prost(message, optional, tag="1")]
    pub feeds: ::core::option::Option<FeedIdsTable>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedResponseMessage {
    ///Feed ids table
    #[prost(message, optional, tag="1")]
    pub feeds: ::core::option::Option<FeedResponseTable>,
}
///Feed ids table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedResponseTable {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<FeedMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessage {
    //// sender id 
    #[prost(bytes="vec", repeated, tag="1")]
    pub sender_id: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    //// message content
    #[prost(string, tag="2")]
    pub content: ::prost::alloc::string::String,
    //// timestamp in milli seconds
    #[prost(uint64, tag="3")]
    pub time: u64,
}
