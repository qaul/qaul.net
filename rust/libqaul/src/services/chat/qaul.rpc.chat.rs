/// Chat service RPC message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chat {
    /// message type
    #[prost(oneof="chat::Message", tags="1, 2, 3, 4, 5")]
    pub message: ::core::option::Option<chat::Message>,
}
/// Nested message and enum types in `Chat`.
pub mod chat {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// request an overview over the last conversations
        #[prost(message, tag="1")]
        OverviewRequest(super::ChatOverviewRequest),
        /// contains the overview list
        #[prost(message, tag="2")]
        OverviewList(super::ChatOverviewList),
        /// request a specific conversation
        #[prost(message, tag="3")]
        ConversationRequest(super::ChatConversationRequest),
        /// list of a chat conversation
        #[prost(message, tag="4")]
        ConversationList(super::ChatConversationList),
        /// send a new chat message
        #[prost(message, tag="5")]
        Send(super::ChatMessageSend),
    }
}
/// request for overview list of all conversations
/// this request shall be sent continuously when the view is open
///
/// at the moment always the entire list is sent
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatOverviewRequest {
}
/// overview list of conversations
/// this can eighter be the entire list or the last updates
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatOverviewList {
    #[prost(message, repeated, tag="1")]
    pub overview_list: ::prost::alloc::vec::Vec<ChatOverview>,
}
/// a chat conversation overview item
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatOverview {
    /// id of the user
    #[prost(bytes="vec", tag="1")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// last message id
    #[prost(uint64, tag="2")]
    pub last_message_index: u64,
    /// name of the conversation
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// time when last message was sent or received
    #[prost(uint64, tag="4")]
    pub last_message_at: u64,
    /// unread messages
    #[prost(int32, tag="5")]
    pub unread: i32,
    /// content type
    #[prost(enumeration="ChatContentType", tag="6")]
    pub content_type: i32,
    /// preview text of the last message
    #[prost(bytes="vec", tag="7")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// sender of the last message
    #[prost(bytes="vec", tag="8")]
    pub last_message_sender_id: ::prost::alloc::vec::Vec<u8>,
}
/// request messages of a specific chat conversation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatConversationRequest {
    /// conversation id
    #[prost(bytes="vec", tag="1")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// send only changes that are newer than the last received
    #[prost(uint64, tag="2")]
    pub last_index: u64,
}
/// list of chat messages of a specific conversation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatConversationList {
    /// conversation id
    #[prost(bytes="vec", tag="1")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// several messages
    #[prost(message, repeated, tag="2")]
    pub message_list: ::prost::alloc::vec::Vec<ChatMessage>,
}
/// a single chat message
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    /// index
    #[prost(uint64, tag="1")]
    pub index: u64,
    /// id of the sending user
    #[prost(bytes="vec", tag="2")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// message id or member id
    #[prost(bytes="vec", tag="3")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// message status
    #[prost(enumeration="MessageStatus", tag="4")]
    pub status: i32,
    /// message reception confirmed
    ///
    /// When a user receives a message, sent by us,
    /// the user is confirming the reception of this message.
    /// We are only getting this confirmation if we are the sender of this
    /// message.
    #[prost(message, repeated, tag="10")]
    pub message_reception_confirmed: ::prost::alloc::vec::Vec<MessageReceptionConfirmed>,
    /// conversation id
    #[prost(bytes="vec", tag="5")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// time when the message was sent
    #[prost(uint64, tag="6")]
    pub sent_at: u64,
    /// time when the message was received
    #[prost(uint64, tag="7")]
    pub received_at: u64,
    /// content type
    #[prost(enumeration="ChatContentType", tag="8")]
    pub content_type: i32,
    /// content of the message
    #[prost(bytes="vec", tag="9")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// message reception confirmed
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageReceptionConfirmed {
    /// user id
    #[prost(bytes="vec", tag="1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// time of confirmation
    #[prost(uint64, tag="2")]
    pub confirmed_at: u64,
}
/// chat content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatContent {
    /// message text
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
}
/// file content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileContent {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
    /// file name
    #[prost(string, tag="2")]
    pub file_name: ::prost::alloc::string::String,
    /// file extension
    #[prost(string, tag="3")]
    pub file_extension: ::prost::alloc::string::String,
    /// file size
    #[prost(uint32, tag="4")]
    pub file_size: u32,
    /// file description
    #[prost(string, tag="5")]
    pub file_description: ::prost::alloc::string::String,
}
/// Group event information
/// this message is purely informational
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEvent {
    /// group event type
    #[prost(enumeration="GroupEventType", tag="1")]
    pub event_type: i32,
    /// user ID of user joined or left
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// send chat message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessageSend {
    /// conversation id to which this message is sent
    #[prost(bytes="vec", tag="1")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// content of the message
    #[prost(string, tag="2")]
    pub content: ::prost::alloc::string::String,
}
/// Chat Content Type
///
/// describes the message content type
/// of the message encoded in the ChatMessage content field
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChatContentType {
    /// Undefined / Error
    None = 0,
    /// chat content message
    /// ChatContent
    Chat = 1,
    /// file content message
    /// FileContent
    File = 2,
    /// group event information
    /// GroupEvent
    Group = 3,
    /// RTC event information
    Rtc = 4,
}
impl ChatContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChatContentType::None => "NONE",
            ChatContentType::Chat => "CHAT",
            ChatContentType::File => "FILE",
            ChatContentType::Group => "GROUP",
            ChatContentType::Rtc => "RTC",
        }
    }
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
    /// group was closed
    Closed = 4,
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
            GroupEventType::Closed => "CLOSED",
        }
    }
}
