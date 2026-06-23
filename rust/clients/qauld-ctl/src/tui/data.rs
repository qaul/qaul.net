// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Direct RPC calls used by the TUI to populate its views. Each
//! function opens a fresh `SocketTransport`, does one round-trip,
//! and returns parsed rows. The TUI re-fetches periodically.

use std::time::Duration;

use futures::{SinkExt, StreamExt};
use prost::Message;
use qauld_rpc::transport::{ConnectInfo, SocketTransport};
use qauld_rpc::{proto, RpcTransport};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::codec::LengthDelimitedCodec;
use uuid::Uuid;

use super::app::{FeedRow, UserRow};

use qaul_proto::qaul_rpc_crypto as crypto_proto;
use qaul_proto::qaul_rpc_crypto::RotationEventKind;
use qaul_proto::qaul_rpc_dtn as dtn_proto;
use qaul_proto::qaul_rpc_feed as feed_proto;
use qaul_proto::qaul_rpc_router as router_proto;
use qaul_proto::qaul_rpc_subscribe as sub_proto;
use qaul_proto::qaul_rpc_user_accounts as ua_proto;
use qaul_proto::qaul_rpc_users as users_proto;

/// One event line pushed onto the UI's deque, tagged so the UI can
/// route DTN events to the DTN tab and everything else to the
/// generic events panel.
///
/// Some topics carry structured payloads the UI wants to merge
/// into a typed buffer rather than render as a string — that's
/// what `parsed` is for.
#[derive(Debug, Clone)]
pub struct EventLine {
    pub topic: String,
    pub text: String,
    pub parsed: ParsedEvent,
}

/// Structured payloads broken out for tabs that maintain their own
/// typed buffers (e.g. the Crypto tab merges these into its
/// `crypto_events` list so push and poll converge on the same view).
#[derive(Debug, Clone, Default)]
pub enum ParsedEvent {
    #[default]
    None,
    CryptoRotation(CryptoRotationEvent),
}

async fn open(connect: &ConnectInfo) -> Result<SocketTransport, Box<dyn std::error::Error>> {
    SocketTransport::connect(connect).await
}

async fn round_trip(
    transport: &mut SocketTransport,
    module: proto::Modules,
    data: Vec<u8>,
    timeout: Duration,
) -> Result<proto::QaulRpc, Box<dyn std::error::Error>> {
    let envelope = proto::QaulRpc {
        module: module.into(),
        request_id: Uuid::new_v4().to_string(),
        user_id: Vec::new(),
        data,
    };
    transport
        .request(envelope, timeout, true)
        .await?
        .ok_or_else(|| "no response".into())
}

/// Display string + raw PeerId bytes for the daemon's default
/// account. The bytes are required as the `user_id` field on every
/// command envelope sent to libqaul — sending `Vec::new()` makes
/// libqaul fail to decode the multihash and reject the request.
#[derive(Debug, Clone, Default)]
pub struct DefaultUser {
    pub label: String,
    pub id_bytes: Vec<u8>,
}

pub async fn fetch_default_user(
    connect: &ConnectInfo,
    timeout: Duration,
) -> Result<DefaultUser, Box<dyn std::error::Error>> {
    let req = ua_proto::UserAccounts {
        message: Some(ua_proto::user_accounts::Message::GetDefaultUserAccount(true)),
    };
    let mut t = open(connect).await?;
    let resp = round_trip(&mut t, proto::Modules::Useraccounts, req.encode_to_vec(), timeout).await?;
    let parsed = ua_proto::UserAccounts::decode(&resp.data[..])?;
    if let Some(ua_proto::user_accounts::Message::DefaultUserAccount(d)) = parsed.message {
        if let Some(acct) = d.my_user_account {
            let label = format!(
                "{} ({}…)",
                acct.name,
                &acct.id_base58[..12.min(acct.id_base58.len())]
            );
            return Ok(DefaultUser {
                label,
                id_bytes: acct.id,
            });
        }
    }
    Ok(DefaultUser {
        label: "(no default user)".to_string(),
        id_bytes: Vec::new(),
    })
}

