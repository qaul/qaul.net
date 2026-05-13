// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Long-running event subscription mode for qauld-ctl.
//!
//! Sends one `SubscribeRequest` to libqaul and then prints every `Event`
//! pushed back over the same socket. Stops on Ctrl-C or when the daemon
//! closes the connection.

use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use uuid::Uuid;

use crate::cli::Cli;

pub use qaul_proto::qaul_rpc as proto;
pub use qaul_proto::qaul_rpc_subscribe as proto_sub;

/// Run the subscribe command: connect, send SubscribeRequest, print events.
pub async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let (client, addr) = crate::connect_to_qauld(&cli).await?;
    println!("qauld-ctl subscribed via: {addr}");

    let mut framed = LengthDelimitedCodec::builder()
        .length_field_offset(0)
        .length_field_type::<u16>()
        .length_adjustment(0)
        .new_framed(client);

    let request_id = Uuid::new_v4().to_string();
    send_subscribe_request(&mut framed, &request_id).await?;
    println!("waiting for events (Ctrl-C to stop)");

    loop {
        tokio::select! {
            biased;
            _ = tokio::signal::ctrl_c() => {
                println!("\nstopping subscription");
                return Ok(());
            }
            frame = framed.next() => {
                match frame {
                    Some(Ok(bytes)) => match proto::QaulRpc::decode(&bytes[..]) {
                        Ok(msg) => print_event(&msg.data),
                        Err(e) => log::warn!("subscribe: decode QaulRpc failed: {e}"),
                    },
                    Some(Err(e)) => {
                        log::error!("subscribe: read error: {e}");
                        return Ok(());
                    }
                    None => {
                        println!("daemon closed the connection");
                        return Ok(());
                    }
                }
            }
        }
    }
}

/// Open a subscription connection and push every formatted event line
/// into the returned channel. Returns the receiver immediately (the
/// connection is owned by a spawned task).
///
/// Used by shell mode to display events asynchronously alongside the
/// prompt. The task exits when the daemon closes the connection or when
/// the receiver is dropped (channel send fails).
pub(crate) async fn spawn_event_listener(
    cli: &Cli,
) -> Result<mpsc::UnboundedReceiver<String>, Box<dyn std::error::Error>> {
    let (client, _addr) = crate::connect_to_qauld(cli).await?;
    let mut framed = LengthDelimitedCodec::builder()
        .length_field_offset(0)
        .length_field_type::<u16>()
        .length_adjustment(0)
        .new_framed(client);

    let request_id = Uuid::new_v4().to_string();
    send_subscribe_request(&mut framed, &request_id).await?;

    let (tx, rx) = mpsc::unbounded_channel::<String>();
    tokio::spawn(async move {
        while let Some(frame) = framed.next().await {
            let bytes = match frame {
                Ok(b) => b,
                Err(e) => {
                    log::warn!("shell event listener: read error: {e}");
                    return;
                }
            };
            let qrpc = match proto::QaulRpc::decode(&bytes[..]) {
                Ok(m) => m,
                Err(e) => {
                    log::warn!("shell event listener: QaulRpc decode failed: {e}");
                    continue;
                }
            };
            if let Some(line) = format_event(&qrpc.data) {
                if tx.send(line).is_err() {
                    // Receiver dropped; tear down the listener.
                    return;
                }
            }
        }
        log::trace!("shell event listener: daemon closed connection");
    });
    Ok(rx)
}

async fn send_subscribe_request<T>(
    framed: &mut Framed<T, LengthDelimitedCodec>,
    request_id: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let envelope = proto_sub::Subscribe {
        message: Some(proto_sub::subscribe::Message::Request(
            proto_sub::SubscribeRequest { topics: Vec::new() },
        )),
    };
    let mut data = Vec::with_capacity(envelope.encoded_len());
    envelope.encode(&mut data)?;

    let rpc = proto::QaulRpc {
        module: proto::Modules::Subscribe as i32,
        request_id: request_id.to_string(),
        user_id: Vec::new(),
        data,
    };
    let mut buf = Vec::with_capacity(rpc.encoded_len());
    rpc.encode(&mut buf)?;
    framed.send(buf.into()).await?;
    Ok(())
}

