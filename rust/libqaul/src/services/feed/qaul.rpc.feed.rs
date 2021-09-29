/// Feed service RPC message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// message type
    #[prost(oneof="feed::Message", tags="1, 2, 3")]
    pub message: ::core::option::Option<feed::Message>,
}
/// Nested message and enum types in `Feed`.
pub mod feed {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// received messages
        #[prost(message, tag="1")]
        Received(super::FeedMessageList),
        /// send a new feed message
        #[prost(message, tag="2")]
        Send(super::SendMessage),
        /// request received messages
        #[prost(message, tag="3")]
        Request(super::FeedMessageRequest),
    }
}
/// request feed messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessageRequest {
    /// message id of the last received message
    /// this can be empty, then all last messages
    /// are sent.
    #[prost(bytes="vec", tag="1")]
    pub last_received: ::prost::alloc::vec::Vec<u8>,
}
/// List of feed messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessageList {
    #[prost(message, repeated, tag="1")]
    pub feed_message: ::prost::alloc::vec::Vec<FeedMessage>,
}
/// A single feed message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessage {
    #[prost(bytes="vec", tag="1")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub sender_id_base58: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub message_id_base58: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub time_sent: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub time_received: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub content: ::prost::alloc::string::String,
}
/// send feed message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessage {
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
}