pub async fn fetch_users(
    connect: &ConnectInfo,
    timeout: Duration,
) -> Result<Vec<UserRow>, Box<dyn std::error::Error>> {
    let req = users_proto::Users {
        message: Some(users_proto::users::Message::UserRequest(
            users_proto::UserRequest { offset: 0, limit: 0 },
        )),
    };
    let mut t = open(connect).await?;
    let resp = round_trip(&mut t, proto::Modules::Users, req.encode_to_vec(), timeout).await?;
    let parsed = users_proto::Users::decode(&resp.data[..])?;
    let mut rows = Vec::new();
    if let Some(users_proto::users::Message::UserList(list)) = parsed.message {
        for u in list.user {
            let connectivity = match users_proto::Connectivity::try_from(u.connectivity) {
                Ok(c) => c.as_str_name().to_string(),
                Err(_) => "?".to_string(),
            };
            rows.push(UserRow {
                name: u.name,
                id: bs58::encode(&u.id).into_string(),
                connectivity,
                bio: u.bio,
                profile_version: u.profile_version,
            });
        }
    }
    Ok(rows)
}

#[derive(Debug, Clone, Default)]
pub struct DtnState {
    pub used_size: u64,
    pub message_count: u32,
    pub unconfirmed_count: u32,
}

#[derive(Debug, Clone, Default)]
pub struct DtnConfig {
    pub total_size: u32,
    pub users: Vec<String>,
}

pub async fn fetch_dtn_state(
    connect: &ConnectInfo,
    timeout: Duration,
) -> Result<DtnState, Box<dyn std::error::Error>> {
    let req = dtn_proto::Dtn {
        message: Some(dtn_proto::dtn::Message::DtnStateRequest(
            dtn_proto::DtnStateRequest {},
        )),
    };
    let mut t = open(connect).await?;
    let resp = round_trip(&mut t, proto::Modules::Dtn, req.encode_to_vec(), timeout).await?;
    let parsed = dtn_proto::Dtn::decode(&resp.data[..])?;
    if let Some(dtn_proto::dtn::Message::DtnStateResponse(s)) = parsed.message {
        return Ok(DtnState {
            used_size: s.used_size,
            message_count: s.dtn_message_count,
            unconfirmed_count: s.unconfirmed_count,
        });
    }
    Err("unexpected DTN state response".into())
}

pub async fn fetch_dtn_config(
    connect: &ConnectInfo,
    timeout: Duration,
) -> Result<DtnConfig, Box<dyn std::error::Error>> {
    let req = dtn_proto::Dtn {
        message: Some(dtn_proto::dtn::Message::DtnConfigRequest(
            dtn_proto::DtnConfigRequest {},
        )),
    };
    let mut t = open(connect).await?;
    let resp = round_trip(&mut t, proto::Modules::Dtn, req.encode_to_vec(), timeout).await?;
    let parsed = dtn_proto::Dtn::decode(&resp.data[..])?;
    if let Some(dtn_proto::dtn::Message::DtnConfigResponse(c)) = parsed.message {
        return Ok(DtnConfig {
            total_size: c.total_size,
            users: c
                .users
                .iter()
                .map(|u| bs58::encode(u).into_string())
                .collect(),
        });
    }
    Err("unexpected DTN config response".into())
}

#[derive(Debug, Clone, Default)]
pub struct NetworkSnapshot {
    pub lan_peers: u32,
    pub internet_peers: u32,
    pub ble_peers: u32,
    pub local_peers: u32,
    /// Flat per-peer view: (module, user_id_base58, hop_count, rtt_ms).
    /// One row per (peer × module) so the same peer reachable via two
    /// transports gets two rows — matches what the router actually
    /// tracks.
    pub peers: Vec<PeerRow>,
}

