//! Alexandria user management
//!

/// Just a regular person (...maybe)
pub struct User {
    /// The name of the user namespace
    id: String,
    /// Associated pubkey
    pubkey: Option<String>,
    /// Access token for future operations
    token: String,
}
