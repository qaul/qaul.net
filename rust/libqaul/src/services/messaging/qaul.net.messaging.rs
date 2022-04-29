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
    /// encrypted message data
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<Data>,
}
/// encrypted message data
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// message nonce for encryption
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    /// the encrypted message data
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// messaging unified message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Messaging {
    #[prost(oneof="messaging::Message", tags="1, 2, 3")]
    pub message: ::core::option::Option<messaging::Message>,
}
/// Nested message and enum types in `Messaging`.
pub mod messaging {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        ConfirmationMessage(super::Confirmation),
        #[prost(message, tag="2")]
        ChatMessage(super::ChatMessage),
        #[prost(message, tag="3")]
        CryptoService(super::CryptoService),
    }
}
/// Crypto Service Message
///
/// This message is for crypto specific tasks,
/// such as completing a handshake.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoService {
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
