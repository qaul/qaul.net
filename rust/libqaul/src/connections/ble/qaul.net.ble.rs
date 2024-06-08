/// BLE network communication message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleMessage {
    /// message type
    #[prost(oneof = "ble_message::Message", tags = "1, 2, 3, 4")]
    pub message: ::core::option::Option<ble_message::Message>,
}
/// Nested message and enum types in `BleMessage`.
pub mod ble_message {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// info message
        #[prost(bytes, tag = "1")]
        Info(::prost::alloc::vec::Vec<u8>),
        /// feed message
        #[prost(bytes, tag = "2")]
        Feed(::prost::alloc::vec::Vec<u8>),
        /// messaging message
        #[prost(bytes, tag = "3")]
        Messaging(::prost::alloc::vec::Vec<u8>),
        /// identification request
        #[prost(message, tag = "4")]
        Identification(super::Identification),
    }
}
/// Identfication Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identification {
    #[prost(bool, tag = "1")]
    pub request: bool,
    #[prost(message, optional, tag = "2")]
    pub node: ::core::option::Option<NodeIdentification>,
}
/// Identity Information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeIdentification {
    /// Node ID
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
