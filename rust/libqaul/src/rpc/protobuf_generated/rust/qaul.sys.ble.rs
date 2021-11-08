/// BLE system communication message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ble {
    /// message type
    #[prost(oneof="ble::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub message: ::core::option::Option<ble::Message>,
}
/// Nested message and enum types in `Ble`.
pub mod ble {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// device information request
        #[prost(message, tag="1")]
        InfoRequest(super::BleInfoRequest),
        /// device information response
        #[prost(message, tag="2")]
        InfoResponse(super::BleInfoResponse),
        /// start device request
        #[prost(message, tag="3")]
        StartRequest(super::BleStartRequest),
        /// start device result
        #[prost(message, tag="4")]
        StartResult(super::BleStartResult),
        /// advertising set message content
        #[prost(message, tag="5")]
        AdvertisingSet(super::BleAdvertisingSet),
        /// send advertsing message
        #[prost(message, tag="6")]
        AdvertisingSend(super::BleAdvertisingSend),
        /// advertising message received
        #[prost(message, tag="7")]
        AdvertisingReceived(super::BleAdvertisingReceived),
        /// send a direct message
        #[prost(message, tag="8")]
        DirectSend(super::BleDirectSend),
        /// direct message received
        #[prost(message, tag="9")]
        DirectReceived(super::BleDirectReceived),
    }
}
/// device information request message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleInfoRequest {
}
/// device information response message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleInfoResponse {
    /// fill in a device information for each device on the system
    #[prost(message, repeated, tag="1")]
    pub device: ::prost::alloc::vec::Vec<BleDeviceInfo>,
}
/// BLE device information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDeviceInfo {
    /// Bluetooth device address
    /// 48 bit unique Bluetooth device addr
    /// e.g. 80:86:F2:08:C7:98
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// vendor name, device name, etc
    /// this is field is purely informative
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// device powered on
    #[prost(bool, tag="3")]
    pub powered: bool,
    /// BLE advertising features supported
    /// This field informs us if the basic necessities for 
    /// qaul BLE requirements are supportorted by
    /// this device.
    /// These requirements are:
    /// * BLE device roles: central & peripheral
    /// * Send & receive BLE advertisements
    #[prost(bool, tag="4")]
    pub ble_support: bool,
    /// does it support the 251 byte advertisement messages?
    #[prost(bool, tag="7")]
    pub adv_251: bool,
    /// is extended advertising supported
    #[prost(bool, tag="8")]
    pub adv_extended: bool,
    /// what is the maximal amount of bytes sendable via advertising
    #[prost(uint32, tag="9")]
    pub adv_extended_bytes: u32,
    /// the following checks for BLE 5 features
    /// is extended advertising supported?
    #[prost(bool, tag="10")]
    pub adv_1m: bool,
    /// is extended advertising supported with 2M phy?
    #[prost(bool, tag="11")]
    pub adv_2m: bool,
    /// is extended advertising supported in coded
    /// mode? (For long distance connections)
    #[prost(bool, tag="12")]
    pub adv_coded: bool,
    /// is LE audio supported?
    /// this is the most recent feature, supported by
    /// android 12 and above
    /// linux ?
    /// ios ?
    /// macos ?
    /// windows ?
    #[prost(bool, tag="13")]
    pub le_audio: bool,
}
/// start device request message
/// the module will try to start the device, power it up,
/// get all rights, configure it for qaul, and
/// send & receive advertising messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleStartRequest {
}
/// start device result message
/// this is the feedback 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleStartResult {
    /// whether the start of the device was a success or not
    #[prost(bool, tag="1")]
    pub success: bool,
    /// error message
    #[prost(string, tag="2")]
    pub error_message: ::prost::alloc::string::String,
    /// error reasons
    #[prost(bool, tag="3")]
    pub unknonw_error: bool,
    /// rights not provided
    #[prost(bool, tag="4")]
    pub no_rights: bool,
}
/// advertising set message content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleAdvertisingSet {
    /// set data which can be used for interval data advertisement
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// send advertsing message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleAdvertisingSend {
    /// advertising mode
    #[prost(enumeration="BleMode", tag="1")]
    pub mode: i32,
    /// the data to be sent in the data field
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// advertising message received
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleAdvertisingReceived {
    /// the Bluetooth address of the device sending the advertisement
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// the received signal strength of this device
    #[prost(int32, tag="2")]
    pub rssi: i32,
    /// the mode it was sent in
    #[prost(enumeration="BleMode", tag="3")]
    pub mode: i32,
    /// the data part of the advertising message
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// send a direct message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDirectSend {
    /// message id (as a reference for the result message)
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// bluetooth address of the device to send it to
    #[prost(bytes="vec", tag="2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    /// sending mode
    #[prost(enumeration="BleMode", tag="3")]
    pub mode: i32,
    /// data to be sent
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// result after sending the direct message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDirectSendResult {
    /// message id
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// result after sending the message
    #[prost(bool, tag="2")]
    pub success: bool,
    /// error messages
    #[prost(string, tag="3")]
    pub error_message: ::prost::alloc::string::String,
}
/// direct message received message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDirectReceived {
    /// bluetooth address of the sending device
    #[prost(bytes="vec", tag="1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    /// received signal strength of the sending device
    #[prost(int32, tag="2")]
    pub rssi: i32,
    /// the mode this message was sent in
    #[prost(enumeration="BleMode", tag="3")]
    pub mode: i32,
    /// the data received
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// enum to describe how to send a message
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BleMode {
    /// use legacy advertising mode (only 31 Byte payload)
    Legacy = 0,
    /// 1m phy
    Le1m = 1,
    /// 2m phy
    Le2m = 2,
    /// LE coded, which only half the speed
    Coded2 = 3,
    /// LE coded, which is 8 times slower
    Coded8 = 4,
}
