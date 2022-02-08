/// Qaul Feed Network Message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// signature
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// message content
    #[prost(bytes="vec", tag="2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
/// Feed Message Content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessageContent {
    /// sender
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    /// timestamp in milli seconds
    #[prost(uint64, tag="2")]
    pub time: u64,
    /// message content
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
