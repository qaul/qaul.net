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
    /// payload
    #[prost(bytes="vec", tag="3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// envelop payload
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvelopPayload {
    #[prost(oneof="envelop_payload::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<envelop_payload::Payload>,
}
/// Nested message and enum types in `EnvelopPayload`.
pub mod envelop_payload {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// encrypted message data
        #[prost(message, tag="1")]
        Encrypted(super::Encrypted),
        /// DTN message
        #[prost(bytes, tag="2")]
        Dtn(::prost::alloc::vec::Vec<u8>),
    }
}
/// encrypted message data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Encrypted {
    /// one or several Data messages
    /// of maximally 64KB each.
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Data>,
}
/// encrypted message data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// message nonce for encryption
    ///
    /// each nonce is only used once per key
    /// and increases by one fore each new data package.
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    /// the encrypted message data slice
    /// each data package contains maximally
    /// 64KB
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// messaging unified message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Messaging {
    #[prost(oneof="messaging::Message", tags="1, 2, 3, 4, 5")]
    pub message: ::core::option::Option<messaging::Message>,
}
/// Nested message and enum types in `Messaging`.
pub mod messaging {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// confirm chat message
        #[prost(message, tag="1")]
        ConfirmationMessage(super::Confirmation),
        /// crypto service
        #[prost(message, tag="2")]
        CryptoService(super::CryptoService),
        /// rtc stream
        #[prost(message, tag="3")]
        RtcStreamMessage(super::RtcStreamMessage),
        /// group notify
        #[prost(message, tag="4")]
        GroupNotifyMessage(super::GroupNotifyMessage),
        /// common message
        #[prost(message, tag="5")]
        CommonMessage(super::CommonMessage),
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
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// receveived at timestamp
    #[prost(uint64, tag="2")]
    pub received_at: u64,
}
/// common message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonMessage {
    /// message ID
    #[prost(bytes="vec", tag="1")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// conversation id
    #[prost(bytes="vec", tag="2")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// conversation id
    #[prost(uint64, tag="3")]
    pub sent_at: u64,
    /// payload
    #[prost(oneof="common_message::Payload", tags="4, 5, 6, 7")]
    pub payload: ::core::option::Option<common_message::Payload>,
}
/// Nested message and enum types in `CommonMessage`.
pub mod common_message {
    /// payload
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// chat message
        #[prost(message, tag="4")]
        ChatMessage(super::ChatMessage),
        /// file message
        #[prost(message, tag="5")]
        FileMessage(super::FileMessage),
        /// group message
        #[prost(message, tag="6")]
        GroupMessage(super::GroupMessage),
        /// rtc message
        #[prost(message, tag="7")]
        RtcMessage(super::RtcMessage),
    }
}
/// chat content
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    /// content
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
}
/// file message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMessage {
    #[prost(bytes="vec", tag="1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// rtc message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcMessage {
    #[prost(bytes="vec", tag="1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// rtc stream mesasge
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcStreamMessage {
    #[prost(bytes="vec", tag="1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// group message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMessage {
    #[prost(bytes="vec", tag="1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// group notify message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupNotifyMessage {
    #[prost(bytes="vec", tag="1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// DTN message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dtn {
    #[prost(oneof="dtn::Message", tags="1, 2")]
    pub message: ::core::option::Option<dtn::Message>,
}
/// Nested message and enum types in `Dtn`.
pub mod dtn {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// message container
        #[prost(bytes, tag="1")]
        Container(::prost::alloc::vec::Vec<u8>),
        /// message received response
        #[prost(message, tag="2")]
        Response(super::DtnResponse),
    }
}
/// DTN response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnResponse {
    /// the type of the message
    #[prost(enumeration="dtn_response::Type", tag="1")]
    pub r#type: i32,
    /// message signature reference
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// reason of rejection
    #[prost(enumeration="dtn_response::Reason", tag="3")]
    pub reason: i32,
}
/// Nested message and enum types in `DtnResponse`.
pub mod dtn_response {
    /// the enum definition of the type
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// the message was accepted for storage
        Accepted = 0,
        /// the message was rejected
        Rejected = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Accepted => "ACCEPTED",
                Type::Rejected => "REJECTED",
            }
        }
    }
    /// the enum definition of the rejection reason
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reason {
        /// this user is not accepted
        UserNotAccepted = 0,
        /// overall quota reached
        OverallQuota = 1,
        /// user quota reached
        UserQuota = 2,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::UserNotAccepted => "USER_NOT_ACCEPTED",
                Reason::OverallQuota => "OVERALL_QUOTA",
                Reason::UserQuota => "USER_QUOTA",
            }
        }
    }
}
