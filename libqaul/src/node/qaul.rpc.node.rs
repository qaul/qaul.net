/// node rpc message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(oneof="node::Message", tags="1, 2")]
    pub message: ::core::option::Option<node::Message>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(bool, tag="1")]
        GetNodeInfo(bool),
        #[prost(message, tag="2")]
        Info(super::NodeInformation),
    }
}
/// node information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInformation {
    #[prost(string, tag="1")]
    pub id_base58: ::prost::alloc::string::String,
}
