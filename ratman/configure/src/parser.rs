use crate::config::{Endpoint, Network, Params, Patch};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// Parse a json configuration into a network config
///
/// Check the `config` module for details.  Following is a sample
/// configuration to get you started.  Alternatively, you can also use
/// the `config` types directly.
///
/// ```rust
/// # let json = r#"
/// {
///   "endpoints": [
///     {
///       "id": 0,
///       "type": "tcp",
///       "params": {
///         "addr": "0.0.0.0",
///         "port": 9000,
///         "peers": ["127.0.0.1:8080"],
///         "dynamic": false
///       }
///     },
///     {
///       "id": 1,
///       "type": "local-udp",
///       "params": { "addr": "0.0.0.0" }
///     }
///   ],
///   "patches": {
///     "0": "external",
///     "1": "external"
///   }
/// }
/// # "#;
/// # ratman_configure::parse_json(&json);
/// ```
pub fn parse_json(cfg: &str) -> Network {
    let mut map: Map<String, Value> = serde_json::from_str(cfg).unwrap();

    let endpoints = parse_endpoints(map.remove("endpoints").expect("Missing field `endpoints`"));
    let patches = parse_patches(map.remove("patches").expect("Missing field `patches`"));

    Network { endpoints, patches }
}

fn parse_endpoints(eps: Value) -> BTreeMap<usize, Endpoint> {
    match eps {
        Value::Array(vec) => vec
            .into_iter()
            .map(|hash| match hash {
                Value::Object(hash) => Endpoint {
                    id: hash
                        .get("id")
                        .expect("Endpoint has no `id` field")
                        .as_u64()
                        .expect("Id needs to be a number!") as usize,
                    params: match hash
                        .get("type")
                        .expect("Endpoint has no `type` field")
                        .as_str()
                    {
                        Some("virtual") => Params::Virtual,
                        Some("tcp") => {
                            match hash.get("params").expect("Endpoint has no `params` field") {
                                Value::Object(params) => Params::Tcp {
                                    addr: params
                                        .get("addr")
                                        .and_then(|addr| addr.as_str().map(|s| s.to_owned()))
                                        .expect("Missing endpoint `addr` param"),
                                    port: params
                                        .get("port")
                                        .and_then(|addr| addr.as_u64().map(|port| port as u16))
                                        .expect("Missing endpoint `port` param"),
                                    peers: params
                                        .get("peers")
                                        .and_then(|addr| {
                                            addr.as_array().map(|vec| {
                                                vec.iter()
                                                    .map(|val| {
                                                        val.to_string()
                                                            .replace("\"", "")
                                                            .parse()
                                                            .unwrap()
                                                    })
                                                    .collect()
                                            })
                                        })
                                        .expect("Missing endpoint `port` param"),
                                    dynamic: params
                                        .get("dynamic")
                                        .and_then(|addr| addr.as_bool())
                                        .expect("Missing endpoint `bool` param"),
                                },
                                _ => unimplemented!(),
                            }
                        }
                        Some("local-udp") => {
                            match hash.get("params").expect("Endpoint has no `params` field") {
                                Value::Object(params) => Params::LocalUpd {
                                    addr: params
                                        .get("addr")
                                        .and_then(|addr| addr.as_str().map(|s| s.to_owned()))
                                        .expect("Missing endpoint `addr` param"),
                                },
                                _ => unimplemented!(),
                            }
                        }
                        tt => panic!(format!("Unknown type state: {:?}", tt)),
                    },
                },
                _ => unimplemented!(),
            })
            .map(|ep| (ep.id, ep))
            .collect(),
        _ => unimplemented!(),
    }
}

fn parse_patches(patch: Value) -> BTreeMap<usize, Patch> {
    patch
        .as_object()
        .map(|map| {
            map.into_iter()
                .map(|(k, v)| {
                    (
                        str::parse(&k).unwrap(),
                        match v {
                            Value::String(s) if s == "external" => Patch::External,
                            Value::Number(id) => {
                                Patch::Internal(id.as_u64().expect("Invalid Id!") as usize)
                            }
                            _ => panic!("Invalid patch value!"),
                        },
                    )
                })
                .collect()
        })
        .expect("Missing field `patches`")
}
