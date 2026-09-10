use crate::{
    cli::{RouterSubcmd, RouterV2Subcmd},
    commands::RpcCommand,
    proto::Modules,
};
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

    fn space_name(&self, space: i32) -> &'static str {
        match proto::IndexSpace::try_from(space) {
            Ok(proto::IndexSpace::NodeSpace) => "node",
            _ => "user",
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
            RouterSubcmd::V2 { view } => Router {
                message: Some(router::Message::RouterV2Request(proto::RouterV2Request {
                    view: match view {
                        RouterV2Subcmd::Status => proto::RouterV2View::Status as i32,
                        RouterV2Subcmd::Table => proto::RouterV2View::Table as i32,
                        RouterV2Subcmd::Neighbours => proto::RouterV2View::Neighbours as i32,
                    },
                })),
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
            Some(router::Message::RouterV2Status(r)) => {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "node_id": bs58::encode(&r.node_id).into_string(),
                            "propagation_form": if r.is_node_form { "node" } else { "user" },
                            "is_gateway": r.is_gateway,
                            "seq_num": r.seq_num,
                            "manifest_version": r.manifest_version,
                            "manifest_entries": r.manifest_entries,
                            "manifest_log_base": r.manifest_log_base,
                            "neighbours": r.neighbours,
                            "user_entries": r.user_entries,
                            "node_entries": r.node_entries,
                            "user_dict_size": r.user_dict_size,
                            "node_dict_size": r.node_dict_size,
                            "pending_user_intros": r.pending_user_intros,
                            "pending_node_intros": r.pending_node_intros,
                            "profile_fetches_in_flight": r.profile_fetches_in_flight,
                            "manifest_requests_outstanding": r.manifest_requests_outstanding,
                        }))?
                    );
                } else {
                    println!("\nrouter_v2 status\n");
                    println!(
                        "  node id            {}",
                        bs58::encode(&r.node_id).into_string()
                    );
                    println!(
                        "  propagation form   {} (§3.2)",
                        if r.is_node_form {
                            "node entry"
                        } else {
                            "user entry"
                        }
                    );
                    println!(
                        "  gateway            {} (§2.3)",
                        if r.is_gateway { "yes" } else { "no" }
                    );
                    println!("  seq num            {}", r.seq_num);
                    println!();
                    println!(
                        "  own manifest       v{}, {} entr{}, log_base {}",
                        r.manifest_version,
                        r.manifest_entries,
                        if r.manifest_entries == 1 { "y" } else { "ies" },
                        r.manifest_log_base
                    );
                    println!("  neighbours         {}", r.neighbours);
                    println!(
                        "  routing entries    {} user / {} node",
                        r.user_entries, r.node_entries
                    );
                    println!(
                        "  dictionaries       {} user / {} node",
                        r.user_dict_size, r.node_dict_size
                    );
                    println!(
                        "  pending intros     {} user / {} node (§3.8)",
                        r.pending_user_intros, r.pending_node_intros
                    );
                    println!(
                        "  in flight          {} profile fetch(es), {} manifest request(s)",
                        r.profile_fetches_in_flight, r.manifest_requests_outstanding
                    );
                    println!();
                }
            }
            Some(router::Message::RouterV2Table(r)) => {
                if json {
                    let entries: Vec<serde_json::Value> = r
                        .entries
                        .iter()
                        .map(|e| {
                            serde_json::json!({
                                "space": self.space_name(e.space),
                                "index": e.index,
                                "target_id": bs58::encode(&e.target_id).into_string(),
                                "seq_num": e.seq_num,
                                "metric": e.metric,
                                "hop_count": e.hop_count,
                                "local_only": e.local_only,
                                "next_hop_index": e.next_hop_index,
                                "next_hop_id": bs58::encode(&e.next_hop_id).into_string(),
                                "transport": self.module_name(e.transport),
                                "age_ms": e.age_ms,
                            })
                        })
                        .collect();
                    println!("{}", serde_json::to_string_pretty(&entries)?);
                } else {
                    println!("\nrouter_v2 routing table\n");
                    if r.entries.is_empty() {
                        println!("  (empty)\n");
                    } else {
                        println!("  space | idx | target        | seq   | metric | hops | local | transport | via           | age");
                        for e in &r.entries {
                            println!(
                                "  {:5} | {:3} | {:13} | {:5} | {:6} | {:4} | {:5} | {:9} | {:13} | {}s",
                                self.space_name(e.space),
                                e.index,
                                bs58::encode(&e.target_id).into_string(),
                                e.seq_num,
                                e.metric,
                                e.hop_count,
                                if e.local_only { "yes" } else { "no" },
                                self.module_name(e.transport),
                                bs58::encode(&e.next_hop_id).into_string(),
                                e.age_ms / 1000,
                            );
                        }
                        println!();
                    }
                }
            }
            Some(router::Message::RouterV2Neighbours(r)) => {
                if json {
                    let ns: Vec<serde_json::Value> = r.neighbours.iter().map(|n| serde_json::json!({
                        "peer_id": bs58::encode(&n.peer_id).into_string(),
                        "node_id": bs58::encode(&n.node_id).into_string(),
                        "transports": n.transports.iter().map(|t| self.module_name(*t)).collect::<Vec<_>>(),
                        "rtt_micros": n.rtt_micros,
                        "user_mirror_size": n.user_mirror_size,
                        "node_mirror_size": n.node_mirror_size,
                        "dump_stale_users": n.dump_stale_users,
                        "dump_stale_nodes": n.dump_stale_nodes,
                    })).collect();
                    println!("{}", serde_json::to_string_pretty(&ns)?);
                } else {
                    println!("\nrouter_v2 neighbours\n");
                    if r.neighbours.is_empty() {
                        println!("  (none)\n");
                    } else {
                        for n in &r.neighbours {
                            let transports: Vec<&str> =
                                n.transports.iter().map(|t| self.module_name(*t)).collect();
                            println!(
                                "  node {} via peer {}",
                                bs58::encode(&n.node_id).into_string(),
                                bs58::encode(&n.peer_id).into_string()
                            );
                            println!("    transports    {}", transports.join(", "));
                            println!("    rtt           {} µs", n.rtt_micros);
                            println!(
                                "    mirrors       {} user / {} node binding(s) (§3.6)",
                                n.user_mirror_size, n.node_mirror_size
                            );
                            if n.dump_stale_users > 0 || n.dump_stale_nodes > 0 {
                                println!(
                                    "    dump pending  {} user / {} node unconfirmed (§8.4)",
                                    n.dump_stale_users, n.dump_stale_nodes
                                );
                            }
                        }
                        println!();
                    }
                }
            }
            _ => {
                return Err("unprocessable RPC router message".into());
            }
        };
        Ok(())
    }
}
