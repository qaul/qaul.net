use libp2p::{
    PeerId,
    core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo, upgrade},
};
use futures::{
    Future,
    io::{AsyncRead, AsyncWrite},
    AsyncWriteExt,
};
use std::{error, fmt, io, iter, pin::Pin
};

use crate::types::{
    QaulInfoData,
    QaulInfoReceived,
    QaulInfoSend,
};

/// Implementation of `ConnectionUpgrade` for the qaul_info protocol.
#[derive(Debug, Clone, Default)]
pub struct QaulInfoProtocol {}

impl QaulInfoProtocol {
    /// Builds a new `QaulInfoProtocol`.
    pub fn new() -> QaulInfoProtocol {
        QaulInfoProtocol {}
    }
}

impl UpgradeInfo for QaulInfoProtocol {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/qaul_info/1.0.0")
    }
}

impl<TSocket> InboundUpgrade<TSocket> for QaulInfoProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = QaulInfoData;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, mut substream: TSocket, _info: Self::Info) -> Self::Future {
        Box::pin(async move {
            // receive new QaulInfo binary message
            let data = upgrade::read_length_prefixed( &mut substream, 4096 ).await?;

            // hand it directly to the internal message bus
            Ok(QaulInfoData { data })
        })
    }
}


impl UpgradeInfo for QaulInfoData {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/qaul_info/1.0.0")
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for QaulInfoData
where
    TSocket: AsyncWrite + AsyncRead + Send + Unpin + 'static,
{
    type Output = ();
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_outbound(self, mut substream: TSocket, _: Self::Info) -> Self::Future {
        Box::pin(async move {
            upgrade::write_length_prefixed(&mut substream, self.data).await?;
            substream.close().await?;

            Ok(())
        })
    }
}