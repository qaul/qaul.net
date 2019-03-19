//! Metadata package types

/// File metadata
#[derive(Serialize, Deserialize)]
pub struct FileMeta {
    description: String,
    extention: String,
    hash: String,
    size: u64,
}

/// (Selective) User metadata
#[derive(Serialize, Deserialize)]
pub struct UserMeta {
    username: String,
    fingerprint: String,
    pubkey: Option<String>,
}

/// User metadata selector
#[derive(Serialize, Deserialize)]
pub enum UserMetaType {
    Username,
    Pubkey,
    Avatar,
}
