/// Router information Container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterInfoContainer {
    /// signature
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// message content
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
/// Router information content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterInfoContent {
    /// node id
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// RouterInfo Module
    #[prost(enumeration = "RouterInfoModule", tag = "2")]
    pub router_info_module: i32,
    /// message content
    #[prost(bytes = "vec", tag = "3")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// timestamp in milli seconds
    #[prost(uint64, tag = "4")]
    pub time: u64,
}
/// Router information message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterInfoMessage {
    /// node id
    #[prost(bytes = "vec", tag = "1")]
    pub node: ::prost::alloc::vec::Vec<u8>,
    /// Routing information table
    #[prost(message, optional, tag = "2")]
    pub routes: ::core::option::Option<RoutingInfoTable>,
    /// Latest Feed ids table
    #[prost(message, optional, tag = "4")]
    pub feeds: ::core::option::Option<FeedIdsTable>,
    /// timestamp
    #[prost(uint64, tag = "5")]
    pub timestamp: u64,
}
/// Routing information to send to neighbours
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInfoTable {
    #[prost(message, repeated, tag = "1")]
    pub entry: ::prost::alloc::vec::Vec<RoutingInfoEntry>,
}
/// Routing structures to send over the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInfoEntry {
    /// user id
    #[prost(bytes = "vec", tag = "1")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    /// round trip time
    #[prost(uint32, tag = "2")]
    pub rtt: u32,
    /// hop count
    #[prost(bytes = "vec", tag = "3")]
    pub hc: ::prost::alloc::vec::Vec<u8>,
    /// propagation id
    #[prost(uint32, tag = "5")]
    pub pgid: u32,
}
/// User information table
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdTable {
    /// user ids
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// User information table
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoTable {
    /// user info
    #[prost(message, repeated, tag = "1")]
    pub info: ::prost::alloc::vec::Vec<UserInfo>,
}
/// User info structure for sending to the neighbours
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// user id
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// public key of the user
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// user name
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// List of feed ID's
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedIdsTable {
    /// feed id
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Feed request message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedRequestMessage {
    /// Feed ids table
    #[prost(message, optional, tag = "1")]
    pub feeds: ::core::option::Option<FeedIdsTable>,
}
/// Feed response message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedResponseMessage {
    /// Feed table
    #[prost(message, optional, tag = "1")]
    pub feeds: ::core::option::Option<FeedResponseTable>,
}
/// Feed response table
/// containing the feed messages for response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedResponseTable {
    /// feed messages
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<FeedMessage>,
}
/// Feed Message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessage {
    /// message id
    #[prost(bytes = "vec", tag = "1")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// sender id
    #[prost(bytes = "vec", tag = "2")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// message content
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    /// timestamp in milli seconds
    #[prost(uint64, tag = "4")]
    pub time: u64,
}
/// RouterInfoModule
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouterInfoModule {
    /// Message is a common RouterInfoMessage
    RouterInfo = 0,
    /// Message is a FeedRequestMessage
    FeedRequest = 1,
    /// Message is a FeedResponseMessage
    FeedResponse = 2,
    /// Message is a UserRequestMessage
    UserRequest = 3,
    /// Message is a UserResponseMessage
    UserResponse = 4,
}
impl RouterInfoModule {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RouterInfoModule::RouterInfo => "ROUTER_INFO",
            RouterInfoModule::FeedRequest => "FEED_REQUEST",
            RouterInfoModule::FeedResponse => "FEED_RESPONSE",
            RouterInfoModule::UserRequest => "USER_REQUEST",
            RouterInfoModule::UserResponse => "USER_RESPONSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROUTER_INFO" => Some(Self::RouterInfo),
            "FEED_REQUEST" => Some(Self::FeedRequest),
            "FEED_RESPONSE" => Some(Self::FeedResponse),
            "USER_REQUEST" => Some(Self::UserRequest),
            "USER_RESPONSE" => Some(Self::UserResponse),
            _ => None,
        }
    }
}
