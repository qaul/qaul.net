/// GroupChat network message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChatContainer {
    #[prost(oneof="group_chat_container::Message", tags="1, 2, 3, 4, 5")]
    pub message: ::core::option::Option<group_chat_container::Message>,
}
/// Nested message and enum types in `GroupChatContainer`.
pub mod group_chat_container {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// group invite
        #[prost(message, tag="1")]
        InviteMember(super::InviteMember),
        /// reply invite
        #[prost(message, tag="2")]
        ReplyInvite(super::ReplyInvite),
        /// group notify
        #[prost(message, tag="3")]
        Notify(super::GroupNotify),
        /// member removed
        #[prost(message, tag="4")]
        Removed(super::RemovedMember),
        /// group chat message
        #[prost(message, tag="5")]
        GroupchatMessage(super::GroupchatMessage),
    }
}
/// Invite member
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InviteMember {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// group name
    #[prost(string, tag="2")]
    pub group_name: ::prost::alloc::string::String,
    /// group admin id
    #[prost(bytes="vec", tag="3")]
    pub admin_id: ::prost::alloc::vec::Vec<u8>,
    /// group created at
    #[prost(uint64, tag="4")]
    pub created_at: u64,
    /// group member count
    #[prost(uint32, tag="5")]
    pub members_count: u32,
}
/// Group member
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Member {
    ///user id
    #[prost(bytes="vec", tag="1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    ///role
    #[prost(uint32, tag="2")]
    pub role: u32,
    ///joined at
    #[prost(uint64, tag="3")]
    pub joined_at: u64,
    ///state 
    #[prost(uint32, tag="4")]
    pub state: u32,
}
/// Group Notify
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupNotify {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// group name
    #[prost(string, tag="2")]
    pub group_name: ::prost::alloc::string::String,
    ///created at
    #[prost(uint64, tag="3")]
    pub created_at: u64,
    /// creator id
    #[prost(bytes="vec", tag="4")]
    pub creator_id: ::prost::alloc::vec::Vec<u8>,
    /// updated members
    #[prost(message, repeated, tag="5")]
    pub members: ::prost::alloc::vec::Vec<Member>,
}
/// Accept Invite
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyInvite {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// accept true : accept, false: decline
    #[prost(bool, tag="2")]
    pub accept: bool,
}
/// Removed member 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovedMember {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
}
///Group chat message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupchatMessage {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// content
    #[prost(string, tag="2")]
    pub content: ::prost::alloc::string::String,
    /// sent at
    #[prost(uint64, tag="3")]
    pub sent_at: u64,
}
