//! Table of all direct neighbour nodes
//! 
//! There is a table per connection module.

use libp2p::PeerId;
use state::Storage;
use std::{
    collections::HashMap,
    sync::RwLock,
    time::SystemTime,
};

use crate::connections::ConnectionModule;
use super::info::RouterInfo;

/// mutable state of Neighbours table per ConnectionModule
static INTERNET: Storage<RwLock<Neighbours>> = Storage::new();
static LAN: Storage<RwLock<Neighbours>> = Storage::new();

pub struct Neighbours {
    nodes: HashMap<PeerId, Neighbour>,
}

pub struct Neighbour {
    /// round trip time in micro seconds
    rtt: u32,
    /// when was this node last seen
    updated_at: SystemTime,
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

    /// update table with a new value
    /// 
    /// If the node already exists, it updates it's rtt value.
    /// If the node does not yet exist, it creates it.
    pub fn update_node( module: ConnectionModule, node_id: PeerId, rtt: u32 ) {
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
            node.rtt = Self::calculate_rtt( node.rtt , rtt);
            node.updated_at = SystemTime::now();
        }
        else {
            log::debug!("add node {:?} to neighbours table", node_id);
            neighbours.nodes.insert( node_id, Neighbour { rtt, updated_at: SystemTime::now() } );

            // add neighbour in RouterInfo neighbours table
            RouterInfo::add_neighbour(node_id);
        } 
    }

    /// Delete Neighbour
    pub fn delete( module: ConnectionModule, node_id: PeerId ) {
        // get table
        let mut neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().write().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().write().unwrap(),
            ConnectionModule::None => return,
        }

        // delete entry
        neighbours.nodes.remove( &node_id );
    }

    /// Calculate average rtt
    fn calculate_rtt( old_rtt: u32, new_rtt: u32 ) -> u32 {
        (old_rtt * 3 + new_rtt) / 4
    }

    /// get rtt for a neighbour
    /// returns the round trip time for the neighbour in the 
    /// connection module.
    /// If the neighbour does not exist, it returns None.
    pub fn get_rtt( neighbour_id: &PeerId, module: &ConnectionModule ) -> Option<u32> {
        // get table
        let neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().read().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().read().unwrap(),
            ConnectionModule::None => return None,
        }

        // search for neighbour
        if let Some(neighbour) = neighbours.nodes.get(neighbour_id) {
            return Some(neighbour.rtt)
        } else {
            return None
        }
    }

    /// Is this node ID a neighbour in any module?
    /// returns the first found module or `None`
    pub fn is_neighbour( node_id: &PeerId ) -> ConnectionModule {
        // check if neighbour is in Lan table
        {
            let lan = LAN.get().read().unwrap();
            if lan.nodes.contains_key(node_id) {
                return ConnectionModule::Lan
            }
        }
        // check if neighbour exists in Internet table
        {
            let internet = INTERNET.get().read().unwrap();
            if internet.nodes.contains_key(node_id) {
                return ConnectionModule::Internet
            }
        }

        ConnectionModule::None
    }

    /// neighbours CLI commands
    /// 
    /// you get here with the commands:
    /// ```
    /// router neighbours list
    /// ```
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
            _ => log::error!("unknown user command"),
        }
    }
}

