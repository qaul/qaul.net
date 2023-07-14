/// router rpc message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Router {
    #[prost(oneof = "router::Message", tags = "1, 2, 3, 4, 5, 6")]
    pub message: ::core::option::Option<router::Message>,
}
/// Nested message and enum types in `Router`.
pub mod router {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        RoutingTableRequest(super::RoutingTableRequest),
        #[prost(message, tag = "2")]
        RoutingTable(super::RoutingTableList),
        #[prost(message, tag = "3")]
        ConnectionsRequest(super::ConnectionsRequest),
        #[prost(message, tag = "4")]
        ConnectionsList(super::ConnectionsList),
        #[prost(message, tag = "5")]
        NeighboursRequest(super::NeighboursRequest),
        #[prost(message, tag = "6")]
        NeighboursList(super::NeighboursList),
    }
}
/// UI request for routing table list
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingTableRequest {}
/// Routing table list
/// This table presents the best view for each user.
/// It represents the decision the router takes
/// when sending and routing packages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingTableList {
    #[prost(message, repeated, tag = "1")]
    pub routing_table: ::prost::alloc::vec::Vec<RoutingTableEntry>,
}
/// Routing table user entry
/// This message contains the best connection to this
/// user per module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingTableEntry {
    #[prost(bytes = "vec", tag = "1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub connections: ::prost::alloc::vec::Vec<RoutingTableConnection>,
}
/// Routing table connection entry.
/// This message contains a connection to a specific user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingTableConnection {
    /// the connection module (LAN, Internet, BLE, etc.)
    #[prost(enumeration = "ConnectionModule", tag = "2")]
    pub module: i32,
    /// the round trip time for this connection
    #[prost(uint32, tag = "3")]
    pub rtt: u32,
    /// hop count
    #[prost(uint32, tag = "5")]
    pub hop_count: u32,
    /// node id via which this connection is routed
    #[prost(bytes = "vec", tag = "4")]
    pub via: ::prost::alloc::vec::Vec<u8>,
}
/// UI request for connections list
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionsRequest {}
/// Connections list per module.
/// All connections per user per module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionsList {
    /// users connected via the LAN module
    #[prost(message, repeated, tag = "1")]
    pub lan: ::prost::alloc::vec::Vec<ConnectionsUserEntry>,
    /// users connected via the Internet module
    #[prost(message, repeated, tag = "2")]
    pub internet: ::prost::alloc::vec::Vec<ConnectionsUserEntry>,
    /// users connected via the BLE module
    #[prost(message, repeated, tag = "3")]
    pub ble: ::prost::alloc::vec::Vec<ConnectionsUserEntry>,
    /// users connected locally (on the same node)
    #[prost(message, repeated, tag = "4")]
    pub local: ::prost::alloc::vec::Vec<ConnectionsUserEntry>,
}
/// connections entry for a user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionsUserEntry {
    /// the id of the user
    #[prost(bytes = "vec", tag = "1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// all connections to this user via this module
    #[prost(message, repeated, tag = "2")]
    pub connections: ::prost::alloc::vec::Vec<ConnectionEntry>,
}
/// all connections of this user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionEntry {
    /// round trip time in milli seconds
    #[prost(uint32, tag = "1")]
    pub rtt: u32,
    /// hop count to the user.
    /// This represents the number of nodes between this node and the user.
    #[prost(uint32, tag = "2")]
    pub hop_count: u32,
    /// connection can be established via the node with the following id
    #[prost(bytes = "vec", tag = "3")]
    pub via: ::prost::alloc::vec::Vec<u8>,
}
/// UI request for neighbours list
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeighboursRequest {}
/// neighbours list per module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeighboursList {
    #[prost(message, repeated, tag = "1")]
    pub lan: ::prost::alloc::vec::Vec<NeighboursEntry>,
    #[prost(message, repeated, tag = "2")]
    pub internet: ::prost::alloc::vec::Vec<NeighboursEntry>,
    #[prost(message, repeated, tag = "3")]
    pub ble: ::prost::alloc::vec::Vec<NeighboursEntry>,
}
/// neighbours entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeighboursEntry {
    /// the ID of the neighbour node
    #[prost(bytes = "vec", tag = "1")]
    pub node_id: ::prost::alloc::vec::Vec<u8>,
    /// rtt to this neighbour
    #[prost(uint32, tag = "2")]
    pub rtt: u32,
}
/// Connection modules
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConnectionModule {
    None = 0,
    Lan = 1,
    Internet = 2,
    Ble = 3,
    Local = 4,
}
impl ConnectionModule {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionModule::None => "NONE",
            ConnectionModule::Lan => "LAN",
            ConnectionModule::Internet => "INTERNET",
            ConnectionModule::Ble => "BLE",
            ConnectionModule::Local => "LOCAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "LAN" => Some(Self::Lan),
            "INTERNET" => Some(Self::Internet),
            "BLE" => Some(Self::Ble),
            "LOCAL" => Some(Self::Local),
            _ => None,
        }
    }
}
