use crate::rpc::utils::*;
use bluer::Address;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct BleScanDevice {
    pub qaul_id: Vec<u8>,
    pub rssi: i32,
    pub mac_address: Address,
    pub device: bluer::Device,
    pub last_found_time: i64,
    pub name: String,
    pub is_connected: bool,
}

lazy_static! {
    static ref IGNORE_LIST: Mutex<Vec<BleScanDevice>> = Mutex::new(Vec::new());
    static ref DEVICE_LIST: Mutex<Vec<BleScanDevice>> = Mutex::new(Vec::new());
    static ref MSG_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Convert a MAC address to a stringified version.
pub fn mac_to_string(addr: &Address) -> String {
    addr.map(|octet| format!("{:02x?}", octet)).join(":")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_prints_address() {
        assert_eq!(
            mac_to_string(&Address::any()),
            String::from("00:00:00:00:00:00")
        )
    }
}

/// Add a new device to list of all previously discovered devices.
pub fn add_device(device: BleScanDevice) {
    let mut devices = DEVICE_LIST.lock().unwrap();
    devices.push(device);
}

/// Key value lookup for previously discovered.
pub fn find_device_by_mac(mac_address: Address) -> Option<BleScanDevice> {
    let devices = DEVICE_LIST.lock().unwrap();
    match devices
        .iter()
        .find(|device| device.mac_address == mac_address)
    {
        Some(device) => Some(device.clone()),
        None => None,
    }
}

/// Remove a device from the list of previously discovered devices.
pub fn remove_device_by_mac(mac_address: Address) {
    let mut devices = DEVICE_LIST.lock().unwrap();
    devices.retain(|device| device.mac_address != mac_address);
}

/// Add a new message to the message map maintained by ble listner.
pub fn add_msg_map(stringified_addr: String, hex_msg: String) {
    let mut msg_map = MSG_MAP.lock().unwrap();
    msg_map.insert(stringified_addr.clone(), hex_msg.clone());
}

/// Key value lookup for message map maintained by ble listner.
pub fn find_msg_map_by_mac(stringified_addr: String) -> Option<String> {
    let msg_map = MSG_MAP.lock().unwrap();
    msg_map.get(&stringified_addr).cloned()
}

/// Remove a message from the message map maintained by ble listner.
pub fn remove_msg_map_by_mac(stringified_addr: String) {
    let mut msg_map = MSG_MAP.lock().unwrap();
    msg_map.remove(&stringified_addr);
}

/// Add a new device to list of devices present nearby and maintain their last found time.
pub fn add_ignore_device(device: BleScanDevice) {
    let mut ignore_devices = IGNORE_LIST.lock().unwrap();
    ignore_devices.push(device);
}

/// Key value lookup for devices present nearby.
pub fn find_ignore_device_by_mac(mac_address: Address) -> Option<BleScanDevice> {
    let devices = IGNORE_LIST.lock().unwrap();
    match devices
        .iter()
        .find(|device| device.mac_address == mac_address)
    {
        Some(device) => Some(device.clone()),
        None => None,
    }
}

/// Update the last found time of a device present nearby.
pub fn update_last_found(mac_address: Address) {
    let mut devices = IGNORE_LIST.lock().unwrap();
    for device in devices.iter_mut() {
        if device.mac_address == mac_address {
            device.last_found_time = current_time_millis();
        }
    }
}

/// Remove a device from the list of devices present nearby.
pub fn remove_ignore_device_by_mac(mac_address: Address) {
    let mut devices = DEVICE_LIST.lock().unwrap();
    devices.retain(|device| device.mac_address != mac_address);
}

/// Get the current time in milliseconds since UNIX_EPOCH.
pub fn current_time_millis() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis().try_into().unwrap()
}

/// Check if a device is out of range and remove it from the list of devices present nearby or Update the last found time of the device.
pub fn out_of_range_checker(mut internal_sender: BleResultSender) {
    async_std::task::spawn(async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(2)).await;
            let ignore_list = IGNORE_LIST.lock().unwrap();
            let current_time = current_time_millis();
            if ignore_list.len() == 0 {
                continue;
            }
            for device in ignore_list.iter() {
                if device.last_found_time != 0 && device.last_found_time < current_time - 5000 {
                    log::error!("Device out of range: {:?}", device.mac_address);
                    internal_sender.send_device_unavailable(device.qaul_id.clone());
                    let mac_address: Address = device.mac_address;
                    remove_device_by_mac(mac_address);
                    remove_ignore_device_by_mac(mac_address);
                }
            }
        }
    });
}

/// Byte to Hex conversion.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let hex_chars: Vec<String> = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();

    hex_chars.join("")
}

/// Bytes to String conversion.
pub fn bytes_to_str(bytes: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(bytes)
}

/// Hex to Byte conversion.
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in 0..hex.len() / 2 {
        let byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
        bytes.push(byte);
    }
    bytes
}
