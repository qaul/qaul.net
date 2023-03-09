/// users rpc message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Users {
    #[prost(oneof = "users::Message", tags = "1, 2, 3, 4, 5, 6")]
    pub message: ::core::option::Option<users::Message>,
}
/// Nested message and enum types in `Users`.
pub mod users {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// User Request returns a user list
        /// containing all users with their connectivity
        /// field set to either online or offline.
        /// The connections are not set.
        #[prost(message, tag = "1")]
        UserRequest(super::UserRequest),
        /// User Online Request returns a user list
        /// of all users currently online in the network.
        /// Each user has
        #[prost(message, tag = "2")]
        UserOnlineRequest(super::UserOnlineRequest),
        /// User List
        ///
        /// Libqaul's return message for  'UserRequest' and
        /// 'UserOnlineRequest', containing a list of UserEntry's
        #[prost(message, tag = "3")]
        UserList(super::UserList),
        /// User Update
        ///
        /// Sent to libqaul to update the verification & blocked fields
        /// of a user.
        /// All other fields will be ignored.
        #[prost(message, tag = "4")]
        UserUpdate(super::UserEntry),
        /// Security Number Request
        ///
        /// Requests the specific security number for
        /// for the connection with this user.
        #[prost(message, tag = "5")]
        SecurityNumberRequest(super::SecurityNumberRequest),
        /// Security Number Response
        ///
        /// Libqaul's response containing the security number.
        ///
        /// The security number contains 8 blocks of 5 digit numbers.
        /// They shall be rendered in two rows. If a number is
        /// smaller then five-digits, the missing digits shall be filled
        /// with leading zeros.
        ///
        /// example rendering of security number:
        /// 13246 42369 46193 12484
        /// 12142 31101 09874 34545
        #[prost(message, tag = "6")]
        SecurityNumberResponse(super::SecurityNumberResponse),
    }
}
/// UI request for some users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRequest {}
/// UI request for some online users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOnlineRequest {}
/// user list
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserList {
    #[prost(message, repeated, tag = "1")]
    pub user: ::prost::alloc::vec::Vec<UserEntry>,
}
/// user entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEntry {
    /// user name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// user ID (38 Byte PeerID)
    #[prost(bytes = "vec", tag = "2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// direct chat group id
    ///
    /// this is a predictable 16 bytes UUID
    #[prost(bytes = "vec", tag = "3")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// base58 string of public key
    #[prost(string, tag = "7")]
    pub key_base58: ::prost::alloc::string::String,
    /// reachability of the user: online | reachable | offline
    #[prost(enumeration = "Connectivity", tag = "8")]
    pub connectivity: i32,
    /// user has been verified
    #[prost(bool, tag = "9")]
    pub verified: bool,
    /// user is blocked
    #[prost(bool, tag = "10")]
    pub blocked: bool,
    /// routing connection entries
    /// RoutingTableConnection connections = 11;
    #[prost(message, repeated, tag = "11")]
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
/// security number request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityNumberRequest {
    /// user id
    #[prost(bytes = "vec", tag = "1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// security number response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityNumberResponse {
    /// the user id of the remote user
    #[prost(bytes = "vec", tag = "1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// deliver the full bytes of the hash
    #[prost(bytes = "vec", tag = "2")]
    pub security_hash: ::prost::alloc::vec::Vec<u8>,
    /// fill in 8 numbers of 16bits
    /// uint16 data type does not exist in protobuf, just fill them in the u16 as
    /// u32.
    #[prost(uint32, repeated, tag = "3")]
    pub security_number_blocks: ::prost::alloc::vec::Vec<u32>,
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
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Online" => Some(Self::Online),
            "Reachable" => Some(Self::Reachable),
            "Offline" => Some(Self::Offline),
            _ => None,
        }
    }
}
