use std::path::PathBuf;

/// The hub configuration
pub(crate) struct Config {
    /// Path to initial peer set
    pub(crate) peers: PathBuf,
    /// Runtime mode (in netmod-tcp)
    pub(crate) mode: String,
}

// //! Configuration for the Linux daemon\
// use serde::{Serialize, Deserialize};

// pub trait NameableNetmodConfig {
//     fn name(&self) -> &str;
//     fn netmod(&self) -> &str;
// }

// /// Top level configuration type for the daemon
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Config {
//     modules: Vec<NetmodConfig>
// }

// /// Network module configuration enum, for selecting which modules are to be loaded.
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum NetmodConfig {
//     Udp(UdpNetmodConfig),
//     Overlay(OverlayNetmodConfig)
//     // TODO: Configs for more modules here
// }

// /// Network module config struct for netmod-udp. Prototype for other network modules.
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct UdpNetmodConfig {
//     /// A semantic name, to make understanding error messages, etc, easier.
//     pub name: String,
//     /// The UDP address (IP or hostname) on which to listen
//     // TODO: This actually isn't possible with the current netmod implementation
//     address: Option<String>,
//     /// The UDP port on which to listen
//     pub port: u16
// }

// impl NameableNetmodConfig for UdpNetmodConfig {
//     fn name(&self) -> &str { &self.name }
//     fn netmod(&self) -> &str { "netmod-udp" }
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct OverlayNetmodConfig {
//     /// A semantic name, to make understanding error messages, etc, easier.
//     pub name: String,
//     /// The address of the server to connect to
//     pub server: String,
//     /// The port on the server to which to connect
//     pub server_port: u16,
//     /// The address to bind to; optional, will use system defaults
//     pub bind_address: Option<String>,
//     /// The port to bind to; optional, will use a random port as assigned by the system
//     pub bind_port: Option<u16>
// }

// impl NameableNetmodConfig for OverlayNetmodConfig {
//     fn name(&self) -> &str { &self.name }
//     fn netmod(&self) -> &str { "netmod-overlay" }
// }
