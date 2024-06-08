/// node rpc message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// message contains all node message types
    #[prost(oneof = "node::Message", tags = "1, 2")]
    pub message: ::core::option::Option<node::Message>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    /// message contains all node message types
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// request node info message from libqaul
        #[prost(bool, tag = "1")]
        GetNodeInfo(bool),
        /// libqaul sends node info
        #[prost(message, tag = "2")]
        Info(super::NodeInformation),
    }
}
/// node information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInformation {
    /// the node ID in base 58 encoding
    #[prost(string, tag = "1")]
    pub id_base58: ::prost::alloc::string::String,
    /// all known multi addresses under which
    /// this node can be connected.
    #[prost(string, repeated, tag = "2")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
