/// Libqaul RPC Debug Messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Debug {
    /// message type
    #[prost(oneof="debug::Message", tags="1, 2, 3")]
    pub message: ::core::option::Option<debug::Message>,
}
/// Nested message and enum types in `Debug`.
pub mod debug {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// request a heartbeat
        #[prost(message, tag="1")]
        HeartbeatRequest(super::HeartbeatRequest),
        /// response to the heartbeat request
        #[prost(message, tag="2")]
        HeartbeatResponse(super::HeartbeatResponse),
        /// libqaul panics immediatly
        #[prost(message, tag="3")]
        Panic(super::Panic),
    }
}
/// Request a Heartbeat from Libqaul
///
/// The UI requests regular heartbeats from libqaul,
/// to check if libqaul is still alive
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatRequest {
}
/// Heartbeat Reply
///
/// Libqaul answers to the heartbeat request
/// with the heartbeat reply answer
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatResponse {
}
/// Panic
///
/// If libqaul receives this panic message, it
/// throws an error and panics immediatly.
///
/// This message is for debugging only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Panic {
}
