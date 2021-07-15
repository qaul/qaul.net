/**
 * Protocol of the QaulRouterBehaviour
 */

use libp2p::{
    core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo},
    swarm::NegotiatedSubstream,
};
use void::Void;
use wasm_timer::Instant;
use futures::prelude::*;
use rand::{distributions, prelude::*};
use std::{io, iter, time::Duration};

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
        iter::once(b"/qaul/router/1.0.0")
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
    let payload: [u8; BEHAVIOUR_SIZE] = thread_rng().sample(distributions::Standard);
    log::debug!("Preparing ping payload {:?}", payload);
    stream.write_all(&payload).await?;
    stream.flush().await?;
    let duration = Duration::new(0,0);
    Ok((stream, duration))
}

/**
 * Receive routing table information
 */
pub async fn recv_routing_table<S>(mut stream: S) -> io::Result<S>
where
    S: AsyncRead + AsyncWrite + Unpin
{
    let mut payload = [0u8; BEHAVIOUR_SIZE];
    log::debug!("Waiting for ping ...");
    stream.read_exact(&mut payload).await?;
    log::debug!("Sending pong for {:?}", payload);
    stream.write_all(&payload).await?;
    stream.flush().await?;
    Ok(stream)
}

