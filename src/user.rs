//! Alexandria user management
//!

/// Just a regular person (...maybe)
pub struct User {
    /// The name of the user namespace
    pub id: String,
    /// Associated pubkey
    pub pubkey: Option<String>,
    /// Access token for future operations
    pub token: String,
}
