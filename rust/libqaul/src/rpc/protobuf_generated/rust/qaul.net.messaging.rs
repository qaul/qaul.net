/// qaul network messaging service
///
/// is responsible to distribute messages to users
/// the container contains the entire message with signature
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// signed by sending user
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Message envelope
    #[prost(message, optional, tag="2")]
    pub envelope: ::core::option::Option<Envelope>,
}
/// message envelop with sender and receiver
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Envelope {
    /// the qaul ID of the sender
    #[prost(bytes="vec", tag="1")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// the qaul ID of the receiver
    #[prost(bytes="vec", tag="2")]
    pub receiver_id: ::prost::alloc::vec::Vec<u8>,
    /// the encrypted message data
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// message received confirmation
/// 
/// every message that was received by a user
/// sends an acknowledgment package, to the sender
/// to confirm the receive.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Confirmation {
    /// message ID
    #[prost(bytes="vec", tag="1")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// receveived at timestamp
    #[prost(uint64, tag="2")]
    pub received_at: u64,
}
/// chat message
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    /// group chat
    #[prost(bool, tag="1")]
    pub group: bool,
    /// conversation id 
    /// (only for group chat messages)
    #[prost(bytes="vec", tag="2")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// sent timestamp
    #[prost(uint64, tag="3")]
    pub sent_at: u64,
    /// message
    #[prost(string, tag="4")]
    pub content: ::prost::alloc::string::String,
}
