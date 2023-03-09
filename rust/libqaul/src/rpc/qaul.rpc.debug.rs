/// Libqaul RPC Debug Messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Debug {
    /// message type
    #[prost(oneof = "debug::Message", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub message: ::core::option::Option<debug::Message>,
}
/// Nested message and enum types in `Debug`.
pub mod debug {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// request a heartbeat
        #[prost(message, tag = "1")]
        HeartbeatRequest(super::HeartbeatRequest),
        /// response to the heartbeat request
        #[prost(message, tag = "2")]
        HeartbeatResponse(super::HeartbeatResponse),
        /// libqaul panics immediately
        #[prost(message, tag = "3")]
        Panic(super::Panic),
        /// enable/disable logging to file
        #[prost(message, tag = "4")]
        LogToFile(super::LogToFile),
        /// Storage Path Request
        #[prost(message, tag = "5")]
        StoragePathRequest(super::StoragePathRequest),
        /// Storage Path Response
        #[prost(message, tag = "6")]
        StoragePathResponse(super::StoragePathResponse),
        /// Request for library to delete logs
        #[prost(message, tag = "7")]
        DeleteLibqaulLogsRequest(super::DeleteLibqaulLogsRequest),
    }
}
/// Request a Heartbeat from Libqaul
///
/// The UI requests regular heartbeats from libqaul,
/// to check if libqaul is still alive
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatRequest {}
/// Heartbeat Reply
///
/// Libqaul answers to the heartbeat request
/// with the heartbeat reply answer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatResponse {}
/// Panic
///
/// If libqaul receives this panic message, it
/// throws an error and panics immediatly.
///
/// This message is for debugging only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Panic {}
/// LogToFile
///
/// If libqaul receives this enable message, it
/// start or stop to log error contents into error_xxx.log file.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogToFile {
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
/// StoragePathRequest
///
/// Return storage path
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePathRequest {}
/// StoragePathResponse
///
/// Contains Storage Path
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePathResponse {
    #[prost(string, tag = "1")]
    pub storage_path: ::prost::alloc::string::String,
}
/// DeleteLibqaulLogsRequest
///
/// Requests for the log folder to be wiped clean
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLibqaulLogsRequest {}
