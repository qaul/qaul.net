/// Group network message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupContainer {
    #[prost(oneof = "group_container::Message", tags = "1, 2, 3, 4")]
    pub message: ::core::option::Option<group_container::Message>,
}
/// Nested message and enum types in `GroupContainer`.
pub mod group_container {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// group invite
        #[prost(message, tag = "1")]
        InviteMember(super::InviteMember),
        /// reply invite
        #[prost(message, tag = "2")]
        ReplyInvite(super::ReplyInvite),
        /// group status update
        #[prost(message, tag = "3")]
        GroupInfo(super::GroupInfo),
        /// member removed
        #[prost(message, tag = "4")]
        Removed(super::RemovedMember),
    }
}
/// Invite member
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InviteMember {
    /// Group Info
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
}
/// Group member
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
/// Group Info
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
    /// group revision
    #[prost(uint32, tag = "4")]
    pub revision: u32,
    /// updated members
    #[prost(message, repeated, tag = "5")]
    pub members: ::prost::alloc::vec::Vec<GroupMember>,
}
/// Reply to Invite
///
/// Accept / Reject invitation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyInvite {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// accept true : accept, false: decline
    #[prost(bool, tag = "2")]
    pub accept: bool,
}
/// Removed member
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovedMember {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
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
