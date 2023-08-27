// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Router module functions

use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.router.rs"); }

/// router module function handling
pub struct Router {}

impl Router {

    /// Process received RPC message
    /// 
    /// Decodes received protobuf encoded binary RPC message
    /// of the router module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Router::decode(&data[..]) {
            Ok(router) => {
                match router.message {
                    Some(proto::router::Message::RoutingTable(proto_message)) => {
                        println!("");
                        println!("Routing Table");
                        println!("No. | User ID");
                        println!("      * Connection Module | RTT in ms | hop count | Via Neighbour Node Id");
        
                        let mut line = 1;

                        // loop through all routing table entries
                        for entry in proto_message.routing_table {
                            // print routing table entry header
                            println!("{} | {}", line, bs58::encode(entry.user_id).into_string());

                            // loop through all connection entries
                            for connection in &entry.connections {
                                // get node id as string
                                let via = bs58::encode(connection.via.clone());

                                // get enum name as string
                                let module = match proto::ConnectionModule::from_i32(connection.module) {
                                    Some(proto::ConnectionModule::None) => "None",
                                    Some(proto::ConnectionModule::Lan) => "Lan",
                                    Some(proto::ConnectionModule::Internet) => "Internet",
                                    Some(proto::ConnectionModule::Ble) => "Ble",
                                    Some(proto::ConnectionModule::Local) => "Local",
                                    None => "Unknown",
                                };

                                // print connection entry
                                println!("      * {} | {} | {} | {}", module, connection.rtt, connection.hop_count, via.into_string());
                            }

                            line += 1;
                        }
                        println!("");
                    },
                    Some(proto::router::Message::NeighboursList(proto_message)) => {
                        println!("");
                        println!("Neighbours List - All Nodes that are Direct Neighbours");
                        println!("");
                        
                        println!("LAN Neighbours");
                        Self::rpc_display_neighbours_list(proto_message.lan);

                        println!("Internet Neighbours");
                        Self::rpc_display_neighbours_list(proto_message.internet);
                    },
                    Some(proto::router::Message::ConnectionsList(proto_message)) => {
                        println!("");
                        println!("Connections List - All Connections of this Node");
                        println!("");

                        println!("LAN Connections");
                        Self::rpc_display_connections_list(proto_message.lan);

                        println!("Internet Connections");
                        Self::rpc_display_connections_list(proto_message.internet);
                    },
                    _ => {
                        log::error!("unprocessable RPC router message");
                    },
                }    
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }

    /// Display Neighbours list
    fn rpc_display_neighbours_list(neighbours_list: Vec<proto::NeighboursEntry>) {
        for entry in neighbours_list {
            println!("{}, {} rtt", bs58::encode(entry.node_id).into_string(), entry.rtt);
        }
        println!("");
    }

    /// Display Connection per Module
    fn rpc_display_connections_list(connections_list: Vec<proto::ConnectionsUserEntry>) {
        println!("No. | User ID");
        println!("      * RTT in ms | hop count | Via Neighbour Node Id");

        let mut line = 1;

        for entry in connections_list {
            println!("{} | {:?}", line, bs58::encode(entry.user_id).into_string());
            // loop through all neighbour entries of a user entry
            for connection in entry.connections {
                println!("      * {} | {} | {:?}", connection.rtt, connection.hop_count, bs58::encode(connection.via).into_string());
            }
            line += 1;
        }
        println!("");
    }
}