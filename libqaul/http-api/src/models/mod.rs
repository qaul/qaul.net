mod secret;
pub use secret::Secret;

mod user;
pub use user::User;

mod grant;
pub use grant::Grant;

mod trust;
pub use trust::Trust;

#[cfg(feature = "messaging")]
mod text_message;
#[cfg(feature = "messaging")]
pub use text_message::TextMessage;

mod qaul_message;
pub use qaul_message::QaulMessage;

use crate::error::GenericError;
use hex::{decode, encode};
use identity::{Identity, ID_LEN};
use libqaul::messages::{MsgId, ID_LEN as MSG_ID_LEN};

pub fn from_identity(id: &Identity) -> String {
    encode(id)
}

pub fn into_identity(s: &str) -> Result<Identity, GenericError> {
    decode(s)
        .map_err(|e| GenericError::new("Invalid Identity".into()).detail(format!("{}", e)))
        .and_then(|i| {
            if i.len() != ID_LEN {
                Err(GenericError::new("Invalid Identity".into()).detail(format!(
                    "Invalid length: expected {}, got {}",
                    ID_LEN,
                    i.len()
                )))
            } else {
                Ok(Identity::truncate(&i))
            }
        })
}

pub fn from_message_id(id: &MsgId) -> String {
    encode(id)
}

pub fn into_message_id(s: &str) -> Result<MsgId, GenericError> {
    decode(s)
        .map_err(|e| GenericError::new("Invalid Message Id".into()).detail(format!("{}", e)))
        .and_then(|i| {
            if i.len() != MSG_ID_LEN {
                Err(GenericError::new("Invalid Message Id".into()).detail(format!(
                    "Invalid length: expected {}, got {}",
                    MSG_ID_LEN,
                    i.len()
                )))
            } else {
                Ok(i.into_iter().zip(0..MSG_ID_LEN).fold([0; ID_LEN], |mut acc, (x, i)| {
                    acc[i] = x;
                    acc
                }).into())
            }
        })
}
