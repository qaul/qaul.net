/// Connections rpc message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connections {
    #[prost(oneof = "connections::Message", tags = "1, 2, 3, 4, 5, 6")]
    pub message: ::core::option::Option<connections::Message>,
}
/// Nested message and enum types in `Connections`.
pub mod connections {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Request a list of all internet nodes.
        /// libqaul returns an internet_nodes_list message.
        #[prost(message, tag = "1")]
        InternetNodesRequest(super::InternetNodesRequest),
        /// returns a list of all internet nodes and
        /// an information about why this message has been sent.
        #[prost(message, tag = "2")]
        InternetNodesList(super::InternetNodesList),
        /// Add a new internet node address.
        /// libqaul returns an internet_nodes_list message.
        #[prost(message, tag = "3")]
        InternetNodesAdd(super::InternetNodesEntry),
        /// Remove an internet node address.
        /// libqaul returns an internet_nodes_list message.
        #[prost(message, tag = "4")]
        InternetNodesRemove(super::InternetNodesEntry),
        /// Update an internet node state.
        /// libqaul returns an internet_nodes_list message.
        #[prost(message, tag = "5")]
        InternetNodesState(super::InternetNodesEntry),
        /// Rename internet node.
        /// libqaul returns an internet_nodes_list message.
        #[prost(message, tag = "6")]
        InternetNodesRename(super::InternetNodesEntry),
    }
}
/// UI request for Internet nodes list
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternetNodesRequest {}
/// Internet Nodes List
///
/// This is a list of all peer nodes the internet
/// connections module tries to connect to.
///
/// This message is returned after a request, or when
/// adding or removing a node address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternetNodesList {
    /// Information about why this message is sent
    /// and the result of the request, adding or removing
    /// of nodes.
    #[prost(enumeration = "Info", tag = "1")]
    pub info: i32,
    /// list of all node multiaddresses that
    /// the internet module will try to connect to.
    #[prost(message, repeated, tag = "2")]
    pub nodes: ::prost::alloc::vec::Vec<InternetNodesEntry>,
}
/// Internet Nodes Entry
///
/// Contains a node address as a libp2p multiaddress.
/// e.g. "/ip4/144.91.74.192/tcp/9229"
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternetNodesEntry {
    /// address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// enabled
    #[prost(bool, tag = "2")]
    pub enabled: bool,
    /// name
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Information about the system actions that led to
/// the creation of this message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Info {
    /// Internet Nodes Request
    /// By default, this message is sent due to an
    /// internet nodes request message.
    Request = 0,
    /// Add Internet Node
    /// Successfully added an address
    AddSuccess = 1,
    /// Error: not a valid multiaddress
    AddErrorInvalid = 2,
    /// Remove Internet Node
    /// Successfully removed the address
    RemoveSuccess = 5,
    /// Successfully changed state of the address
    StateSuccess = 6,
    /// Error: Address not found
    RemoveErrorNotFound = 7,
}
impl Info {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Info::Request => "REQUEST",
            Info::AddSuccess => "ADD_SUCCESS",
            Info::AddErrorInvalid => "ADD_ERROR_INVALID",
            Info::RemoveSuccess => "REMOVE_SUCCESS",
            Info::StateSuccess => "STATE_SUCCESS",
            Info::RemoveErrorNotFound => "REMOVE_ERROR_NOT_FOUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REQUEST" => Some(Self::Request),
            "ADD_SUCCESS" => Some(Self::AddSuccess),
            "ADD_ERROR_INVALID" => Some(Self::AddErrorInvalid),
            "REMOVE_SUCCESS" => Some(Self::RemoveSuccess),
            "STATE_SUCCESS" => Some(Self::StateSuccess),
            "REMOVE_ERROR_NOT_FOUND" => Some(Self::RemoveErrorNotFound),
            _ => None,
        }
    }
}
