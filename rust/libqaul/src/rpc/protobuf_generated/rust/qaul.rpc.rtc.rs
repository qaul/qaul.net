/// RTC service RPC message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcRpc {
    /// message type
    #[prost(oneof = "rtc_rpc::Message", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub message: ::core::option::Option<rtc_rpc::Message>,
}
/// Nested message and enum types in `RtcRpc`.
pub mod rtc_rpc {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// rtc session request
        #[prost(message, tag = "1")]
        RtcSessionRequest(super::RtcSessionRequest),
        /// rtc session response for request
        #[prost(message, tag = "2")]
        RtcSessionResponse(super::RtcSessionResponse),
        /// rtc session management
        #[prost(message, tag = "3")]
        RtcSessionManagement(super::RtcSessionManagement),
        /// rtc outgoing
        #[prost(message, tag = "4")]
        RtcOutgoing(super::RtcOutgoing),
        /// rtc incoming
        #[prost(message, tag = "5")]
        RtcIncoming(super::RtcIncoming),
        /// rtc session list request
        #[prost(message, tag = "6")]
        RtcSessionListRequest(super::RtcSessionListRequest),
        /// rtc session list response
        #[prost(message, tag = "7")]
        RtcSessionListResponse(super::RtcSessionListResponse),
    }
}
/// rtc session request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcSessionRequest {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
}
/// rtc session response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcSessionResponse {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
}
/// rtc session management
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcSessionManagement {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// option
    #[prost(uint32, tag = "2")]
    pub option: u32,
}
/// rtc outgoing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcOutgoing {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// content
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// rtc incoming
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcIncoming {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// content
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// rtc sessions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcSessionListRequest {}
/// rtc session
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcSession {
    /// group id
    #[prost(bytes = "vec", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// session type
    #[prost(uint32, tag = "2")]
    pub session_type: u32,
    /// stste
    #[prost(uint32, tag = "3")]
    pub state: u32,
    /// created at
    #[prost(uint64, tag = "4")]
    pub created_at: u64,
}
/// rtc session list response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcSessionListResponse {
    /// session list
    #[prost(message, repeated, tag = "1")]
    pub sessions: ::prost::alloc::vec::Vec<RtcSession>,
}
