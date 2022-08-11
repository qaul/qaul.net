/// Group service RPC message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// message type
    #[prost(oneof="group::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub message: ::core::option::Option<group::Message>,
}
/// Nested message and enum types in `Group`.
pub mod group {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// group create request
        #[prost(message, tag="1")]
        GroupCreateRequest(super::GroupCreateRequest),
        /// group create response
        #[prost(message, tag="2")]
        GroupCreateResponse(super::GroupCreateResponse),
        /// group rename request
        #[prost(message, tag="3")]
        GroupRenameRequest(super::GroupRenameRequest),
        /// group invite member request
        #[prost(message, tag="4")]
        GroupInviteMemberRequest(super::GroupInviteMemberRequest),
        /// group remove member request
        #[prost(message, tag="5")]
        GroupRemoveMemberRequest(super::GroupRemoveMemberRequest),
        ///group info request
        #[prost(message, tag="6")]
        GroupInfoRequest(super::GroupInfoRequest),
        ///group info response
        #[prost(message, tag="7")]
        GroupInfoResponse(super::GroupInfoResponse),
        ///group reply invite
        #[prost(message, tag="8")]
        GroupReplyInviteRequest(super::GroupReplyInviteRequest),
        ///group list request
        #[prost(message, tag="9")]
        GroupListRequest(super::GroupListRequest),
        ///group list response
        #[prost(message, tag="10")]
        GroupListResponse(super::GroupListResponse),
        ///group send message
        #[prost(message, tag="11")]
        GroupSendRequest(super::GroupSendRequest),
    }
}
/// Create New Group 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateRequest {
    /// group name
    #[prost(string, tag="1")]
    pub group_name: ::prost::alloc::string::String,
}
/// Group creating response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateResponse {
    /// group name
    #[prost(string, tag="1")]
    pub group_name: ::prost::alloc::string::String,
    /// group id
    #[prost(bytes="vec", tag="2")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
}
/// Group rename request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRenameRequest {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// group name
    #[prost(string, tag="2")]
    pub group_name: ::prost::alloc::string::String,
}
/// Invite member
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteMemberRequest {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// Reply Invite 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReplyInviteRequest {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// accept
    #[prost(bool, tag="3")]
    pub accept: bool,
}
/// Remove member
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberRequest {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// Group info request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfoRequest {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
}
/// Group member response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
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
    ///last message index 
    #[prost(uint32, tag="5")]
    pub last_message_index: u32,
}
/// Group info response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfoResponse {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// group name
    #[prost(string, tag="2")]
    pub group_name: ::prost::alloc::string::String,
    /// created at
    #[prost(uint64, tag="3")]
    pub created_at: u64,
    ///members
    #[prost(message, repeated, tag="4")]
    pub members: ::prost::alloc::vec::Vec<GroupMember>,
}
/// Group list request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupListRequest {
}
/// Group info response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupListResponse {
    /// group list
    #[prost(message, repeated, tag="1")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfoResponse>,
}
/// Group send message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendRequest {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// message
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
/// Group send message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupConversationRequest {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
}
