/// Feed service RPC message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// message type
    #[prost(oneof = "feed::Message", tags = "1, 2, 3")]
    pub message: ::core::option::Option<feed::Message>,
}
/// Nested message and enum types in `Feed`.
pub mod feed {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// received messages
        #[prost(message, tag = "1")]
        Received(super::FeedMessageList),
        /// send a new feed message
        #[prost(message, tag = "2")]
        Send(super::SendMessage),
        /// request received messages
        #[prost(message, tag = "3")]
        Request(super::FeedMessageRequest),
    }
}
/// request feed messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessageRequest {
    /// DEPRECATED
    #[prost(bytes = "vec", tag = "1")]
    pub last_received: ::prost::alloc::vec::Vec<u8>,
    /// Index of the last message received
    ///
    /// The message index is a continues numbering
    /// of incoming messages in the database of the node.
    ///
    /// When this variable is set, only
    /// newer messages will be sent.
    /// Default value is 0, when the value
    /// is 0, all feed messages will be sent.
    #[prost(uint64, tag = "2")]
    pub last_index: u64,
}
/// List of feed messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessageList {
    #[prost(message, repeated, tag = "1")]
    pub feed_message: ::prost::alloc::vec::Vec<FeedMessage>,
}
/// A single feed message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// DEPRECATED
    #[prost(string, tag = "2")]
    pub sender_id_base58: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// DEPRECATED
    #[prost(string, tag = "4")]
    pub message_id_base58: ::prost::alloc::string::String,
    /// DEPRECATED
    #[prost(string, tag = "5")]
    pub time_sent: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    pub timestamp_sent: u64,
    /// DEPRECATED
    #[prost(string, tag = "6")]
    pub time_received: ::prost::alloc::string::String,
    #[prost(uint64, tag = "10")]
    pub timestamp_received: u64,
    #[prost(string, tag = "7")]
    pub content: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub index: u64,
}
/// send feed message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessage {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
