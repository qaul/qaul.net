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
    /// timestamp
    #[prost(uint64, tag="4")]
    pub timestamp: u64,
}
/// 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Routing {
    #[prost(oneof="routing::Message", tags="1, 2, 3, 4")]
    pub message: ::core::option::Option<routing::Message>,
}
/// Nested message and enum types in `Routing`.
pub mod routing {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        UserInfoTable(super::UserInfoTable),
        #[prost(message, tag="2")]
        UserInfo(super::UserInfo),
        #[prost(message, tag="3")]
        RoutingInfoTable(super::RoutingInfoTable),
        #[prost(message, tag="4")]
        RoutingInfoEntry(super::RoutingInfoEntry),
    }
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
    //// package loss
    #[prost(float, tag="4")]
    pub pl: f32,
}
/// User information table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoTable {
    #[prost(message, repeated, tag="1")]
    pub info: ::prost::alloc::vec::Vec<UserInfo>,
}
/// User info structure for sending to the  
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
