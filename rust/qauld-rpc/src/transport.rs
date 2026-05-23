// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Transport layer for qauld RPC: the same `RpcTransport` trait
//! implemented over a Unix socket (Windows: TCP) and, behind the
//! `embedded` feature, in-process against a linked `libqaul`.

use std::path::PathBuf;
use std::time::Duration;

use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[cfg(unix)]
use tokio::net::UnixStream;
#[cfg(windows)]
use tokio::net::TcpStream;

use crate::proto;

#[cfg(unix)]
type Stream = UnixStream;
#[cfg(windows)]
type Stream = TcpStream;

#[cfg(windows)]
const DEFAULT_TCP_ADDR: &str = "127.0.0.1:9199";

/// Where to find the qauld daemon. Mirrors the CLI's discovery
/// rules (explicit path > env > directory > cwd).
#[derive(Debug, Clone, Default)]
pub struct ConnectInfo {
    /// Explicit socket path (Unix) or `host:port` (Windows).
    pub socket: Option<String>,
    /// Directory containing `qauld.sock`.
    pub dir: Option<String>,
}

#[async_trait]
pub trait RpcTransport: Send {
    /// Send a fully-formed `QaulRpc` envelope and wait for the
    /// daemon's response carrying the same `request_id`. Returns
    /// `Ok(None)` when `expect_response` is false.
    async fn request(
        &mut self,
        envelope: proto::QaulRpc,
        timeout: Duration,
        expect_response: bool,
    ) -> Result<Option<proto::QaulRpc>, Box<dyn std::error::Error>>;
}

pub(crate) fn encode_envelope(envelope: &proto::QaulRpc) -> Vec<u8> {
    let mut buf = Vec::with_capacity(envelope.encoded_len());
    envelope
        .encode(&mut buf)
        .expect("Vec<u8> provides capacity as needed");
    buf
}

// -----------------------------------------------------------------
// SocketTransport
// -----------------------------------------------------------------

pub struct SocketTransport {
    framed: Framed<Stream, LengthDelimitedCodec>,
    pub addr: String,
}

impl SocketTransport {
    pub async fn connect(info: &ConnectInfo) -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, addr) = open_stream(info).await?;
        let framed = LengthDelimitedCodec::builder()
            .length_field_offset(0)
            .length_field_type::<u16>()
            .length_adjustment(0)
            .new_framed(stream);
        Ok(Self { framed, addr })
    }

    /// Expose the framed stream for callers that need raw access
    /// (subscribe mode, shell mode REPL).
    pub fn into_framed(self) -> Framed<Stream, LengthDelimitedCodec> {
        self.framed
    }
}

#[cfg(unix)]
async fn open_stream(info: &ConnectInfo) -> Result<(Stream, String), Box<dyn std::error::Error>> {
    let path = if let Some(socket) = &info.socket {
        socket.clone()
    } else if let Some(dir) = &info.dir {
        PathBuf::from(dir)
            .join("qauld.sock")
            .to_string_lossy()
            .into_owned()
    } else {
        "qauld.sock".to_string()
    };
    let stream = UnixStream::connect(&path).await?;
    Ok((stream, path))
}

#[cfg(windows)]
async fn open_stream(info: &ConnectInfo) -> Result<(Stream, String), Box<dyn std::error::Error>> {
    let addr = info
        .socket
        .clone()
        .unwrap_or_else(|| DEFAULT_TCP_ADDR.to_string());
    let stream = TcpStream::connect(&addr).await?;
    Ok((stream, addr))
}

#[async_trait]
impl RpcTransport for SocketTransport {
    async fn request(
        &mut self,
        envelope: proto::QaulRpc,
        timeout: Duration,
        expect_response: bool,
    ) -> Result<Option<proto::QaulRpc>, Box<dyn std::error::Error>> {
        let bytes = encode_envelope(&envelope);
        self.framed.send(bytes.into()).await?;

        if !expect_response {
            return Ok(None);
        }

        let next = tokio::time::timeout(timeout, self.framed.next())
            .await
            .map_err(|_| "timed out waiting for daemon response")?;
        match next {
            Some(Ok(frame)) => {
                let resp = proto::QaulRpc::decode(&frame[..])
                    .map_err(|e| format!("malformed RPC envelope: {e}"))?;
                Ok(Some(resp))
            }
            Some(Err(e)) => Err(format!("socket read error: {e}").into()),
            None => Err("daemon closed the connection without responding".into()),
        }
    }
}

// -----------------------------------------------------------------
// EmbeddedTransport (optional)
// -----------------------------------------------------------------

#[cfg(feature = "embedded")]
mod embedded {
    use super::*;
    use std::sync::Arc;

    pub struct EmbeddedTransport {
        instance: Arc<libqaul::Libqaul>,
    }

    impl EmbeddedTransport {
        pub async fn start(
            storage_path: String,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let instance = libqaul::api::start_instance_in_thread(storage_path, None);
            let deadline = std::time::Instant::now() + Duration::from_secs(30);
            while !instance.is_initialized() {
                if std::time::Instant::now() >= deadline {
                    return Err("embedded libqaul failed to initialise within 30s".into());
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            if libqaul::node::user_accounts::UserAccounts::len(&*instance.state) == 0 {
                let name = format!(
                    "Community Node {}",
                    libqaul::utilities::timestamp::Timestamp::get_timestamp()
                );
                libqaul::node::user_accounts::UserAccounts::create(
                    &*instance.state,
                    name,
                    None,
                );
            }
            Ok(Self { instance })
        }
    }

    #[async_trait]
    impl RpcTransport for EmbeddedTransport {
        async fn request(
            &mut self,
            envelope: proto::QaulRpc,
            timeout: Duration,
            expect_response: bool,
        ) -> Result<Option<proto::QaulRpc>, Box<dyn std::error::Error>> {
            let want_id = envelope.request_id.clone();
            let bytes = encode_envelope(&envelope);
            libqaul::rpc::Rpc::send_to_libqaul(&*self.instance.state, bytes);

            if !expect_response {
                return Ok(None);
            }

            let deadline = std::time::Instant::now() + timeout;
            loop {
                match libqaul::rpc::Rpc::receive_from_libqaul(&*self.instance.state) {
                    Ok(data) => match proto::QaulRpc::decode(&data[..]) {
                        Ok(resp) if resp.request_id == want_id => return Ok(Some(resp)),
                        Ok(_) => continue,
                        Err(e) => return Err(format!("malformed RPC envelope: {e}").into()),
                    },
                    Err(_) => {
                        if std::time::Instant::now() >= deadline {
                            return Err("timed out waiting for libqaul response".into());
                        }
                        tokio::time::sleep(Duration::from_millis(5)).await;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "embedded")]
pub use embedded::EmbeddedTransport;
