/// qaul network messaging service
///
/// is responsible to distribute messages to users
/// the container contains the entire message with signature
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// signed by sending user
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Message envelope
    #[prost(message, optional, tag = "2")]
    pub envelope: ::core::option::Option<Envelope>,
}
/// message envelop with sender and receiver
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Envelope {
    /// the qaul ID of the sender
    #[prost(bytes = "vec", tag = "1")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// the qaul ID of the receiver
    #[prost(bytes = "vec", tag = "2")]
    pub receiver_id: ::prost::alloc::vec::Vec<u8>,
    /// payload
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// envelop payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvelopPayload {
    #[prost(oneof = "envelop_payload::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<envelop_payload::Payload>,
}
/// Nested message and enum types in `EnvelopPayload`.
pub mod envelop_payload {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// encrypted message data
        #[prost(message, tag = "1")]
        Encrypted(super::Encrypted),
        /// DTN message
        #[prost(bytes, tag = "2")]
        Dtn(::prost::alloc::vec::Vec<u8>),
    }
}
/// encrypted message data
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Encrypted {
    /// state of the crypto session
    #[prost(enumeration = "CryptoState", tag = "1")]
    pub state: i32,
    /// crypto session id
    #[prost(uint32, tag = "2")]
    pub session_id: u32,
    /// one or several Data messages
    /// of maximally 64KB each.
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<Data>,
}
/// encrypted message data
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// message nonce for encryption
    ///
    /// each nonce is only used once per key
    /// and increases by one fore each new data package.
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    /// the encrypted message data slice
    /// each data package contains maximally
    /// 64KB
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// messaging unified message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Messaging {
    #[prost(oneof = "messaging::Message", tags = "1, 2, 3, 4, 5, 6")]
    pub message: ::core::option::Option<messaging::Message>,
}
/// Nested message and enum types in `Messaging`.
pub mod messaging {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// confirm chat message
        #[prost(message, tag = "1")]
        ConfirmationMessage(super::Confirmation),
        /// dtn response message
        #[prost(message, tag = "2")]
        DtnResponse(super::DtnResponse),
        /// crypto service
        #[prost(message, tag = "3")]
        CryptoService(super::CryptoService),
        /// rtc stream
        #[prost(message, tag = "4")]
        RtcStreamMessage(super::RtcStreamMessage),
        /// group invite messages
        #[prost(message, tag = "5")]
        GroupInviteMessage(super::GroupInviteMessage),
        /// common message
        #[prost(message, tag = "6")]
        CommonMessage(super::CommonMessage),
    }
}
/// message received confirmation
///
/// every message that was received by a user
/// sends an acknowledgment package, to the sender
/// to confirm the receive.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Confirmation {
    /// message ID
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// received at timestamp
    #[prost(uint64, tag = "2")]
    pub received_at: u64,
}
/// Crypto Service Message
///
/// This message is for crypto specific tasks,
/// such as completing a handshake.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoService {}
/// rtc stream mesasge
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcStreamMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// group invite message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// common message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonMessage {
    /// message ID
    #[prost(bytes = "vec", tag = "1")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// group id
    #[prost(bytes = "vec", tag = "2")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// sent at timestamp
    #[prost(uint64, tag = "3")]
    pub sent_at: u64,
    /// payload
    #[prost(oneof = "common_message::Payload", tags = "4, 5, 6, 7")]
    pub payload: ::core::option::Option<common_message::Payload>,
}
/// Nested message and enum types in `CommonMessage`.
pub mod common_message {
    /// payload
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// chat message
        #[prost(message, tag = "4")]
        ChatMessage(super::ChatMessage),
        /// file message
        #[prost(message, tag = "5")]
        FileMessage(super::FileMessage),
        /// group message
        #[prost(message, tag = "6")]
        GroupMessage(super::GroupMessage),
        /// rtc message
        #[prost(message, tag = "7")]
        RtcMessage(super::RtcMessage),
    }
}
/// chat content
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    /// content
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
/// file message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// group message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// rtc message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// DTN message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dtn {
    #[prost(oneof = "dtn::Message", tags = "1, 2")]
    pub message: ::core::option::Option<dtn::Message>,
}
/// Nested message and enum types in `Dtn`.
pub mod dtn {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// message container
        #[prost(bytes, tag = "1")]
        Container(::prost::alloc::vec::Vec<u8>),
        /// message received response
        #[prost(message, tag = "2")]
        Response(super::DtnResponse),
    }
}
/// DTN response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnResponse {
    /// the type of the message
    #[prost(enumeration = "dtn_response::ResponseType", tag = "1")]
    pub response_type: i32,
    /// message signature reference
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// reason of rejection
    #[prost(enumeration = "dtn_response::Reason", tag = "3")]
    pub reason: i32,
}
/// Nested message and enum types in `DtnResponse`.
pub mod dtn_response {
    /// the enum definition of the type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ResponseType {
        /// the message was accepted for storage
        Accepted = 0,
        /// the message was rejected
        Rejected = 1,
    }
    impl ResponseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseType::Accepted => "ACCEPTED",
                ResponseType::Rejected => "REJECTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCEPTED" => Some(Self::Accepted),
                "REJECTED" => Some(Self::Rejected),
                _ => None,
            }
        }
    }
    /// the enum definition of the rejection reason
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Reason {
        /// none
        None = 0,
        /// this user is not accepted
        UserNotAccepted = 1,
        /// overall quota reached
        OverallQuota = 2,
        /// user quota reached
        UserQuota = 3,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::None => "NONE",
                Reason::UserNotAccepted => "USER_NOT_ACCEPTED",
                Reason::OverallQuota => "OVERALL_QUOTA",
                Reason::UserQuota => "USER_QUOTA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "USER_NOT_ACCEPTED" => Some(Self::UserNotAccepted),
                "OVERALL_QUOTA" => Some(Self::OverallQuota),
                "USER_QUOTA" => Some(Self::UserQuota),
                _ => None,
            }
        }
    }
}
/// state of the crypto session
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CryptoState {
    /// no crypto at all
    None = 0,
    /// crypto session is in handshake state
    Handshake = 1,
    /// crypto session is in transport state
    Transport = 2,
}
impl CryptoState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CryptoState::None => "NONE",
            CryptoState::Handshake => "HANDSHAKE",
            CryptoState::Transport => "TRANSPORT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "HANDSHAKE" => Some(Self::Handshake),
            "TRANSPORT" => Some(Self::Transport),
            _ => None,
        }
    }
}
