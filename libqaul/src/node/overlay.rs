/**
 * # Internet Overlay Module
 * 
 * **Statically connect to peers in the Internet.**
 * 
 * This module connects to static peers in the internet.
 * The overlay peers are read from the config file:
 * 
 * ```toml
 * [node]
 * peers = ["/ip4/144.91.74.192/tcp/9229"]
 * ```
 */
use libp2p::{
    swarm::Swarm,
};
use log::info;
use crate::node::mdns::{
    QaulBehaviour
};
use crate::configuration::Configuration;

#[derive(Debug)]
pub struct Overlay {

}

impl Overlay {
    pub fn init( config: &Configuration, swarm: &mut Swarm<QaulBehaviour> ) {
        for addr in &config.node.peers {
            match addr.parse() {
                Ok(addr) => match swarm.dial_addr(addr) {
                    Ok(_) => info!("peer connected"),
                    Err(error) => info!("peer swarm dial error: {:?}", error),
                },
                Err(error) => info!("peer address parse error: {:?}", error),
            }
        }
    }
}
