/// Chat service RPC message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chat {
    /// message type
    #[prost(oneof = "chat::Message", tags = "3, 4, 5")]
    pub message: ::core::option::Option<chat::Message>,
}
/// Nested message and enum types in `Chat`.
pub mod chat {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// request a specific conversation
        #[prost(message, tag = "3")]
        ConversationRequest(super::ChatConversationRequest),
        /// list of a chat conversation
        #[prost(message, tag = "4")]
        ConversationList(super::ChatConversationList),
        /// send a new chat message
        #[prost(message, tag = "5")]
        Send(super::ChatMessageSend),
    }
}
/// request messages of a specific chat conversation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatConversationRequest {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// send only changes that are newer than the last received
    #[prost(uint64, tag = "2")]
    pub last_index: u64,
}
/// list of chat messages of a specific conversation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatConversationList {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// several messages
    #[prost(message, repeated, tag = "2")]
    pub message_list: ::prost::alloc::vec::Vec<ChatMessage>,
}
/// a single chat message
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    /// index
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// id of the sending user
    #[prost(bytes = "vec", tag = "2")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// message id or member id
    #[prost(bytes = "vec", tag = "3")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// message status
    #[prost(enumeration = "MessageStatus", tag = "4")]
    pub status: i32,
    /// message reception confirmed
    ///
    /// When a user receives a message, sent by us,
    /// the user is confirming the reception of this message.
    /// We are only getting this confirmation if we are the sender of this
    /// message.
    #[prost(message, repeated, tag = "10")]
    pub message_reception_confirmed: ::prost::alloc::vec::Vec<MessageReceptionConfirmed>,
    /// group id
    #[prost(bytes = "vec", tag = "5")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// time when the message was sent
    #[prost(uint64, tag = "6")]
    pub sent_at: u64,
    /// time when the message was received
    #[prost(uint64, tag = "7")]
    pub received_at: u64,
    /// chat content message
    #[prost(bytes = "vec", tag = "8")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// message reception confirmed
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageReceptionConfirmed {
    /// user id
    #[prost(bytes = "vec", tag = "1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// time of confirmation
    #[prost(uint64, tag = "2")]
    pub confirmed_at: u64,
}
/// chat content message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatContentMessage {
    #[prost(oneof = "chat_content_message::Message", tags = "1, 2, 3")]
    pub message: ::core::option::Option<chat_content_message::Message>,
}
/// Nested message and enum types in `ChatContentMessage`.
pub mod chat_content_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// a chat content message
        #[prost(message, tag = "1")]
        ChatContent(super::ChatContent),
        /// a file content message
        #[prost(message, tag = "2")]
        FileContent(super::FileContent),
        /// a group event information
        #[prost(message, tag = "3")]
        GroupEvent(super::GroupEvent),
    }
}
/// chat content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatContent {
    /// message text
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// file content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileContent {
    /// file id
    #[prost(uint64, tag = "1")]
    pub file_id: u64,
    /// file name
    #[prost(string, tag = "2")]
    pub file_name: ::prost::alloc::string::String,
    /// file extension
    #[prost(string, tag = "3")]
    pub file_extension: ::prost::alloc::string::String,
    /// file size
    #[prost(uint32, tag = "4")]
    pub file_size: u32,
    /// file description
    #[prost(string, tag = "5")]
    pub file_description: ::prost::alloc::string::String,
}
/// Group event information
/// this message is purely informational
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEvent {
    /// group event type
    #[prost(enumeration = "GroupEventType", tag = "1")]
    pub event_type: i32,
    /// user ID of user joined or left
    #[prost(bytes = "vec", tag = "2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// send chat message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessageSend {
    /// group id to which this message is sent
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// content of the message
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
/// Sending status of sent messages
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageStatus {
    /// message not sent yet
    ///
    /// this state is used for receiving files too
    Sending = 0,
    /// message successfully sent to another node
    Sent = 1,
    /// reciption has been confirmed
    Confirmed = 2,
    /// all group members confirmed that they received
    /// the message
    ConfirmedByAll = 3,
    /// message receiving
    Receiving = 4,
    /// message received
    Received = 5,
}
impl MessageStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageStatus::Sending => "SENDING",
            MessageStatus::Sent => "SENT",
            MessageStatus::Confirmed => "CONFIRMED",
            MessageStatus::ConfirmedByAll => "CONFIRMED_BY_ALL",
            MessageStatus::Receiving => "RECEIVING",
            MessageStatus::Received => "RECEIVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SENDING" => Some(Self::Sending),
            "SENT" => Some(Self::Sent),
            "CONFIRMED" => Some(Self::Confirmed),
            "CONFIRMED_BY_ALL" => Some(Self::ConfirmedByAll),
            "RECEIVING" => Some(Self::Receiving),
            "RECEIVED" => Some(Self::Received),
            _ => None,
        }
    }
}
/// Group info type definition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupEventType {
    /// default value, undefined message
    /// ignore this message
    Default = 0,
    /// user invited to group
    Invited = 1,
    /// user joined group
    Joined = 2,
    /// user left group
    Left = 3,
    /// your user was removed
    Removed = 4,
    /// group was closed
    Closed = 5,
    /// group was created
    Created = 6,
    /// group invite was accepted
    ///
    /// this state indicates, that we accepted
    /// an invite, but that we haven't received
    /// the group update from the administrator yet,
    /// and are therefore not yet an official member of the group.
    InviteAccepted = 7,
}
impl GroupEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GroupEventType::Default => "DEFAULT",
            GroupEventType::Invited => "INVITED",
            GroupEventType::Joined => "JOINED",
            GroupEventType::Left => "LEFT",
            GroupEventType::Removed => "REMOVED",
            GroupEventType::Closed => "CLOSED",
            GroupEventType::Created => "CREATED",
            GroupEventType::InviteAccepted => "INVITE_ACCEPTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "INVITED" => Some(Self::Invited),
            "JOINED" => Some(Self::Joined),
            "LEFT" => Some(Self::Left),
            "REMOVED" => Some(Self::Removed),
            "CLOSED" => Some(Self::Closed),
            "CREATED" => Some(Self::Created),
            "INVITE_ACCEPTED" => Some(Self::InviteAccepted),
            _ => None,
        }
    }
}
