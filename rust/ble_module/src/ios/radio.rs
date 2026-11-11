use crate::rpc::proto_sys::{BleDeviceInfo, BleInfoResponse};
use ble_protocol::{engine::{Engine, Peer, Role}, QAUL_ID_ADVERT_BYTES, QAUL_ID_BYTES};
use bytes::Bytes;
use std::{error::Error, fmt};
use tokio::sync::mpsc;

use super::ffi::{call_swift, Action};

/// A peripheral's CoreBluetooth UUID. repr(C) so Swift can pass it by value
/// as `QaulBleAddress { uint8_t bytes[16]; }`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(pub [u8; 16]);

/// For actions that arent about one peer (Start, Stop).
const NO_ADDRESS: Address = Address([0; 16]);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.0.iter().enumerate() {
            if matches!(i, 4 | 6 | 8 | 10) {
                write!(f, "-")?;
            }
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

/// What the radio side tells the loop. One channel for all of them, so the loop sees one ordered stream.
#[derive(Debug)]
pub enum RadioEvent {
    /// The peer wrote one frame to MSG_CHAR. Can arrive before `Subscribed`,
    /// because Android writes SEND_ID first.
    FrameReceived { peer: PeerKey, bytes: Vec<u8> },
    /// A frame was sent to the radio, tell engine so that the next may be retrieved from this peer's link
    FrameSent { peer: PeerKey },
    /// The peer subscribed to notifications on MSG_CHAR. Outbox isnt needed on IOS so we just pass '()'.
    Subscribed {
        peer: PeerKey,
        mtu: usize,
        outbox: Outbox,
    },
    /// Scanner saw this advert, pass to engine for connection decision making
    Advert {
        address: Address,
        id_prefix:  [u8; QAUL_ID_ADVERT_BYTES], 
        rssi: i16
    },

    /// We read the peer's qaul ID from its READ_CHAR (central only).
    PeerIdRead { peer: PeerKey, id: [u8; QAUL_ID_BYTES] },
    /// A connect we started didn't produce a link (central only).
    ConnectFailed { peer: PeerKey },
    /// The peer's sending or recieving pipe failed.
    Gone { peer: PeerKey },
}

/// How the engine names a link on IOS: the peer CB UUID address and our role on it. 
/// The same peer could be linked in both roles at once, so the address alone isn't unique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerKey {
    pub address: Address,
    pub role: Role,
}

impl PeerKey {
    /// A phone connected to our GATT server.
    pub fn peripheral(address: Address) -> Self {
        Self { address, role: Role::Peripheral }
    }

    /// A phone we connected to as GATT client.
    pub fn central(address: Address) -> Self {
        Self { address, role: Role::Central }
    }
}

impl Peer for PeerKey {
    fn role(&self) -> Role {
        self.role
    }
}

/// iOS has none: transmit calls Swift directly
pub type Outbox = ();

/// Everything that exists only while BLE is on. Dropping it takes the
/// service off air.
pub struct Running {
    /// Swift's events, pushed by the C shim through `ffi::EVENTS`.
    pub events: mpsc::UnboundedReceiver<RadioEvent>,
    /// The protocol state. Lives here so a restart always begins with no links.
    pub engine: Engine<PeerKey>,
}

impl Running {
    pub async fn start(_device_id: &str, qaul_id: Bytes) -> Result<Running, Box<dyn Error>> {
        // Checked first, so a bad ID fails before anything goes on air.
        let our_id: [u8; QAUL_ID_BYTES] = qaul_id[..]
            .try_into()
            .map_err(|_| format!("qaul id must be {} bytes, got {}", QAUL_ID_BYTES, qaul_id.len()))?;

        let (events_tx, events_rx) = mpsc::unbounded_channel::<RadioEvent>();

        // Pin the sender on the shims side, so Swifts calls reach this loop
        if let Ok(mut board) = super::ffi::EVENTS.lock() {
            *board = Some(events_tx);
        }
        // Swift starts scanning only when told: libqaul decides whether BLE runs.
        call_swift(Action::Start, NO_ADDRESS, &[]);

        Ok(Running{
            events: events_rx,
            engine: Engine::new(our_id, gossip_seq_seed()),
        })
    }

    /// Start connecting to `peer` as central. The result comes back on `events`.
    pub fn connect(&self, peer: PeerKey) {
        log::info!("iOS -> Swift: connect {}", peer.address);
        call_swift(Action::Connect, peer.address, &[]);
    }

    /// No scanning while one of our connects is in flight, from Connect until Subscribed or ConnectFailed. 
    /// BlueZ needs scan pausing, iOS doesn't.
    pub fn update_scan_pause(&self) {
     
    }


    pub async fn update_advertising(&mut self) {
        // not needed on ios
    }

    pub fn transmit(&self, peer: PeerKey, bytes: Vec<u8>) {
        call_swift(Action::Transmit, peer.address, &bytes);
    }

    /// Nothing to do on iOS: Swift owns the write queue
    pub fn peer_ready(&mut self, peer: PeerKey, outbox: Outbox) {
     
    }

   
    pub fn disconnect(&mut self, peer: PeerKey) {
        log::info!("iOS -> Swift: disconnect {}", peer.address);
        call_swift(Action::Disconnect, peer.address, &[]);
    }


}

/// BLE stopped: take the sender off the board, so Swift's calls from now on are ignored.
impl Drop for Running {
    fn drop(&mut self) {
        if let Ok(mut board) = super::ffi::EVENTS.lock() {
            *board = None;
        }
        // Swift stops scanning and drops every connection.
        call_swift(Action::Stop, NO_ADDRESS, &[]);
    }
}

/// Where our neighbour list seq starts: the clock in whole seconds, so it moves ahead across restarts.
/// A random start would be behind half the time, and peers would drop our lists as stale until their entry for us expires 
fn gossip_seq_seed() -> u16 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs() as u16)
}

/// Check if bluetooth is powered on by the device.
pub async fn is_ble_enabled() -> bool {
    crate::ios::ffi::BLE_POWERED.load(std::sync::atomic::Ordering::Relaxed)
}

pub async fn get_device_info() -> Result<BleInfoResponse, Box<dyn Error>> {
    let response = BleInfoResponse {
        device: None,
        devices: vec![BleDeviceInfo { ble_support: true, bluetooth_on: true, name: "iOS".into(), ..Default::default() }]
    };
    Ok(response)
}
