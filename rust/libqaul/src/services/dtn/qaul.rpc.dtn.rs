/// DTN service RPC message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dtn {
    /// message type
    #[prost(oneof="dtn::Message", tags="1, 2")]
    pub message: ::core::option::Option<dtn::Message>,
}
/// Nested message and enum types in `DTN`.
pub mod dtn {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// dtn state request
        #[prost(message, tag="1")]
        DtnStateRequest(super::DtnStateRequest),
        /// dtn state response
        #[prost(message, tag="2")]
        DtnStateResponse(super::DtnStateResponse),
    }
}
/// Dtn State Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnStateRequest {
}
/// Dtn State Response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnStateResponse {
    /// used size
    #[prost(uint64, tag="1")]
    pub used_size: u64,
    /// dtn message count
    #[prost(uint32, tag="2")]
    pub dtn_message_count: u32,
    /// unconfirmed count
    #[prost(uint32, tag="3")]
    pub unconfirmed_count: u32,
}
