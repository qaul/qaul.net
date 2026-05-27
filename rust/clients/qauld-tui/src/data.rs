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

use crate::app::{FeedRow, UserRow};

use qaul_proto::qaul_rpc_feed as feed_proto;
use qaul_proto::qaul_rpc_subscribe as sub_proto;
use qaul_proto::qaul_rpc_user_accounts as ua_proto;
use qaul_proto::qaul_rpc_users as users_proto;

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
    tx: UnboundedSender<String>,
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

fn format_event(data: &[u8]) -> Option<String> {
    let env = sub_proto::Subscribe::decode(data).ok()?;
    let event = match env.message? {
        sub_proto::subscribe::Message::Event(e) => e,
        _ => return None,
    };
    Some(match event.topic.as_str() {
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
        other => format!(
            "[{}] {} ({} bytes)",
            event.timestamp,
            other,
            event.payload.len()
        ),
    })
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
