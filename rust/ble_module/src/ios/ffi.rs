

use ble_protocol::{QAUL_ID_ADVERT_BYTES, QAUL_ID_BYTES};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex; 
use tokio::sync::mpsc;
use crate::ios::radio::{PeerKey, RadioEvent, Address};
use ble_protocol::engine::Role;
use ble_protocol::advert::id_prefix_from_name;

pub(crate) static BLE_POWERED: AtomicBool = AtomicBool::new(false);
/// Unbounded: Swifts calls cant wait for room, so a bound would only drop events.
pub(crate) static EVENTS: Mutex<Option<mpsc::UnboundedSender<RadioEvent>>> = Mutex::new(None);
static CALLBACK: Mutex<Option<ActionCallback>> = Mutex::new(None);


/****  Helpers  ****/

/// Push a radio event to the events channel, if it exists. Called from Swift.
/// Cant ever panic or block.
pub fn push(event: RadioEvent) {
    let Ok(board) = EVENTS.lock() else { return };
    // None while BLE isn't running: the event is ignored.
    if let Some(tx) = board.as_ref() {
        // Only fails once the loop has shut down.
        if let Err(err) = tx.send(event) {
            log::warn!("iOS radio event dropped: {}", err);
        }
    }
}

fn peer_key(addr: Address, role: u8) -> Option<PeerKey> {
    match role {
        0 => Some(PeerKey::central(addr)),
        1 => Some(PeerKey::peripheral(addr)),
        _ => {
            log::warn!("iOS event from {} dropped: unknown role {}", addr, role);
            None
        }
    }
}

/// The reverse of peer_key: the role byte Swift receives. Keep the two aligned.
fn role_byte(role: Role) -> u8 {
    match role {
        Role::Central => 0,
        Role::Peripheral => 1,
    }
}


/*********        Swift to Rust       ************/

#[no_mangle]
pub extern "C" fn qaul_ble_powered(on: bool) {
    log::info!("iOS Bluetooth powered: {}", on);
    BLE_POWERED.store(on, Ordering::Relaxed);
}
// Swift to Rust callbacks for BLE radio events. These are called from Swift when the corresponding event occurs.

#[no_mangle]
pub extern "C" fn qaul_ble_frame_received(addr: Address, role: u8, bytes: *const u8, len: u32) {
    let Some(peer) = peer_key(addr, role) else { return };
    if bytes.is_null() || len == 0 {
        log::warn!("iOS radio event dropped: frame received with null pointer or zero length");
        return;
    }
    if len > 5000 {
        log::error!("message size to big! size is {} bytes", len);
        return;
    }
    push(RadioEvent::FrameReceived {
        peer,
        bytes: unsafe { std::slice::from_raw_parts(bytes, len as usize) }.to_vec()
    });
}

#[no_mangle]
pub extern "C" fn qaul_ble_frame_sent(addr: Address, role: u8) {
    let Some(peer) = peer_key(addr, role) else { return };
    push(RadioEvent::FrameSent{ peer });
}

#[no_mangle]
pub extern "C" fn qaul_ble_subscribed(addr: Address, role: u8, mtu: u32) {
    let Some(peer) = peer_key(addr, role) else { return };
    push(RadioEvent::Subscribed{
        peer,
        mtu: mtu as usize,
        outbox: ()
    });
}

/// `len` is what Swift received, Rust checks it, so a short advert cant make us read past Swifts buffer.
#[no_mangle]
pub extern "C" fn qaul_ble_advert(addr: Address, id_prefix: *const u8, len: u32, rssi: i16) {
    if id_prefix.is_null() {
        log::warn!("iOS advert from {} dropped: null pointer", addr);
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(id_prefix, len as usize) };
    let Ok(id_prefix) = bytes.try_into() else {
        log::warn!("iOS advert from {} dropped: id prefix is {} bytes, expected {}", addr, len, QAUL_ID_ADVERT_BYTES);
        return;
    };
    push(RadioEvent::Advert { address: addr, id_prefix, rssi });
}

/// An advert without qaul manufacturer data, but with a name: so another iPhone. Rust has to then
/// decode this seperately. 
#[no_mangle]
pub extern "C" fn qaul_ble_advert_name(addr: Address, name: *const u8, len: u32, rssi: i16) {
    if name.is_null() {
        log::warn!("iOS advert from {} dropped: null pointer", addr);
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(name, len as usize) };
    let Ok(name) = std::str::from_utf8(bytes) else {
        log::warn!("iOS advert from {} dropped: name is not valid UTF-8", addr);
        return;
    };
    let Some(id_prefix) = id_prefix_from_name(name) else {
        log::debug!("iOS advert from {} ignored: name {:?} is not a qaul ID", addr, name);
        return;
    };
    push(RadioEvent::Advert { address: addr, id_prefix, rssi });
}

#[no_mangle]
pub extern "C" fn qaul_ble_peer_id_read(addr: Address, role: u8, id: *const u8, len: u32) {
    let Some(peer) = peer_key(addr, role) else { return };
    if id.is_null() {
        log::warn!("iOS peer id from {} dropped: null pointer", addr);
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(id, len as usize) };
    let Ok(id) = bytes.try_into() else {
        log::warn!("iOS peer id from {} dropped: id is {} bytes, expected {}", addr, len, QAUL_ID_BYTES);
        return;
    };
    push(RadioEvent::PeerIdRead { peer, id });
}

#[no_mangle]
pub extern "C" fn qaul_ble_connect_failed(addr: Address, role: u8) {
    let Some(peer) = peer_key(addr, role) else { return };
    push(RadioEvent::ConnectFailed{ peer });
}

#[no_mangle]
pub extern "C" fn qaul_ble_gone(addr: Address, role: u8) {
    let Some(peer) = peer_key(addr, role) else { return };
    push(RadioEvent::Gone { peer });
}



/*********        Rust to Swift       ************/

// Swift hands rust a function pointer to call when a radio event occurs. Rust calls it from the radio loop.
pub type ActionCallback = extern "C" fn(action: u8, addr: Address, role: u8, bytes: *const u8, len: u32);

/// What rust asks swift to do. The numbers are the action byte Swift receives.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Action { Start = 0, Stop = 1, Connect = 2, Disconnect = 3, Transmit = 4 }

/// Swift calls this once at startup. `Option` because Swift could pass null, and a
/// plain Rust fn pointer must never be null: null arrives here as `None`.
#[no_mangle]
pub extern "C" fn qaul_ble_set_callback(cb: Option<ActionCallback>) {
    log::info!("iOS Swift callback {}", if cb.is_some() { "registered" } else { "cleared" });
    if let Ok(mut board) = CALLBACK.lock() {
        *board = cb;
    }
}

/// Ask Swift to do something.
pub(crate) fn call_swift(action: Action, addr: Address, role: Role, bytes: &[u8]) {
    // Copy the fn pointer out and let the lock go before calling Swift otherwise can be deadlock
    let cb = match CALLBACK.lock() {
        Ok(board) => *board,
        Err(_) => None,
    };
    let Some(cb) = cb else {
        log::warn!("iOS {:?} for {} not sent: no Swift callback registered", action, addr);
        return;
    };
    let ptr = if bytes.is_empty() { std::ptr::null() } else { bytes.as_ptr() };
    cb(action as u8, addr, role_byte(role), ptr, bytes.len() as u32);
}