#[derive(Debug, Clone)]
pub struct PeerRow {
    pub module: &'static str,
    pub user_id: String,
    pub hops: u32,
    pub rtt_ms: u32,
}

pub async fn fetch_network(
    connect: &ConnectInfo,
    timeout: Duration,
) -> Result<NetworkSnapshot, Box<dyn std::error::Error>> {
    let req = router_proto::Router {
        message: Some(router_proto::router::Message::ConnectionsRequest(
            router_proto::ConnectionsRequest {},
        )),
    };
    let mut t = open(connect).await?;
    let resp = round_trip(&mut t, proto::Modules::Router, req.encode_to_vec(), timeout).await?;
    let parsed = router_proto::Router::decode(&resp.data[..])?;
    let mut snap = NetworkSnapshot::default();
    if let Some(router_proto::router::Message::ConnectionsList(list)) = parsed.message {
        snap.lan_peers = list.lan.len() as u32;
        snap.internet_peers = list.internet.len() as u32;
        snap.ble_peers = list.ble.len() as u32;
        snap.local_peers = list.local.len() as u32;
        push_peers(&mut snap.peers, "LAN", &list.lan);
        push_peers(&mut snap.peers, "Internet", &list.internet);
        push_peers(&mut snap.peers, "BLE", &list.ble);
        push_peers(&mut snap.peers, "Local", &list.local);
    }
    Ok(snap)
}

