/**
 * Protocol of the QaulRouterBehaviour
 */

use libp2p::{
    PeerId,
    core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo},
    swarm::NegotiatedSubstream,
};
use void::Void;
use futures::prelude::*;
use std::{io, iter, time::Duration};

use crate::router::info::RouterInfo;

/**
 * # Qaul Router Behaviour Protocol
 * 
 * This behaviour does the following things:
 * 
 * * Sends information of the nodes routing table 
 *   to it's peers.
 * * Asks and sends information of users.
 */
#[derive(Default, Debug, Copy, Clone)]
pub struct QaulRouterBehaviour;

const BEHAVIOUR_SIZE: usize = 32;

impl UpgradeInfo for QaulRouterBehaviour {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/qaul/router/0.1.0")
    }
}

impl InboundUpgrade<NegotiatedSubstream> for QaulRouterBehaviour {
    type Output = NegotiatedSubstream;
    type Error = Void;
    type Future = future::Ready<Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, stream: NegotiatedSubstream, _: Self::Info) -> Self::Future {
        future::ok(stream)
    }
}

impl OutboundUpgrade<NegotiatedSubstream> for QaulRouterBehaviour {
    type Output = NegotiatedSubstream;
    type Error = Void;
    type Future = future::Ready<Result<Self::Output, Self::Error>>;

    fn upgrade_outbound(self, stream: NegotiatedSubstream, _: Self::Info) -> Self::Future {
        future::ok(stream)
    }
}

/**
 * Send routing table information
 */
pub async fn send_routing_table<S>(mut stream: S) -> io::Result<(S, Duration)>
where
    S: AsyncRead + AsyncWrite + Unpin
{
    // get routing table info from router
    let data = RouterInfo::create(None);
    log::debug!("Preparing routing table");    

    stream.write_all(&data).await?;
    stream.flush().await?;
    let duration = Duration::new(0,0);
    Ok((stream, duration))
}

/**
 * Receive routing table information
 */
pub async fn receive_routing_info<S>(mut stream: S) -> io::Result<S>
where
    S: AsyncRead + AsyncWrite + Unpin
{
    let mut data: Vec<u8> = Vec::new();
    log::info!("Receiving router info message");
    let read_result = stream.read_to_end(&mut data).await?;
    // TODO: how many bytes?
    log::info!("Router info message received: {:?}", read_result);
    Ok(stream)
}

