use crate::rpc::utils::*;
use bluer::{Adapter, Address};
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter;
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

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "qaul_id")]
    pub qaul_id: Option<Vec<i8>>,

    #[serde(rename = "message")]
    pub message: Option<Vec<i8>>,
}

lazy_static! {
    static ref IGNORE_LIST: Mutex<HashMap<Address, BleScanDevice>> = Mutex::new(HashMap::new());
    static ref DEVICE_LIST: Mutex<HashMap<Address, BleScanDevice>> = Mutex::new(HashMap::new());
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
    if devices.contains_key(&device.mac_address) {
        devices.get_mut(&device.mac_address).map(|val| { *val = device; });
    }else {
        devices.insert(device.mac_address, device);
    }
}

/// Key value lookup for previously discovered.
pub fn find_device_by_mac(mac_address: Address) -> Option<BleScanDevice> {
    let devices = DEVICE_LIST.lock().unwrap();
    if devices.contains_key(&mac_address) {
        Some(devices[&mac_address].clone())
    } else {
        None
    }
    // match devices
    //     .iter()
    //     .find(|device| device.mac_address == mac_address)
    // {
    //     Some(device) => Some(device.clone()),
    //     None => None,
    // }
}

#[allow(dead_code)]
/// Remove a device from the list of previously discovered devices.
pub fn remove_device_by_mac(mac_address: Address) {
    let mut devices = DEVICE_LIST.lock().unwrap();
    devices.remove(&mac_address);
}

/// Add a new message to the message map maintained by ble listner.
pub fn add_msg_map(stringified_addr: String, hex_msg: String) {
    let mut msg_map = MSG_MAP.lock().unwrap();
    if msg_map.contains_key(stringified_addr.as_str()) {
        msg_map.get_mut(&stringified_addr).map(|val| { *val = hex_msg; });
    }else {
        msg_map.insert(stringified_addr, hex_msg);
    }
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
    if ignore_devices.contains_key(&device.mac_address) {
        ignore_devices.get_mut(&device.mac_address).map(|val| { *val = device; });
    }else {
        ignore_devices.insert(device.mac_address, device);
    }
}

/// Key value lookup for devices present nearby.
pub fn find_ignore_device_by_mac(mac_address: Address) -> Option<BleScanDevice> {
    let ignore_devices = IGNORE_LIST.lock().unwrap();
    if ignore_devices.contains_key(&mac_address) {
        Some(ignore_devices[&mac_address].clone())
    } else {
        None
    }
}

/// Update the last found time of a device present nearby.
pub fn update_last_found(mac_address: Address) {
    let mut devices = IGNORE_LIST.lock().unwrap();
    let k = devices.get_mut(&mac_address);
    match k {
        Some(device) => {
            device.last_found_time = current_time_millis();
            log::info!("Time updated");
        }
        None => log::warn!("Device not discovered"),
    };
}

/// Remove a device from the list of devices present nearby.
pub fn remove_ignore_device_by_mac(mac_address: Address) {
    let mut devices = IGNORE_LIST.lock().unwrap();
    // devices.retain(|device| device.mac_address != mac_address);
    devices.remove(&mac_address);
    log::info!("Device removed from ignore list");
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
pub fn out_of_range_checker(adapter: Adapter, mut internal_sender: BleResultSender) {
    async_std::task::spawn(async move {
        log::info!("Out of range checker started");
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(2)).await;
            let ignore_list = IGNORE_LIST.lock().unwrap();
            let current_time = current_time_millis();
            if ignore_list.len() == 0 {
                continue;
            }
            for (_, device) in ignore_list.iter() {
                log::warn!(
                    "Current time: {}, device last found {}",
                    current_time,
                    device.last_found_time
                );

                if device.last_found_time != 0 && device.last_found_time < current_time - 5000 {
                    let mac_address: Address = device.mac_address;
                    if device.qaul_id.is_empty() {
                        log::error!("Qaul_id is empty");
                    }
                    log::error!("Device out of range: {:?}", mac_address);
                    internal_sender.send_device_unavailable(
                        device.qaul_id.clone(),
                        adapter.clone(),
                        mac_address,
                    );
                    remove_ignore_device_by_mac(mac_address);
                    // remove_device_by_mac(mac_address);
                    drop(ignore_list);
                    break;
                } else {
                    log::info!("Device in range: {:?}", device.mac_address);
                }
            }
        }
    });
}

pub fn message_received(e: (String, Address), mut internal_sender: BleResultSender) {
    let byte_encoded_message = hex_to_bytes(&e.0);
    // log::error!("Byte array: {:?}", byte_encoded_message);
    let json_message: String = match String::from_utf8(byte_encoded_message) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            return;
        }
    };
    // let json_message = String::from_utf8_lossy(&byte_encoded_message);
    // log::error!("Received messages: {:?} ", json_message);
    let msg_object: Message = match serde_json::from_str(&json_message) {   
        Ok(v) => v,
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            return;
        }
    };
    let message = msg_object.message.unwrap();
    let qaul_id = msg_object.qaul_id.unwrap();
    log::info!(
        "Received {} bytes of data from {}",
        message.len(),
        mac_to_string(&e.1)
    );
    let unsigned_message: Vec<u8> = message.into_iter().map(|x| x as u8).collect();
    let unsigned_qaul_id: Vec<u8> = qaul_id.into_iter().map(|x| x as u8).collect();
    internal_sender.send_direct_received(unsigned_qaul_id, unsigned_message)
}

/// Byte to Hex conversion.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let hex_chars: Vec<String> = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();

    hex_chars.join("")
}

// /// Bytes to String conversion.
// pub fn bytes_to_str(bytes: &[u8]) -> Result<&str, std::str::Utf8Error> {
//     std::str::from_utf8(bytes)
// }

/// Hex to Byte conversion.
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    if hex.len() % 2 != 0 {
        log::error!("Must have an even length");
    }

    let bytes_result: Result<Vec<u8>, _> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect();
    bytes_result.unwrap()
}

// Random string name generator
pub fn get_random_string(length: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat_with(|| rng.sample(Alphanumeric))
        .take(length)
        .map(char::from)
        .collect()
}