fn push_peers(
    rows: &mut Vec<PeerRow>,
    module: &'static str,
    entries: &[router_proto::ConnectionsUserEntry],
) {
    for entry in entries {
        let best = entry
            .connections
            .iter()
            .min_by_key(|c| (c.hop_count, c.rtt));
        if let Some(c) = best {
            rows.push(PeerRow {
                module,
                user_id: bs58::encode(&entry.user_id).into_string(),
                hops: c.hop_count,
                rtt_ms: c.rtt,
            });
        } else {
            rows.push(PeerRow {
                module,
                user_id: bs58::encode(&entry.user_id).into_string(),
                hops: 0,
                rtt_ms: 0,
            });
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CryptoConfig {
    pub enabled: bool,
    pub volume_messages: u64,
}

#[derive(Debug, Clone)]
pub struct CryptoRotationEvent {
    pub timestamp_ms: u64,
    pub kind: &'static str,
    pub remote_id: String,
    pub primary_session_id: u32,
    pub draining_session_id: u32,
}

pub async fn fetch_crypto_config(
    connect: &ConnectInfo,
    timeout: Duration,
) -> Result<CryptoConfig, Box<dyn std::error::Error>> {
    let req = crypto_proto::Crypto {
        message: Some(crypto_proto::crypto::Message::GetConfigRequest(
            crypto_proto::GetConfigRequest {},
        )),
    };
    let mut t = open(connect).await?;
    let resp = round_trip(&mut t, proto::Modules::Crypto, req.encode_to_vec(), timeout).await?;
    let parsed = crypto_proto::Crypto::decode(&resp.data[..])?;
    if let Some(crypto_proto::crypto::Message::GetConfigResponse(c)) = parsed.message {
        return Ok(CryptoConfig {
            enabled: c.enabled,
            volume_messages: c.volume_messages,
        });
    }
    Err("unexpected crypto config response".into())
}

pub async fn fetch_crypto_events(
    connect: &ConnectInfo,
    timeout: Duration,
    since_ms: u64,
) -> Result<Vec<CryptoRotationEvent>, Box<dyn std::error::Error>> {
    let req = crypto_proto::Crypto {
        message: Some(crypto_proto::crypto::Message::GetEventsRequest(
            crypto_proto::GetRotationEventsRequest {
                since_ms,
                limit: 0,
            },
        )),
    };
    let mut t = open(connect).await?;
    let resp = round_trip(&mut t, proto::Modules::Crypto, req.encode_to_vec(), timeout).await?;
    let parsed = crypto_proto::Crypto::decode(&resp.data[..])?;
    if let Some(crypto_proto::crypto::Message::GetEventsResponse(r)) = parsed.message {
        return Ok(r.events.iter().map(rotation_event_to_row).collect());
    }
    Err("unexpected crypto events response".into())
}

pub async fn fetch_feed(
    connect: &ConnectInfo,
    timeout: Duration,
) -> Result<Vec<FeedRow>, Box<dyn std::error::Error>> {
    let req = feed_proto::Feed {
        message: Some(feed_proto::feed::Message::Request(
            feed_proto::FeedMessageRequest {
                last_received: Vec::new(),
                last_index: 0,
                offset: 0,
                limit: 0,
            },
        )),
    };
    let mut t = open(connect).await?;
    let resp = round_trip(&mut t, proto::Modules::Feed, req.encode_to_vec(), timeout).await?;
    let parsed = feed_proto::Feed::decode(&resp.data[..])?;
    let mut rows = Vec::new();
    if let Some(feed_proto::feed::Message::Received(list)) = parsed.message {
        for m in list.feed_message {
            rows.push(FeedRow {
                index: m.index,
                sender: m.sender_id_base58,
                content: m.content,
                time_sent: m.time_sent,
            });
        }
    }
    Ok(rows)
}

pub async fn send_feed(
    connect: &ConnectInfo,
    body: &str,
    user_id: &[u8],
    timeout: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    if user_id.is_empty() {
        return Err(
            "cannot send feed message: no default user account on this node \
             (create one with `qauld-ctl account create`)"
                .into(),
        );
    }
    let req = feed_proto::Feed {
        message: Some(feed_proto::feed::Message::Send(feed_proto::SendMessage {
            content: body.to_string(),
        })),
    };
    let envelope = proto::QaulRpc {
        module: proto::Modules::Feed.into(),
        request_id: Uuid::new_v4().to_string(),
        user_id: user_id.to_vec(),
        data: req.encode_to_vec(),
    };
    let mut t = open(connect).await?;
    t.request(envelope, timeout, false).await?;
    Ok(())
}

/// Subscribe and stream events into the supplied channel. Loops
/// until the connection drops.
pub async fn spawn_subscribe(
    connect: ConnectInfo,
    tx: UnboundedSender<EventLine>,
) -> Result<(), Box<dyn std::error::Error>> {
    let transport = SocketTransport::connect(&connect).await?;
    let mut framed = transport.into_framed();

    let envelope = sub_proto::Subscribe {
        message: Some(sub_proto::subscribe::Message::Request(
            sub_proto::SubscribeRequest { topics: Vec::new() },
        )),
    };
    let qaul_rpc = proto::QaulRpc {
        module: proto::Modules::Subscribe.into(),
        request_id: Uuid::new_v4().to_string(),
        user_id: Vec::new(),
        data: envelope.encode_to_vec(),
    };
    let mut buf = Vec::with_capacity(qaul_rpc.encoded_len());
    qaul_rpc.encode(&mut buf)?;
    framed.send(buf.into()).await?;

    while let Some(frame) = framed.next().await {
        let bytes = frame?;
        let outer = proto::QaulRpc::decode(&bytes[..])?;
        if let Some(line) = format_event(&outer.data) {
            if tx.send(line).is_err() {
                break;
            }
        }
    }
    Ok(())
}

fn format_event(data: &[u8]) -> Option<EventLine> {
    let env = sub_proto::Subscribe::decode(data).ok()?;
    let event = match env.message? {
        sub_proto::subscribe::Message::Event(e) => e,
        _ => return None,
    };
    let topic = event.topic.clone();
    let mut parsed = ParsedEvent::None;
    let text = match event.topic.as_str() {
        "chat.message" => match sub_proto::ChatMessageEvent::decode(&event.payload[..]) {
            Ok(m) => format!(
                "[{}] chat.message {}…: {}",
                event.timestamp,
                &bs58::encode(&m.sender_id).into_string()[..8],
                m.content
            ),
            Err(_) => format!("[{}] chat.message <decode failed>", event.timestamp),
        },
        "peers.connected" => match sub_proto::PeerEvent::decode(&event.payload[..]) {
            Ok(p) => format!(
                "[{}] peers.connected {}",
                event.timestamp,
                &bs58::encode(&p.peer_id).into_string()[..16]
            ),
            Err(_) => format!("[{}] peers.connected <decode failed>", event.timestamp),
        },
        "dtn.delivery_response" => {
            match sub_proto::DtnDeliveryResponseEvent::decode(&event.payload[..]) {
                Ok(d) => {
                    let status = match d.response_type {
                        1 => "accepted",
                        2 => "rejected",
                        _ => "unknown",
                    };
                    let reason = if d.reason != 0 {
                        format!(" reason={}", d.reason)
                    } else {
                        String::new()
                    };
                    format!(
                        "[{}] delivery {} via {}…  sig {}…{}",
                        event.timestamp,
                        status,
                        &bs58::encode(&d.storage_node).into_string()[..10],
                        &bs58::encode(&d.signature).into_string()[..8],
                        reason,
                    )
                }
                Err(_) => format!("[{}] dtn.delivery_response <decode failed>", event.timestamp),
            }
        }
        "crypto.rotation" => match crypto_proto::RotationEvent::decode(&event.payload[..]) {
            Ok(r) => {
                let row = rotation_event_to_row(&r);
                let line = format!(
                    "[{}] crypto.rotation {} remote={}… p={} d={}",
                    event.timestamp,
                    row.kind,
                    &row.remote_id[..8.min(row.remote_id.len())],
                    row.primary_session_id,
                    row.draining_session_id,
                );
                parsed = ParsedEvent::CryptoRotation(row);
                line
            }
            Err(_) => format!("[{}] crypto.rotation <decode failed>", event.timestamp),
        },
        other => format!(
            "[{}] {} ({} bytes)",
            event.timestamp,
            other,
            event.payload.len()
        ),
    };
    Some(EventLine { topic, text, parsed })
}

fn rotation_event_to_row(ev: &crypto_proto::RotationEvent) -> CryptoRotationEvent {
    let kind = match RotationEventKind::try_from(ev.kind) {
        Ok(RotationEventKind::Rotated) => "rotated",
        Ok(RotationEventKind::DrainCompleted) => "drain_completed",
        Ok(RotationEventKind::MessageDroppedPostDrain) => "msg_dropped_post_drain",
        _ => "unspecified",
    };
    CryptoRotationEvent {
        timestamp_ms: ev.timestamp_ms,
        kind,
        remote_id: bs58::encode(&ev.remote_id).into_string(),
        primary_session_id: ev.primary_session_id,
        draining_session_id: ev.draining_session_id,
    }
}

// Unused helpers from the framing codec; kept to compile if needed.
#[allow(dead_code)]
fn _codec_dummy() -> LengthDelimitedCodec {
    LengthDelimitedCodec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression test for the InvalidMultihash(UnexpectedEof) error
    /// libqaul logged when the TUI sent a feed message with an empty
    /// `user_id` on the QaulRpc envelope. The fix refuses to issue
    /// the request and returns a clear error to the UI instead.
    #[tokio::test]
    async fn send_feed_refuses_empty_user_id() {
        let connect = ConnectInfo {
            socket: Some("/nonexistent/should-never-be-opened".into()),
            dir: None,
        };
        // Empty user_id must short-circuit before any socket I/O.
        let res = send_feed(&connect, "hello", &[], Duration::from_secs(1)).await;
        let err = res.expect_err("expected Err for empty user_id");
        let msg = err.to_string();
        assert!(
            msg.contains("no default user account"),
            "error message should reference the missing account, got: {msg}"
        );
    }
}
