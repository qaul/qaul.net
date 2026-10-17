// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Receive-side entry point for router_v2 and it
//! includes the wire dispatch (spec §8.8 steps 1-3)

mod manifest_apply;
mod manifest_request;
mod manifest_serve;
mod routing_update;
mod translate;

use libp2p::PeerId;
use tracing::{error, info, warn};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{
            messages::{IndexDump, ManifestDelta, ManifestRequest, NodeManifest, RoutingUpdate},
            CodecError, Header, RoutingMessage,
        },
        OutboundKind, OutboundMsg, Result, RouterV2State,
    },
};

/// What a handler needs to know about the message being processed
pub(crate) struct ReceiveCtx {
    pub neighbour: PeerId,
    pub transport: ConnectionModule,
    pub rssi_dbm: Option<i8>,
    pub now: u64,
}

impl RouterV2State {
    pub fn received(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        rssi_dbm: Option<i8>,
        mut buf: &[u8],
        now: u64,
    ) -> Result<()> {
        // TEMP(): first thing to check when nothing arrives — this
        // separates "never sent" from "sent but not dispatched here".
        info!(
            "router_v2 RECV ← peer={neighbour} transport={transport:?} type={:#04x} bytes={}",
            buf.get(1).copied().unwrap_or(0),
            buf.len(),
        );

        while !buf.is_empty() {
            let (header, body_slice) = match Header::decode(buf) {
                Ok(h) => h,
                Err(CodecError::BadVersion { payload_len, .. }) => {
                    let skip = 4 + payload_len;
                    if buf.len() < skip as usize {
                        break;
                    }
                    buf = &buf[skip as usize..];
                    continue;
                }
                Err(e) => {
                    warn!("failed to decode header: {e}");
                    return Ok(());
                }
            };

            let payload_len = header.payload_len as usize;
            if body_slice.len() < payload_len {
                warn!(
                    "received: truncated body, expected {payload_len} got {}",
                    body_slice.len()
                );
                return Ok(());
            }
            let payload = &body_slice[..payload_len];
            buf = &body_slice[payload_len..];

            match header.message_type {
                RoutingMessage::RoutingUpdate => match RoutingUpdate::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) =
                            self.handle_routing_update(neighbour, transport, rssi_dbm, msg, now)
                        {
                            error!("handle_routing_update failed: {e}");
                        }
                    }
                    Err(e) => error!("RoutingUpdate decode failed: {e}"),
                },
                RoutingMessage::NodeManifest => match NodeManifest::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) = self.handle_node_manifest(msg, now, transport) {
                            error!("handle_node_manifest failed: {e}");
                        }
                    }
                    Err(e) => error!("NodeManifest decode failed: {e}"),
                },
                RoutingMessage::IndexDump => match IndexDump::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) = self.handle_index_dump(neighbour, msg, now) {
                            error!("handle_index_dump failed: {e}");
                        }
                    }
                    Err(e) => error!("IndexDump decode failed: {e}"),
                },
                RoutingMessage::ManifestDelta => match ManifestDelta::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) = self.handle_manifest_delta(neighbour, msg, now, transport) {
                            error!("handle_manifest_delta failed: {e}");
                        }
                    }
                    Err(e) => error!("ManifestDelta decode failed: {e}"),
                },
                RoutingMessage::ManifestRequest => match ManifestRequest::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) = self.handle_manifest_request(neighbour, transport, msg, now)
                        {
                            error!("handle_manifest_request failed: {e}");
                        }
                    }
                    Err(e) => error!("ManifestRequest decode failed: {e}"),
                },
            }
        }
        Ok(())
    }

    pub(crate) fn send_framed(
        &self,
        peer: PeerId,
        transport: ConnectionModule,
        message_type: RoutingMessage,
        body: Vec<u8>,
    ) {
        let payload_len = match u16::try_from(body.len()) {
            Ok(n) => n,
            Err(_) => {
                error!("send_framed: body of {} bytes exceeds u16", body.len());
                return;
            }
        };
        let header = Header {
            version: crate::router_v2::codec::PROTOCOL_VERSION,
            message_type,
            payload_len,
        };
        let mut frame = Vec::with_capacity(4 + body.len());
        header.encode(&mut frame);
        frame.extend(body);

        if let Err(e) = self.tx_outbound.send(OutboundMsg {
            kind: OutboundKind::Routing,
            peer,
            transport,
            bytes: frame,
        }) {
            warn!("send_framed: outbound channel closed for {peer:?}: {e}");
        }
    }
}
