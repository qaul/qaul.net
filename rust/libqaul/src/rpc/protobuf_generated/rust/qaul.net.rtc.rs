/// RTC network message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcContainer {
    #[prost(oneof = "rtc_container::Message", tags = "1, 2")]
    pub message: ::core::option::Option<rtc_container::Message>,
}
/// Nested message and enum types in `RtcContainer`.
pub mod rtc_container {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// rtc session request
        #[prost(message, tag = "1")]
        RtcSessionRequest(super::RtcSessionRequest),
        /// rtc session management
        #[prost(message, tag = "2")]
        RtcSessionManagement(super::RtcSessionManagement),
    }
}
/// rtc session request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcSessionRequest {
    /// type
    #[prost(uint32, tag = "1")]
    pub session_type: u32,
}
/// rtc session management
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcSessionManagement {
    /// option (1: accept, 2: deny, 3: end)
    #[prost(uint32, tag = "1")]
    pub option: u32,
}
/// Rtc message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcMessage {
    /// sequence
    #[prost(uint32, tag = "1")]
    pub sequence: u32,
    /// content
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// Rtc contents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcContent {
    /// content
    #[prost(oneof = "rtc_content::Content", tags = "1, 2, 3")]
    pub content: ::core::option::Option<rtc_content::Content>,
}
/// Nested message and enum types in `RtcContent`.
pub mod rtc_content {
    /// content
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// video content
        #[prost(message, tag = "1")]
        VideoContent(super::RtcVideoContent),
        /// audio content
        #[prost(message, tag = "2")]
        AudioContent(super::RtcAudioContent),
        /// chat content
        #[prost(message, tag = "3")]
        ChatContent(super::RtcChatContent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcChatContent {
    /// content
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcVideoContent {
    /// content
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcAudioContent {
    /// content
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
