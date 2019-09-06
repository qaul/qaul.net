use serde::{Deserialize, Serialize};

/// A key attribute for enabling encrypion
///
/// When encryption is `on`, it must contain a valid
/// public key that can be used for encypting files on-write.
///
/// If it is `off`, no additional data must be provided.
#[derive(Serialize, Deserialize, Debug)]
pub enum KeyAttr {
    On(String),
    Off,
}
