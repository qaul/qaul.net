/// The main libqaul RPC message container.
/// All RPC messages from and to libqaul are packed
/// into this container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QaulRpc {
    /// which module to approach
    #[prost(enumeration = "Modules", tag = "1")]
    pub module: i32,
    /// can be used to identify responses
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// authorisation
    /// binary user id
    #[prost(bytes = "vec", tag = "3")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// the protobuf encoded binary message data
    /// which is passed to the module.
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Identification to which module the message shall be
/// handed to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Modules {
    /// default value, when nothing is defined.
    /// drop this message
    None = 0,
    /// RPC related messages
    /// such as authorisation etc.
    Rpc = 1,
    /// node information
    Node = 2,
    /// user accounts on this node
    Useraccounts = 3,
    /// all users in the network
    Users = 4,
    /// routing information
    Router = 5,
    /// feed module handling
    ///
    /// send and retrieve feed messages
    Feed = 6,
    /// connection information to other nodes
    Connections = 7,
    /// debug information & settings
    Debug = 8,
    /// chat group handling
    ///
    /// manage chat groups and group invites
    Group = 9,
    /// chat module
    /// to send chat messages, get a
    /// conversation overiew and all
    /// messages within a conversation
    Chat = 10,
    /// all functions to send and manage
    /// files sent into a chat conversation
    Chatfile = 11,
    /// BLE module handling
    Ble = 12,
    /// Real Time Communication handling
    Rtc = 13,
    /// Delay Tolerant Networking
    Dtn = 14,
}
impl Modules {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Modules::None => "NONE",
            Modules::Rpc => "RPC",
            Modules::Node => "NODE",
            Modules::Useraccounts => "USERACCOUNTS",
            Modules::Users => "USERS",
            Modules::Router => "ROUTER",
            Modules::Feed => "FEED",
            Modules::Connections => "CONNECTIONS",
            Modules::Debug => "DEBUG",
            Modules::Group => "GROUP",
            Modules::Chat => "CHAT",
            Modules::Chatfile => "CHATFILE",
            Modules::Ble => "BLE",
            Modules::Rtc => "RTC",
            Modules::Dtn => "DTN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "RPC" => Some(Self::Rpc),
            "NODE" => Some(Self::Node),
            "USERACCOUNTS" => Some(Self::Useraccounts),
            "USERS" => Some(Self::Users),
            "ROUTER" => Some(Self::Router),
            "FEED" => Some(Self::Feed),
            "CONNECTIONS" => Some(Self::Connections),
            "DEBUG" => Some(Self::Debug),
            "GROUP" => Some(Self::Group),
            "CHAT" => Some(Self::Chat),
            "CHATFILE" => Some(Self::Chatfile),
            "BLE" => Some(Self::Ble),
            "RTC" => Some(Self::Rtc),
            "DTN" => Some(Self::Dtn),
            _ => None,
        }
    }
}
