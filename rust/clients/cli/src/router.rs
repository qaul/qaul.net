// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Router module functions

use super::rpc::Rpc;
use prost::Message;

/// protobuf RPC definition
use qaul_proto::qaul_rpc_router as proto;

/// router module function handling
pub struct Router {}

impl Router {
    /// CLI command interpretation
    ///
    /// The CLI commands of router module are processed here
    pub fn cli(command: &str) {
        match command {
            // request routing table,
            // with per module connectivity per user.
            cmd if cmd.starts_with("table list") => {
                Self::request_routing_table();
            }
            // request neighbours list of all neighbouring nodes.
            cmd if cmd.starts_with("neighbours list") => {
                Self::request_neighbours_list();
            }
            // request connections table, with all known connections
            // per connection module.
            cmd if cmd.starts_with("connections list") => {
                Self::request_connections_list();
            }
            // unknown command
            _ => log::error!("unknown router command"),
        }
    }

    /// create rpc request for routing table list
    fn request_routing_table() {
        // create request message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::RoutingTableRequest(
                proto::RoutingTableRequest {},
            )),
        };

        // send message
        Self::send_message(proto_message);
    }

    /// create rpc request for neighbours list
    fn request_neighbours_list() {
        // create request message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::NeighboursRequest(
                proto::NeighboursRequest {},
            )),
        };

        // send message
        Self::send_message(proto_message);
    }

    /// create rpc request for connections list
    fn request_connections_list() {
        // create request message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::ConnectionsRequest(
                proto::ConnectionsRequest {},
            )),
        };

        // send message
        Self::send_message(proto_message);
    }

    /// Encode and send protobuf message
    fn send_message(message: proto::Router) {
        // encode message
        let mut buf = Vec::with_capacity(message.encoded_len());
        message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Router.into(),
            "".to_string(),
        );
    }

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
                                let module =
                                    match proto::ConnectionModule::try_from(connection.module) {
                                        Ok(proto::ConnectionModule::None) => "None",
                                        Ok(proto::ConnectionModule::Lan) => "Lan",
                                        Ok(proto::ConnectionModule::Internet) => "Internet",
                                        Ok(proto::ConnectionModule::Ble) => "Ble",
                                        Ok(proto::ConnectionModule::Local) => "Local",
                                        Err(_) => "Unknown",
                                    };

                                // print connection entry
                                println!(
                                    "      * {} | {} | {} | {}",
                                    module,
                                    connection.rtt,
                                    connection.hop_count,
                                    via.into_string()
                                );
                            }

                            line += 1;
                        }
                        println!("");
                    }
                    Some(proto::router::Message::NeighboursList(proto_message)) => {
                        println!("");
                        println!("Neighbours List - All Nodes that are Direct Neighbours");
                        println!("");

                        println!("LAN Neighbours");
                        Self::rpc_display_neighbours_list(proto_message.lan);

                        println!("Internet Neighbours");
                        Self::rpc_display_neighbours_list(proto_message.internet);
                    }
                    Some(proto::router::Message::ConnectionsList(proto_message)) => {
                        println!("");
                        println!("Connections List - All Connections of this Node");
                        println!("");

                        println!("LAN Connections");
                        Self::rpc_display_connections_list(proto_message.lan);

                        println!("Internet Connections");
                        Self::rpc_display_connections_list(proto_message.internet);
                    }
                    _ => {
                        log::error!("unprocessable RPC router message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }

    /// Display Neighbours list
    fn rpc_display_neighbours_list(neighbours_list: Vec<proto::NeighboursEntry>) {
        for entry in neighbours_list {
            println!(
                "{}, {} rtt",
                bs58::encode(entry.node_id).into_string(),
                entry.rtt
            );
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
                println!(
                    "      * {} | {} | {:?}",
                    connection.rtt,
                    connection.hop_count,
                    bs58::encode(connection.via).into_string()
                );
            }
            line += 1;
        }
        println!("");
    }
}
