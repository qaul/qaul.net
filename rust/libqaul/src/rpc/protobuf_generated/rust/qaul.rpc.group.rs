/// Group service RPC message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// message type
    #[prost(
        oneof = "group::Message",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16"
    )]
    pub message: ::core::option::Option<group::Message>,
}
/// Nested message and enum types in `Group`.
pub mod group {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// group create request
        #[prost(message, tag = "1")]
        GroupCreateRequest(super::GroupCreateRequest),
        /// group create response
        #[prost(message, tag = "2")]
        GroupCreateResponse(super::GroupCreateResponse),
        /// group rename request
        #[prost(message, tag = "3")]
        GroupRenameRequest(super::GroupRenameRequest),
        /// group rename response
        #[prost(message, tag = "4")]
        GroupRenameResponse(super::GroupRenameResponse),
        /// group invite member request
        #[prost(message, tag = "5")]
        GroupInviteMemberRequest(super::GroupInviteMemberRequest),
        /// group invite member response
        #[prost(message, tag = "6")]
        GroupInviteMemberResponse(super::GroupInviteMemberResponse),
        /// group remove member request
        #[prost(message, tag = "7")]
        GroupRemoveMemberRequest(super::GroupRemoveMemberRequest),
        /// group remove member response
        #[prost(message, tag = "8")]
        GroupRemoveMemberResponse(super::GroupRemoveMemberResponse),
        /// group info request
        #[prost(message, tag = "9")]
        GroupInfoRequest(super::GroupInfoRequest),
        /// group info response
        #[prost(message, tag = "10")]
        GroupInfoResponse(super::GroupInfo),
        /// group reply invite request
        #[prost(message, tag = "11")]
        GroupReplyInviteRequest(super::GroupReplyInviteRequest),
        /// group reply invite response
        #[prost(message, tag = "12")]
        GroupReplyInviteResponse(super::GroupReplyInviteResponse),
        /// group list request
        #[prost(message, tag = "13")]
        GroupListRequest(super::GroupListRequest),
        /// group list response
        #[prost(message, tag = "14")]
        GroupListResponse(super::GroupListResponse),
        /// group invited
        #[prost(message, tag = "15")]
        GroupInvitedRequest(super::GroupInvitedRequest),
        /// group invited response
        #[prost(message, tag = "16")]
        GroupInvitedResponse(super::GroupInvitedResponse),
    }
}
/// Group Result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupResult {
    /// status
    ///
    /// true = success
    /// false = an error happened
    ///
    /// if the result is false, the message will
    /// contain the error message.
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// message
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Create New Group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateRequest {
    /// group name
    #[prost(string, tag = "1")]
    pub group_name: ::prost::alloc::string::String,
}
/// Group creating response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateResponse {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// result
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<GroupResult>,
}
/// Group rename request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRenameRequest {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// group name
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
}
/// Group rename response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRenameResponse {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// group name
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// result
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<GroupResult>,
}
/// Invite member
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteMemberRequest {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes = "vec", tag = "2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// Invite member response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteMemberResponse {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes = "vec", tag = "2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// result
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<GroupResult>,
}
/// Reply Invite
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReplyInviteRequest {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// accept
    #[prost(bool, tag = "3")]
    pub accept: bool,
}
/// Reply Invite Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReplyInviteResponse {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// result
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<GroupResult>,
}
/// Remove member
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberRequest {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes = "vec", tag = "2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// Remove member
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberResponse {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// user id
    #[prost(bytes = "vec", tag = "2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// result
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<GroupResult>,
}
/// Group info request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfoRequest {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
}
/// Group member response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
    /// user id
    #[prost(bytes = "vec", tag = "1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// role
    #[prost(enumeration = "GroupMemberRole", tag = "2")]
    pub role: i32,
    /// joined at
    #[prost(uint64, tag = "3")]
    pub joined_at: u64,
    /// state
    #[prost(enumeration = "GroupMemberState", tag = "4")]
    pub state: i32,
    /// last message index
    #[prost(uint32, tag = "5")]
    pub last_message_index: u32,
}
/// Group info response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfo {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// group name
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// created at
    #[prost(uint64, tag = "3")]
    pub created_at: u64,
    /// group status
    #[prost(enumeration = "GroupStatus", tag = "4")]
    pub status: i32,
    /// group revision number
    #[prost(uint32, tag = "5")]
    pub revision: u32,
    /// is direct chat
    #[prost(bool, tag = "6")]
    pub is_direct_chat: bool,
    /// members
    #[prost(message, repeated, tag = "7")]
    pub members: ::prost::alloc::vec::Vec<GroupMember>,
    /// unread messages
    #[prost(uint32, tag = "8")]
    pub unread_messages: u32,
    /// time when last message was sent
    #[prost(uint64, tag = "9")]
    pub last_message_at: u64,
    /// content type
    #[prost(bytes = "vec", tag = "10")]
    pub last_message: ::prost::alloc::vec::Vec<u8>,
    /// sender of the last message
    #[prost(bytes = "vec", tag = "11")]
    pub last_message_sender_id: ::prost::alloc::vec::Vec<u8>,
}
/// Group list request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupListRequest {}
/// Group info response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupListResponse {
    /// group list
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInvited {
    /// sender id
    #[prost(bytes = "vec", tag = "1")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// received at
    #[prost(uint64, tag = "2")]
    pub received_at: u64,
    /// group info
    #[prost(message, optional, tag = "3")]
    pub group: ::core::option::Option<GroupInfo>,
}
/// Group list request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInvitedRequest {}
/// Group info response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInvitedResponse {
    /// invited list
    #[prost(message, repeated, tag = "1")]
    pub invited: ::prost::alloc::vec::Vec<GroupInvited>,
}
/// Group member state
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupMemberState {
    /// invited
    Invited = 0,
    /// activated
    Activated = 1,
}
impl GroupMemberState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GroupMemberState::Invited => "Invited",
            GroupMemberState::Activated => "Activated",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invited" => Some(Self::Invited),
            "Activated" => Some(Self::Activated),
            _ => None,
        }
    }
}
/// Group member role
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupMemberRole {
    /// user
    User = 0,
    /// admin
    Admin = 255,
}
impl GroupMemberRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GroupMemberRole::User => "User",
            GroupMemberRole::Admin => "Admin",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "User" => Some(Self::User),
            "Admin" => Some(Self::Admin),
            _ => None,
        }
    }
}
/// Group Status
///
/// Indicates the working status of a group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupStatus {
    /// Group is Active
    ///
    /// The group is in active state and we can post
    /// messages to this group.
    Active = 0,
    /// Invite Accepted
    ///
    /// We accepted the invitation to this group
    /// but we haven't received the updated group
    /// info from the group administrator yet.
    /// We therefore can't yet post messages into
    /// the group.
    InviteAccepted = 1,
    /// The group was deactivated
    ///
    /// We either left the group or have been removed from the group
    /// by the group administrator.
    /// We therefore can't post messages into this group anymore.
    Deactivated = 2,
}
impl GroupStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GroupStatus::Active => "ACTIVE",
            GroupStatus::InviteAccepted => "INVITE_ACCEPTED",
            GroupStatus::Deactivated => "DEACTIVATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTIVE" => Some(Self::Active),
            "INVITE_ACCEPTED" => Some(Self::InviteAccepted),
            "DEACTIVATED" => Some(Self::Deactivated),
            _ => None,
        }
    }
}
