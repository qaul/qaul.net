// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Metrics RPC commands.

use prost::Message;
use serde_json::{json, Value};

use crate::{
    cli::{MetricsFormat, MetricsSubcmd},
    commands::RpcCommand,
    proto::Modules,
};

/// protobuf RPC definition
use qaul_proto::qaul_rpc_metrics as proto;

impl RpcCommand for MetricsSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let message = match self {
            MetricsSubcmd::Snapshot { .. } => {
                proto::metrics::Message::SnapshotRequest(proto::SnapshotRequest {})
            }
            MetricsSubcmd::Enable => {
                proto::metrics::Message::SetEnabledRequest(proto::SetEnabledRequest {
                    enabled: true,
                })
            }
            MetricsSubcmd::Disable => {
                proto::metrics::Message::SetEnabledRequest(proto::SetEnabledRequest {
                    enabled: false,
                })
            }
            MetricsSubcmd::Dtn => {
                proto::metrics::Message::DtnStorageRequest(proto::DtnStorageRequest {})
            }
            MetricsSubcmd::Transit => {
                proto::metrics::Message::TransitPeersRequest(proto::TransitPeersRequest {})
            }
            MetricsSubcmd::Storage => {
                proto::metrics::Message::UserStorageRequest(proto::UserStorageRequest {})
            }
        };
        let proto_message = proto::Metrics {
            message: Some(message),
        };
        Ok((proto_message.encode_to_vec(), Modules::Metrics))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        let response = proto::Metrics::decode(data)
            .map_err(|e| format!("metrics: failed to decode response: {e:?}"))?;

        match response.message {
            Some(proto::metrics::Message::SnapshotResponse(snapshot)) => {
                let format = match self {
                    MetricsSubcmd::Snapshot { format } => *format,
                    _ => MetricsFormat::Table,
                };
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&snapshot_json(&snapshot))?
                    );
                } else {
                    match format {
                        MetricsFormat::Prometheus => {
                            print!("{}", qauld_rpc::prometheus::format(&snapshot))
                        }
                        MetricsFormat::Table => print_snapshot(&snapshot),
                    }
                }
            }
            Some(proto::metrics::Message::Ack(_)) => {
                let enabled = matches!(self, MetricsSubcmd::Enable);
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&json!({ "enabled": enabled }))?
                    );
                } else {
                    println!("metrics {}", if enabled { "enabled" } else { "disabled" });
                }
            }
            Some(proto::metrics::Message::DtnStorageResponse(resp)) => {
                if json {
                    let obj = json!({
                        "v1": { "message_count": resp.v1_message_count, "bytes": resp.v1_bytes },
                        "v2": resp.v2.iter().map(pair_json).collect::<Vec<_>>(),
                    });
                    println!("{}", serde_json::to_string_pretty(&obj)?);
                } else {
                    println!(
                        "V1 (sender not recorded): {} messages, {} bytes",
                        resp.v1_message_count, resp.v1_bytes
                    );
                    println!("V2:");
                    print_pairs(&resp.v2);
                }
            }
            Some(proto::metrics::Message::TransitPeersResponse(resp)) => {
                if json {
                    let obj = json!({
                        "pairs": resp.pairs.iter().map(pair_json).collect::<Vec<_>>(),
                        "overflow": {
                            "message_count": resp.overflow_message_count,
                            "bytes": resp.overflow_bytes,
                        },
                    });
                    println!("{}", serde_json::to_string_pretty(&obj)?);
                } else {
                    print_pairs(&resp.pairs);
                    if resp.overflow_message_count > 0 {
                        println!(
                            "untracked pairs (table full): {} messages, {} bytes",
                            resp.overflow_message_count, resp.overflow_bytes
                        );
                    }
                }
            }
            Some(proto::metrics::Message::UserStorageResponse(resp)) => {
                let user_id = bs58::encode(&resp.user_id).into_string();
                if json {
                    let obj = json!({ "user_id": user_id, "bytes": resp.bytes });
                    println!("{}", serde_json::to_string_pretty(&obj)?);
                } else {
                    println!("{}: {} bytes", user_id, resp.bytes);
                }
            }
            Some(proto::metrics::Message::Error(e)) => {
                return Err(format!("metrics: error {}: {}", e.code, e.message).into());
            }
            other => {
                return Err(format!("metrics: unexpected response variant: {other:?}").into());
            }
        }
        Ok(())
    }
}