/// Decode and pretty-print a single Subscribe envelope received from libqaul.
fn print_event(data: &[u8]) {
    if let Some(line) = format_event(data) {
        println!("{line}");
    }
}

/// Decode a Subscribe envelope and return a pre-formatted line for it,
/// or `None` if the envelope was malformed / not an event.
///
/// Used by the shell-mode async event display so it can drop-in the
/// same formatting that single-shot subscribe uses.
pub(crate) fn format_event(data: &[u8]) -> Option<String> {
    let envelope = match proto_sub::Subscribe::decode(data) {
        Ok(e) => e,
        Err(e) => {
            log::warn!("subscribe: decode Subscribe envelope failed: {e}");
            return None;
        }
    };
    let event = match envelope.message {
        Some(proto_sub::subscribe::Message::Event(ev)) => ev,
        Some(proto_sub::subscribe::Message::Request(_)) => {
            log::warn!("subscribe: received unexpected Request from daemon");
            return None;
        }
        None => return None,
    };

    Some(format_one_event(&event))
}

fn format_one_event(event: &proto_sub::Event) -> String {
    match event.topic.as_str() {
        "chat.message" => match proto_sub::ChatMessageEvent::decode(&event.payload[..]) {
            Ok(m) => format!(
                "[{ts}] chat.message  sender={sender} group={group}  {text}",
                ts = event.timestamp,
                sender = bs58::encode(&m.sender_id).into_string(),
                group = bs58::encode(&m.group_id).into_string(),
                text = m.content,
            ),
            Err(e) => format!(
                "[{ts}] chat.message  <decode failed: {e}, {n} payload bytes>",
                ts = event.timestamp,
                n = event.payload.len(),
            ),
        },
        "dtn.delivery_response" => match proto_sub::DtnDeliveryResponseEvent::decode(&event.payload[..]) {
            Ok(d) => format!(
                "[{ts}] dtn.delivery_response  signature={sig} storage={node}  status={status}{reason}",
                ts = event.timestamp,
                sig = bs58::encode(&d.signature).into_string(),
                node = bs58::encode(&d.storage_node).into_string(),
                status = match d.response_type {
                    1 => "accepted",
                    2 => "rejected",
                    _ => "unknown",
                },
                reason = if d.reason != 0 { format!(" reason={}", d.reason) } else { String::new() },
            ),
            Err(e) => format!(
                "[{ts}] dtn.delivery_response  <decode failed: {e}, {n} payload bytes>",
                ts = event.timestamp,
                n = event.payload.len(),
            ),
        },
        "peers.connected" => match proto_sub::PeerEvent::decode(&event.payload[..]) {
            Ok(p) => format!(
                "[{ts}] peers.connected  peer={peer} via={module}",
                ts = event.timestamp,
                peer = bs58::encode(&p.peer_id).into_string(),
                module = module_name(p.module),
            ),
            Err(e) => format!(
                "[{ts}] peers.connected  <decode failed: {e}, {n} payload bytes>",
                ts = event.timestamp,
                n = event.payload.len(),
            ),
        },
        other => format!(
            "[{ts}] {topic}  <{n} payload bytes>",
            ts = event.timestamp,
            topic = other,
            n = event.payload.len(),
        ),
    }
}

/// Translate the `ConnectionModule` integer used in `PeerEvent.module`
/// into a human-readable name. Mirrors `qaul.connections.ConnectionModule`.
fn module_name(module: u32) -> &'static str {
    match module {
        0 => "local",
        1 => "lan",
        2 => "internet",
        3 => "ble",
        4 => "none",
        _ => "?",
    }
}
