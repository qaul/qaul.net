use std::path::PathBuf;

/// The hub configuration
pub(crate) struct Config {
    /// Path to initial peer set
    pub(crate) peers: PathBuf,
    /// Runtime mode (in netmod-tcp)
    pub(crate) mode: String,
}
