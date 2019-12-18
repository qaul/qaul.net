mod secret;
pub use secret::Secret;

mod user;
pub use user::User;

mod grant;
pub use grant::Grant;

use crate::error::GenericError;
use hex::{decode, encode};
use identity::{Identity, ID_LEN};

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
