use crate::{cli::RouterSubcmd, commands::RpcCommand, proto::Modules};
use prost::Message;

use proto::{router, ConnectionModule, Router};
use qaul_proto::qaul_rpc_router as proto;

impl RouterSubcmd {
    fn module_name(&self, module: i32) -> &'static str {
        match ConnectionModule::try_from(module) {
            Ok(ConnectionModule::Lan) => "LAN",
            Ok(ConnectionModule::Internet) => "Internet",
            Ok(ConnectionModule::Ble) => "BLE",
            Ok(ConnectionModule::Local) => "Local",
            _ => "None",
        }
    }

    fn neighbours_to_json(&self, entries: &[proto::NeighboursEntry]) -> Vec<serde_json::Value> {
        entries
            .iter()
            .map(|e| {
                serde_json::json!({
                    "node_id": bs58::encode(&e.node_id).into_string(),
                    "rtt": e.rtt,
                })
            })
            .collect()
    }

    fn connections_to_json(
        &self,
        entries: &[proto::ConnectionsUserEntry],
    ) -> Vec<serde_json::Value> {
        entries
            .iter()
            .map(|e| {
                let connections: Vec<serde_json::Value> = e
                    .connections
                    .iter()
                    .map(|c| {
                        serde_json::json!({
                            "rtt": c.rtt,
                            "hop_count": c.hop_count,
                            "via": bs58::encode(&c.via).into_string(),
                        })
                    })
                    .collect();
                serde_json::json!({
                    "user_id": bs58::encode(&e.user_id).into_string(),
                    "connections": connections,
                })
            })
            .collect()
    }

    fn print_neighbours(&self, label: &str, entries: &[proto::NeighboursEntry]) {
        println!("{} Neighbours:", label);
        if entries.is_empty() {
            println!("  (none)");
        }
        for e in entries {
            println!(
                "  {} | {} ms rtt",
                bs58::encode(&e.node_id).into_string(),
                e.rtt
            );
        }
    }

    fn print_connections(&self, label: &str, entries: &[proto::ConnectionsUserEntry]) {
        println!("{} Connections:", label);
        if entries.is_empty() {
            println!("  (none)");
        }
        for e in entries {
            println!("  {}", bs58::encode(&e.user_id).into_string());
            for c in &e.connections {
                println!(
                    "    * rtt={} hops={} via={}",
                    c.rtt,
                    c.hop_count,
                    bs58::encode(&c.via).into_string()
                );
            }
        }
    }
}

impl RpcCommand for RouterSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let proto_message = match self {
            RouterSubcmd::Table => Router {
                message: Some(router::Message::RoutingTableRequest(
                    proto::RoutingTableRequest {},
                )),
            },
            RouterSubcmd::Neighbours => Router {
                message: Some(router::Message::NeighboursRequest(
                    proto::NeighboursRequest {},
                )),
            },
            RouterSubcmd::Connections => Router {
                message: Some(router::Message::ConnectionsRequest(
                    proto::ConnectionsRequest {},
                )),
            },
        };

        Ok((proto_message.encode_to_vec(), Modules::Router))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        let router = Router::decode(data)?;
        match router.message {
            Some(router::Message::RoutingTable(r)) => {
                if json {
                    let entries: Vec<serde_json::Value> = r
                        .routing_table
                        .iter()
                        .map(|entry| {
                            let connections: Vec<serde_json::Value> = entry
                                .connections
                                .iter()
                                .map(|c| {
                                    serde_json::json!({
                                        "module": self.module_name(c.module),
                                        "rtt": c.rtt,
                                        "hop_count": c.hop_count,
                                        "via": bs58::encode(&c.via).into_string(),
                                    })
                                })
                                .collect();
                            serde_json::json!({
                                "user_id": bs58::encode(&entry.user_id).into_string(),
                                "connections": connections,
                            })
                        })
                        .collect();
                    println!("{}", serde_json::to_string_pretty(&entries)?);
                } else {
                    println!("");
                    println!("Routing Table");
                    println!("No. | User ID");
                    println!("      * Module | RTT (ms) | Hops | Via");
                    let mut line = 1;
                    for entry in r.routing_table {
                        println!("{} | {}", line, bs58::encode(&entry.user_id).into_string());
                        for c in &entry.connections {
                            println!(
                                "      * {} | {} | {} | {}",
                                self.module_name(c.module),
                                c.rtt,
                                c.hop_count,
                                bs58::encode(&c.via).into_string(),
                            );
                        }
                        line += 1;
                    }
                    println!("");
                }
            }
            Some(router::Message::NeighboursList(r)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "lan": self.neighbours_to_json(&r.lan),
                            "internet": self.neighbours_to_json(&r.internet),
                            "ble": self.neighbours_to_json(&r.ble),
                        }))?
                    );
                } else {
                    println!("");
                    println!("Neighbours");
                    self.print_neighbours("LAN", &r.lan);
                    self.print_neighbours("Internet", &r.internet);
                    self.print_neighbours("BLE", &r.ble);
                    println!("");
                }
            }
            Some(router::Message::ConnectionsList(r)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "lan": self.connections_to_json(&r.lan),
                            "internet": self.connections_to_json(&r.internet),
                            "ble": self.connections_to_json(&r.ble),
                            "local": self.connections_to_json(&r.local),
                        }))?
                    );
                } else {
                    println!("");
                    println!("Connections");
                    self.print_connections("LAN", &r.lan);
                    self.print_connections("Internet", &r.internet);
                    self.print_connections("BLE", &r.ble);
                    self.print_connections("Local", &r.local);
                    println!("");
                }
            }
            _ => {
                log::error!("unprocessable RPC router message");
            }
        };
        Ok(())
    }
}
