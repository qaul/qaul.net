//! Package representations

mod base;
mod meta;
mod data;

pub use self::base::*;
pub use self::meta::*;
pub use self::data::*;

/// A fingerprint buffer (8 bytes – 64bit)
pub type Fingerprint = [u8; 8];

/// A signature buffer (32 bytes – 256bit)
pub type Signature = [u8; 32];

/// An IP address, both for IPv4 and v6
#[derive(Serialize, Deserialize)]
pub enum IpAddress {
    V4([u8; 4]),
    V6([u8; 16]),
}
