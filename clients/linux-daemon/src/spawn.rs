//! Spawn network modules from configurations
use ratman::netmod::Endpoint;
use std::sync::Arc;
use netmod_udp::Endpoint as UdpEndpoint;
use crate::config::NetmodConfig;

/// A network module endpoint that carries its configuration about with it,
/// for easier debugging.
#[derive(Clone)]
pub struct ConfiguredEndpoint {
    config: NetmodConfig,
    endpoint: Arc<dyn Endpoint>
}

pub fn configure_endpoint(config: NetmodConfig) -> Result<ConfiguredEndpoint, Box<dyn std::error::Error>> {
    use NetmodConfig::*;
    Ok(match config.clone() {
        Udp(c) => { ConfiguredEndpoint { config, endpoint: Arc::new(UdpEndpoint::spawn(c.port)) } },
        Overlay(_) => unimplemented!()
    })
}
