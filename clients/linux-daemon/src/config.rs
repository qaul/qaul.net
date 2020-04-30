//! Configuration for the Linux daemon

trait NetmodConfig {
    fn name(&self) -> &str;
    fn netmod(&self) -> &str;
}

/// Top level configuration type for the daemon
#[derive(Serialise, Deserialize, Debug)]
struct Config {
    modules: Vec<NetmodConfig>
}

/// Network module configuration enum, for selecting which modules are to be loaded.
#[derive(Serialise, Deserialize, Debug)]
enum NetmodConfig {
    Udp(UdpNetmodConfig)
    // TODO: Configs for more modules here
}

/// Network module config struct for netmod-udp. Prototype for other network modules.
struct UdpNetmodConfig {
    /// A semantic name, to make understanding error messages, etc, easier.
    name: String,
    /// The UDP address (IP or hostname) on which to listen
    // TODO: This actually isn't possible with the current netmod implementation
    address: String,
    /// The UDP port on which to listen
    port: u32
}

impl NetmodConfig for UdpNetmodConfig {
    fn name(&self) -> &str { &self.name }
    fn netmod(&self) -> &str { "netmod-udp" }
}
