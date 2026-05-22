// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Unix-socket transport (TCP on Windows). Today's default qauld-ctl behaviour.

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

use crate::cli::Cli;
use crate::proto;
use crate::transport::{encode_envelope, RpcTransport};

#[cfg(unix)]
type Stream = UnixStream;
#[cfg(windows)]
type Stream = TcpStream;

#[cfg(windows)]
const DEFAULT_TCP_ADDR: &str = "127.0.0.1:9199";

pub struct SocketTransport {
    framed: Framed<Stream, LengthDelimitedCodec>,
    pub addr: String,
}

impl SocketTransport {
    pub async fn connect(cli: &Cli) -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, addr) = open_stream(cli).await?;
        let framed = LengthDelimitedCodec::builder()
            .length_field_offset(0)
            .length_field_type::<u16>()
            .length_adjustment(0)
            .new_framed(stream);
        Ok(Self { framed, addr })
    }
}

#[cfg(unix)]
async fn open_stream(cli: &Cli) -> Result<(Stream, String), Box<dyn std::error::Error>> {
    let path = if let Some(socket) = &cli.socket {
        socket.clone()
    } else if let Some(dir) = &cli.dir {
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
async fn open_stream(cli: &Cli) -> Result<(Stream, String), Box<dyn std::error::Error>> {
    let addr = cli.socket.clone().unwrap_or_else(|| DEFAULT_TCP_ADDR.to_string());
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

