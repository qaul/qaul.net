/**
 * Table of all direct neighbour nodes
 * 
 * There is a table per connection module.
 */

use libp2p::PeerId;
use std::collections::HashMap;
use state::Storage;
use std::sync::RwLock;
use log::{error, info};
use crate::connections::ConnectionModule;

// mutable state of Neighbours table per ConnectionModule
static INTERNET: Storage<RwLock<Neighbours>> = Storage::new();
static LAN: Storage<RwLock<Neighbours>> = Storage::new();

pub struct Neighbours {
    nodes: HashMap<PeerId, Neighbour>,
}

pub struct Neighbour {
    //id: PeerId,
    rtt: u32,        // round trip time in micro seconds
}

impl Neighbours {
    pub fn init() {
        // neighbours table for internet connection module
        let internet = Neighbours { nodes: HashMap::new() };
        INTERNET.set(RwLock::new(internet));

        // neighbours table for lan connection module
        let lan = Neighbours { nodes: HashMap::new() };
        LAN.set(RwLock::new(lan));
    }

    /**
     * update table with a new value
     * 
     * If the node already exists, it updates it's rtt value.
     * If the node does not yet exist, it creates it.
     */
    pub fn update_node( module: ConnectionModule, node_id: PeerId, rtt: u32 ) {
        tracing::trace!("update_node {:?} {}", node_id, rtt);

        // get table
        let mut neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().write().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().write().unwrap(),
            ConnectionModule::None => return,
        }

        // get node from table
        let node_option = neighbours.nodes.get_mut( &node_id );
        if let Some(node) = node_option {
            tracing::trace!("update node in neighbours table");
            node.rtt = Self::calculate_rtt( node.rtt , rtt);
        }
        else {
            info!("add node to neighbours table");
            neighbours.nodes.insert( node_id, Neighbour { rtt } );
        } 
    }

    /**
     * Calculate average rtt
     */
    fn calculate_rtt( old_rtt: u32, new_rtt: u32 ) -> u32 {
        (old_rtt * 3 + new_rtt) / 4
    }

    /**
     * neighbours CLI commands
     */
    pub fn cli(cmd: &str) {        
        match cmd {
            // list neighbours
            cmd if cmd.starts_with("list") => {
                // display lan connection module neighbours
                {
                    println!("LAN neighbours:");
                    let lan = LAN.get().read().unwrap();

                    for (id, value) in &lan.nodes {
                        println!("{:?}, {} rtt", id, value.rtt);
                    }
                }
                
                // display internet connection module neighbours
                {
                    println!("Internet neighbours:");
                    let internet = INTERNET.get().write().unwrap();

                    for (id, value) in &internet.nodes {
                        println!("{:?}, {} rtt", id, value.rtt);
                    }
                }
            },
            _ => error!("unknown user command"),
        }
    }
}

