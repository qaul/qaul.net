/// BLE system communication message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ble {
    /// message type
    #[prost(oneof = "ble::Message", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub message: ::core::option::Option<ble::Message>,
}
/// Nested message and enum types in `Ble`.
pub mod ble {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// device information request
        #[prost(message, tag = "1")]
        InfoRequest(super::BleInfoRequest),
        /// device information response
        #[prost(message, tag = "2")]
        InfoResponse(super::BleInfoResponse),
        /// start device request
        #[prost(message, tag = "3")]
        StartRequest(super::BleStartRequest),
        /// start device result
        #[prost(message, tag = "4")]
        StartResult(super::BleStartResult),
        /// stop device request
        #[prost(message, tag = "5")]
        StopRequest(super::BleStopRequest),
        /// stop device result
        #[prost(message, tag = "6")]
        StopResult(super::BleStopResult),
        /// device discovered
        #[prost(message, tag = "7")]
        DeviceDiscovered(super::BleDeviceDiscovered),
        /// device became unavailable
        #[prost(message, tag = "8")]
        DeviceUnavailable(super::BleDeviceUnavailable),
        /// send a direct message
        #[prost(message, tag = "9")]
        DirectSend(super::BleDirectSend),
        /// direct message send result
        #[prost(message, tag = "10")]
        DirectSendResult(super::BleDirectSendResult),
        /// direct message received
        #[prost(message, tag = "11")]
        DirectReceived(super::BleDirectReceived),
    }
}
/// device information request message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleInfoRequest {}
/// device information response message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleInfoResponse {
    /// fill in a device information of the BLE device
    #[prost(message, optional, tag = "1")]
    pub device: ::core::option::Option<BleDeviceInfo>,
}
/// BLE device information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDeviceInfo {
    /// Check if Bluetooth / Bluetooth Low Energy is supported
    ///
    /// Android: check if a bluetooth adapter is found
    #[prost(bool, tag = "1")]
    pub ble_support: bool,
    /// Bluetooth device address
    /// 48 bit unique Bluetooth device addr
    /// e.g. 80:86:F2:08:C7:98
    ///
    /// Android: BluetoothAdapter getAddress()
    /// <https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#getAddress(>)
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Get Bluetooth Name
    /// this is field is purely informative
    ///
    /// Android: BluetoothAdapter getName()
    /// <https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#getName(>)
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Bluetooth is enable / powered on
    ///
    /// Android: BluetoothAdapter isEnabled()
    /// <https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#isEnabled(>)
    #[prost(bool, tag = "4")]
    pub bluetooth_on: bool,
    /// Is extended advertisement supported?
    ///
    /// Android: BluetoothAdapter isLeExtendedAdvertisingSupported ()
    /// <https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLeExtendedAdvertisingSupported(>)
    #[prost(bool, tag = "5")]
    pub adv_extended: bool,
    /// what is the maximal amount of bytes sendable via advertising?
    ///
    /// Android: BluetoothAdapter getLeMaximumAdvertisingDataLength()
    /// <https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#getLeMaximumAdvertisingDataLength(>)
    #[prost(uint32, tag = "6")]
    pub adv_extended_bytes: u32,
    /// Is 2M phy supported?
    ///
    /// Android: BluetoothAdapter isLe2MPhySupported()
    /// <https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLe2MPhySupported(>)
    #[prost(bool, tag = "7")]
    pub le_2m: bool,
    /// is extended advertising supported in coded
    /// mode? (For long distance connections)
    ///
    /// Android: BluetoothAdapter isLeCodedPhySupported()
    /// <https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLeCodedPhySupported(>)
    #[prost(bool, tag = "8")]
    pub le_coded: bool,
    /// is LE audio supported?
    ///
    /// This is the most recent BLE feature, supported on:
    ///
    /// * android 12 and above
    /// * linux ?
    /// * ios ?
    /// * macos ?
    /// * windows ?
    ///
    /// Android: AndroidAdapter isLeAudioSupported()
    /// <https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#isLeAudioSupported(>)
    #[prost(bool, tag = "9")]
    pub le_audio: bool,
    /// is periodic advertisment supported?
    ///
    /// Android: BluetoothAdapter isLePeriodicAdvertisingSupported()
    /// <https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLePeriodicAdvertisingSupported(>)
    #[prost(bool, tag = "14")]
    pub le_periodic_adv_support: bool,
    /// Is multi advertisement supported?
    ///
    /// When multi advertisement is supported one can have different
    /// advertisement types parallely. Each advertisement has a
    /// different device address.
    /// For scanning devices it looks, as if multiple devices devices
    /// would advertise themselves.
    /// This is helpful to support several incompatible advertisement
    /// modes at the same time.
    ///
    /// Android: BluetoothAdapter isMultipleAdvertisementSupported()
    /// <https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isMultipleAdvertisementSupported(>)
    #[prost(bool, tag = "15")]
    pub le_multiple_adv_support: bool,
    /// Android Specific: is Offloaded Filtering Supported?
    ///
    /// Android: BluetoothAdapter isOffloadedFilteringSupported()
    ///
    #[prost(bool, tag = "16")]
    pub offload_filter_support: bool,
    /// Android Specific: is Offloaded Scan Batching Supported?
    ///
    /// Android: BluetoothAdapter isOffloadedScanBatchingSupported()
    /// <https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isOffloadedScanBatchingSupported(>)
    #[prost(bool, tag = "17")]
    pub offload_scan_batching_support: bool,
}
/// Start Device
///
/// the module will try to start the device, power it up,
/// get all rights, configure it for qaul, and
/// send & receive advertising messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleStartRequest {
    /// qaul ID
    ///
    /// The small 16 byte qaul id
    /// to be used to identify this node
    #[prost(bytes = "vec", tag = "1")]
    pub qaul_id: ::prost::alloc::vec::Vec<u8>,
    /// power settings
    #[prost(enumeration = "BlePowerSetting", tag = "2")]
    pub power_setting: i32,
}
/// Start device result message
///
/// Feedback from the
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleStartResult {
    /// whether the device was successfully started
    #[prost(bool, tag = "1")]
    pub success: bool,
    /// error reason
    #[prost(enumeration = "BleError", tag = "2")]
    pub error_reason: i32,
    /// error message
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Stop Bluetooth Device
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleStopRequest {}
/// Stop Result
///
/// Feedback of the stop request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleStopResult {
    /// whether the device was successfully stopped
    #[prost(bool, tag = "1")]
    pub success: bool,
    /// error reason
    #[prost(enumeration = "BleError", tag = "2")]
    pub error_reason: i32,
    /// error message
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Device Discovered
///
/// A new device has been discovered.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDeviceDiscovered {
    /// qaul id of the device
    #[prost(bytes = "vec", tag = "1")]
    pub qaul_id: ::prost::alloc::vec::Vec<u8>,
    /// the received signal strength of this device
    #[prost(int32, tag = "2")]
    pub rssi: i32,
}
/// Device Unavailable
///
/// A formerly discovered device has become
/// unavailable. No messages can be sent to it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDeviceUnavailable {
    /// qaul id of the device that
    /// became unavailable
    #[prost(bytes = "vec", tag = "1")]
    pub qaul_id: ::prost::alloc::vec::Vec<u8>,
}
/// send a direct message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDirectSend {
    /// message id (as a reference for the result message)
    #[prost(bytes = "vec", tag = "1")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// qaul id of the device to send it to
    #[prost(bytes = "vec", tag = "2")]
    pub receiver_id: ::prost::alloc::vec::Vec<u8>,
    /// qaul id of the sending device
    #[prost(bytes = "vec", tag = "3")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// data to be sent
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// result after sending the direct message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDirectSendResult {
    /// message id
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// result after sending the message
    #[prost(bool, tag = "2")]
    pub success: bool,
    /// error messages
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// direct message received message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BleDirectReceived {
    /// qaul id of the sending device
    #[prost(bytes = "vec", tag = "1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    /// the data received
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// power settings
///
/// These power settings relate to the android
/// power modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlePowerSetting {
    /// use power saving option
    ///
    /// this option will miss a lot of incoming messages,
    /// as the processor is often sleeping
    LowPower = 0,
    /// use a compromise between power
    /// saving and reactivity
    Balanced = 1,
    /// always listen
    ///
    /// this option uses the most battery power
    LowLatency = 2,
}
impl BlePowerSetting {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlePowerSetting::LowPower => "low_power",
            BlePowerSetting::Balanced => "balanced",
            BlePowerSetting::LowLatency => "low_latency",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "low_power" => Some(Self::LowPower),
            "balanced" => Some(Self::Balanced),
            "low_latency" => Some(Self::LowLatency),
            _ => None,
        }
    }
}
/// BLE Error Reasons
///
/// TODO: this list needs to be completed
///        if none of the reasons apply, use
///        UNKNOWN_ERROR
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BleError {
    /// undefined error
    ///
    /// use this when no other reason applies
    UnknownError = 0,
    /// the rights to use BLE were
    /// not provided by the user
    RightsMissing = 1,
    /// there was a module timeout
    Timeout = 2,
}
impl BleError {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BleError::UnknownError => "UNKNOWN_ERROR",
            BleError::RightsMissing => "RIGHTS_MISSING",
            BleError::Timeout => "TIMEOUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_ERROR" => Some(Self::UnknownError),
            "RIGHTS_MISSING" => Some(Self::RightsMissing),
            "TIMEOUT" => Some(Self::Timeout),
            _ => None,
        }
    }
}