fn attributes_string(attributes: &[proto::Attribute]) -> String {
    attributes
        .iter()
        .map(|a| format!("{}={}", a.key, a.value))
        .collect::<Vec<_>>()
        .join(",")
}

fn print_snapshot(snapshot: &proto::SnapshotResponse) {
    if !snapshot.enabled {
        println!("metrics collection is disabled (enable with `metrics enable`)");
    }
    for metric in &snapshot.metrics {
        let unit = if metric.unit.is_empty() {
            String::new()
        } else {
            format!(" [{}]", metric.unit)
        };
        println!("{}{}", metric.name, unit);
        if metric.points.is_empty() {
            println!("  (no data)");
        }
        for point in &metric.points {
            let attributes = attributes_string(&point.attributes);
            match &point.value {
                Some(proto::data_point::Value::Counter(v)) => {
                    println!("  {:<40} {}", attributes, v)
                }
                Some(proto::data_point::Value::Histogram(h)) => {
                    let mean = if h.count > 0 {
                        h.sum / h.count as f64
                    } else {
                        0.0
                    };
                    println!(
                        "  {:<40} count={} sum={} mean={:.1}",
                        attributes, h.count, h.sum, mean
                    )
                }
                None => {}
            }
        }
    }
}

fn snapshot_json(snapshot: &proto::SnapshotResponse) -> Value {
    json!({
        "enabled": snapshot.enabled,
        "start_time_unix_nano": snapshot.start_time_unix_nano,
        "time_unix_nano": snapshot.time_unix_nano,
        "metrics": snapshot.metrics.iter().map(|m| json!({
            "name": m.name,
            "description": m.description,
            "unit": m.unit,
            "kind": proto::MetricKind::try_from(m.kind)
                .map(|k| k.as_str_name().to_lowercase())
                .unwrap_or_default(),
            "points": m.points.iter().map(|p| {
                let attributes: serde_json::Map<String, Value> = p
                    .attributes
                    .iter()
                    .map(|a| (a.key.clone(), Value::String(a.value.clone())))
                    .collect();
                let value = match &p.value {
                    Some(proto::data_point::Value::Counter(v)) => json!(v),
                    Some(proto::data_point::Value::Histogram(h)) => json!({
                        "bounds": h.bounds,
                        "bucket_counts": h.bucket_counts,
                        "sum": h.sum,
                        "count": h.count,
                    }),
                    None => Value::Null,
                };
                json!({ "attributes": attributes, "value": value })
            }).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

fn pair_json(pair: &proto::PeerPairStats) -> Value {
    json!({
        "sender_id": bs58::encode(&pair.sender_id).into_string(),
        "receiver_id": bs58::encode(&pair.receiver_id).into_string(),
        "message_count": pair.message_count,
        "bytes": pair.bytes,
    })
}

fn print_pairs(pairs: &[proto::PeerPairStats]) {
    if pairs.is_empty() {
        println!("  (none)");
        return;
    }
    let mut pairs: Vec<&proto::PeerPairStats> = pairs.iter().collect();
    pairs.sort_by_key(|pair| std::cmp::Reverse(pair.bytes));
    for pair in pairs {
        println!(
            "  {} -> {}: {} messages, {} bytes",
            bs58::encode(&pair.sender_id).into_string(),
            bs58::encode(&pair.receiver_id).into_string(),
            pair.message_count,
            pair.bytes
        );
    }
}
