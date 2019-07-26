use libqaul::Identity;
use json_api::{ResourceObject, Attributes};
use serde_derive::{Serialize, Deserialize};

/// Returned on successful `Token` grants
///
/// The token is stored in the `id` field
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserGrant {}

impl Attributes for UserGrant { fn kind() -> String { "user_grant".into() } }

