/// BLE RPC Message Container
///
/// Union of all messages that can be sent or received
/// via RPC between the UI and libqaul
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ble {
    #[prost(oneof = "ble::Message", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub message: ::core::option::Option<ble::Message>,
}
/// Nested message and enum types in `Ble`.
pub mod ble {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        InfoRequest(super::InfoRequest),
        #[prost(message, tag = "2")]
        InfoResponse(super::InfoResponse),
        #[prost(message, tag = "3")]
        StartRequest(super::StartRequest),
        #[prost(message, tag = "4")]
        StopRequest(super::StopRequest),
        #[prost(message, tag = "5")]
        DiscoveredRequest(super::DiscoveredRequest),
        #[prost(message, tag = "6")]
        DiscoveredResponse(super::DiscoveredResponse),
        #[prost(message, tag = "7")]
        RightsRequest(super::RightsRequest),
        #[prost(message, tag = "8")]
        RightsResult(super::RightsResult),
    }
}
/// UI request for information on devices and module status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {}
/// BLE Info Response Message
///
/// Contains information on the status of the module,
/// as well as all available BLE devices
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    /// the small 16 byte BLE id
    #[prost(bytes = "vec", tag = "1")]
    pub small_id: ::prost::alloc::vec::Vec<u8>,
    /// status of the module
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
    /// devices
    #[prost(bytes = "vec", tag = "3")]
    pub device_info: ::prost::alloc::vec::Vec<u8>,
}
/// Request BLE module to start
///
/// Start message sent from UI to libqaul.
///
/// This message only has an effect if the module
/// has not already started.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartRequest {}
/// Request BLE module to stop
///
/// Stop message sent from UI to libqaul.
///
/// This message only has an effect if the module
/// was started earlier and is running.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {}
/// Request Discovered Nodes on BLE
///
/// Message sent from UI to libqaul.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveredRequest {}
/// All Discovered Nodes
///
/// Answer from libqaul to UI on DiscoveredRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveredResponse {
    /// number of nodes in discovery table
    #[prost(uint32, tag = "1")]
    pub nodes_count: u32,
    /// number of nodes in to_confirm table
    #[prost(uint32, tag = "2")]
    pub to_confirm_count: u32,
}
/// Request Rights
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RightsRequest {}
/// Rights Request Results
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RightsResult {
    #[prost(bool, tag = "1")]
    pub rights_granted: bool,
}
