/// Group service RPC message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// message type
    #[prost(oneof="group::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14")]
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
        /// group rename response
        #[prost(message, tag="4")]
        GroupRenameResponse(super::GroupRenameResponse),
        /// group invite member request
        #[prost(message, tag="5")]
        GroupInviteMemberRequest(super::GroupInviteMemberRequest),
        /// group invite member response
        #[prost(message, tag="6")]
        GroupInviteMemberResponse(super::GroupInviteMemberResponse),
        /// group remove member request
        #[prost(message, tag="7")]
        GroupRemoveMemberRequest(super::GroupRemoveMemberRequest),
        /// group remove member response
        #[prost(message, tag="8")]
        GroupRemoveMemberResponse(super::GroupRemoveMemberResponse),
        ///group info request
        #[prost(message, tag="9")]
        GroupInfoRequest(super::GroupInfoRequest),
        ///group info response
        #[prost(message, tag="10")]
        GroupInfoResponse(super::GroupInfoResponse),
        ///group reply invite request
        #[prost(message, tag="11")]
        GroupReplyInviteRequest(super::GroupReplyInviteRequest),
        ///group reply invite response
        #[prost(message, tag="12")]
        GroupReplyInviteResponse(super::GroupReplyInviteResponse),
        ///group list request
        #[prost(message, tag="13")]
        GroupListRequest(super::GroupListRequest),
        ///group list response
        #[prost(message, tag="14")]
        GroupListResponse(super::GroupListResponse),
    }
}
/// Group Result
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupResult {
    /// status
    #[prost(bool, tag="1")]
    pub status: bool,
    /// message
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
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
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// result 
    #[prost(message, optional, tag="2")]
    pub result: ::core::option::Option<GroupResult>,
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
/// Group rename response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRenameResponse {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// group name
    #[prost(string, tag="2")]
    pub group_name: ::prost::alloc::string::String,
    /// result 
    #[prost(message, optional, tag="3")]
    pub result: ::core::option::Option<GroupResult>,
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
/// Invite member response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteMemberResponse {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// result 
    #[prost(message, optional, tag="3")]
    pub result: ::core::option::Option<GroupResult>,
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
/// Reply Invite Response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReplyInviteResponse {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// result 
    #[prost(message, optional, tag="3")]
    pub result: ::core::option::Option<GroupResult>,
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
/// Remove member
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberResponse {
    /// group id
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// result 
    #[prost(message, optional, tag="3")]
    pub result: ::core::option::Option<GroupResult>,
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
    /// is direct chat
    #[prost(bool, tag="4")]
    pub is_direct_chat: bool,
    /// members
    #[prost(message, repeated, tag="5")]
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
