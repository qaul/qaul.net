/// DTN service RPC message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dtn {
    /// message type
    #[prost(oneof = "dtn::Message", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub message: ::core::option::Option<dtn::Message>,
}
/// Nested message and enum types in `DTN`.
pub mod dtn {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// dtn state request
        #[prost(message, tag = "1")]
        DtnStateRequest(super::DtnStateRequest),
        /// dtn state response
        #[prost(message, tag = "2")]
        DtnStateResponse(super::DtnStateResponse),
        /// dtn config request
        #[prost(message, tag = "3")]
        DtnConfigRequest(super::DtnConfigRequest),
        /// dtn config response
        #[prost(message, tag = "4")]
        DtnConfigResponse(super::DtnConfigResponse),
        /// dtn add user request
        #[prost(message, tag = "5")]
        DtnAddUserRequest(super::DtnAddUserRequest),
        /// dtn add user response
        #[prost(message, tag = "6")]
        DtnAddUserResponse(super::DtnAddUserResponse),
        /// dtn remove user request
        #[prost(message, tag = "7")]
        DtnRemoveUserRequest(super::DtnRemoveUserRequest),
        /// dtn remove user response
        #[prost(message, tag = "8")]
        DtnRemoveUserResponse(super::DtnRemoveUserResponse),
        /// dtn set total size request
        #[prost(message, tag = "9")]
        DtnSetTotalSizeRequest(super::DtnSetTotalSizeRequest),
        /// dtn set total size response
        #[prost(message, tag = "10")]
        DtnSetTotalSizeResponse(super::DtnSetTotalSizeResponse),
    }
}
/// Dtn State Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnStateRequest {}
/// Dtn State Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnStateResponse {
    /// used size
    #[prost(uint64, tag = "1")]
    pub used_size: u64,
    /// dtn message count
    #[prost(uint32, tag = "2")]
    pub dtn_message_count: u32,
    /// unconfirmed count
    #[prost(uint32, tag = "3")]
    pub unconfirmed_count: u32,
}
/// Dtn Config Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnConfigRequest {}
/// Dtn Config Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnConfigResponse {
    /// total_size
    #[prost(uint32, tag = "1")]
    pub total_size: u32,
    /// users
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub users: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Dtn Add User Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnAddUserRequest {
    /// user id
    #[prost(bytes = "vec", tag = "1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// Dtn Add User Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnAddUserResponse {
    /// total_size
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// users
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Dtn Remove User Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnRemoveUserRequest {
    /// user id
    #[prost(bytes = "vec", tag = "1")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
/// Dtn Remove User Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnRemoveUserResponse {
    /// total_size
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// users
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Dtn SetTotalSize Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnSetTotalSizeRequest {
    /// total_size
    #[prost(uint32, tag = "1")]
    pub total_size: u32,
}
/// Dtn Remove User Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtnSetTotalSizeResponse {
    /// total_size
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// users
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
