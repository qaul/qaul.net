/// Connections rpc message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connections {
    #[prost(oneof="connections::Message", tags="1, 2, 3, 4")]
    pub message: ::core::option::Option<connections::Message>,
}
/// Nested message and enum types in `Connections`.
pub mod connections {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        InternetNodesRequest(super::InternetNodesRequest),
        #[prost(message, tag="2")]
        InternetNodesList(super::InternetNodesList),
        #[prost(message, tag="3")]
        InternetNodesAdd(super::InternetNodesEntry),
        #[prost(message, tag="4")]
        InternetNodesRemove(super::InternetNodesEntry),
    }
}
/// UI request for Internet nodes list
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternetNodesRequest {
}
/// Internet Nodes List
///
/// This is a list of all peer nodes the internet
/// connections module tries to connect to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternetNodesList {
    #[prost(message, repeated, tag="1")]
    pub nodes: ::prost::alloc::vec::Vec<InternetNodesEntry>,
}
/// Internet Nodes Entry
///
/// Contains a node address as a libp2p multiaddress.
/// e.g. "/ip4/144.91.74.192/tcp/9229"
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternetNodesEntry {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
