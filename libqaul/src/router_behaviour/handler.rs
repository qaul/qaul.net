/**
 * Handlers for the QaulRouterBehaviour
 */

use libp2p::{
    PeerId,
    identity::PublicKey,
    swarm::{
        KeepAlive,
        NegotiatedSubstream,
        SubstreamProtocol,
        ProtocolsHandler,
        ProtocolsHandlerUpgrErr,
        ProtocolsHandlerEvent,
    },
};
use std::{
    error::Error,
    io,
    fmt,
    num::NonZeroU32,
    task::{Context, Poll},
    time::Duration
};
use wasm_timer::Delay;
use void::Void;
use futures::prelude::*;
use futures::future::BoxFuture;
use std::collections::VecDeque;

use crate::router_behaviour::protocol;


type QaulRouterTableFuture = BoxFuture<'static, Result<(NegotiatedSubstream, Duration), io::Error>>;
type QaulRouterResponseFuture = BoxFuture<'static, Result<NegotiatedSubstream, io::Error>>;

/// The current state of outbound messages.
enum QaulRouterBehaviourState {
    /// A new substream is being negotiated for the QaulRouterBehaviour protocol.
    OpenStream,
    /// The substream is idle, waiting to send the next RoutingInfo.
    Idle(NegotiatedSubstream),
    /// A QaulRouterBehaviour is being sent and the response awaited.
    RoutingTable(QaulRouterTableFuture),
}


/// The configuration for outbound communication.
#[derive(Clone, Debug)]
pub struct QaulRouterBehaviourConfig {
    /// The timeout of an information request
    timeout: Duration,
    /// The interval for sending routing information
    interval: Duration,
    /// The maximum number of allowed failed connections
    max_failures: NonZeroU32,
    /// Whether the connection should generally be kept alive unless
    /// `max_failures` occur.
    keep_alive: bool,
}

impl QaulRouterBehaviourConfig {
    /// Creates a new `QaulRouterBehaviourConfig` with the following default settings:
    ///
    ///   * [`QaulRouterBehaviourConfig::interval`] 15s
    ///   * [`QaulRouterBehaviourConfig::timeout`] 20s
    ///   * [`QaulRouterBehaviourConfig::max_failures`] 1
    ///   * [`QaulRouterBehaviourConfig::keep_alive`] true

    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(20),
            interval: Duration::from_secs(15),
            max_failures: NonZeroU32::new(1).expect("1 != 0"),
            keep_alive: true,
        }
    }

    /// Sets the QaulRouterBehaviour timeout.
    pub fn with_timeout(mut self, d: Duration) -> Self {
        self.timeout = d;
        self
    }

    /// Sets the QaulRouterBehaviour interval.
    pub fn with_interval(mut self, d: Duration) -> Self {
        self.interval = d;
        self
    }

    /// Sets the maximum number of consecutive QaulRouterBehaviour failures upon which the remote
    /// peer is considered unreachable and the connection closed.
    pub fn with_max_failures(mut self, n: NonZeroU32) -> Self {
        self.max_failures = n;
        self
    }

    /// Sets whether the QaulRouterBehaviour protocol itself should keep the connection alive,
    /// apart from the maximum allowed failures.
    ///
    /// default = true
    ///
    /// If the maximum number of allowed QaulRouterBehaviour failures is reached, the
    /// connection is always terminated as a result of [`ProtocolsHandler::poll`]
    /// returning an error, regardless of the keep-alive setting.
    pub fn with_keep_alive(mut self, b: bool) -> Self {
        self.keep_alive = b;
        self
    }
}

/// The result of an inbound or outbound QaulRouterBehaviour.
pub type QaulRouterBehaviourResult = Result<QaulRouterBehaviourSuccess, QaulRouterBehaviourFailure>;

#[derive(Debug)]
pub struct User {
    id: PeerId,
    key: PublicKey,
    name: String,
}

#[derive(Debug)]
pub struct UserInfo {
    table: Vec<User>,
}

#[derive(Debug)]
pub struct RoutingTableEntry {
    id: PeerId,
    rtt: f64,
    hc: u8,
}

#[derive(Debug)]
pub struct RoutingTableInfo {
    table: Vec<RoutingTableEntry>,
}

#[derive(Debug)]
pub struct UserRequest {
    table: Vec<PeerId>,
}

/// The successful result of processing an inbound or outbound QaulRouterBehaviour.
#[derive(Debug)]
pub enum QaulRouterBehaviourSuccess {
    /// Received a scheduled RoutingInfo, which consists the entire RoutingTable, and UserInfo.
    RoutingInfo { routing: RoutingTableInfo, users: UserInfo },
    /// Received a User Information request.
    InfoRequest { users: UserRequest },
    /// Received UserInfo
    InfoResponse { users: UserInfo },
}

/// An outbound QaulRouterBehaviour failure.
#[derive(Debug)]
pub enum QaulRouterBehaviourFailure {
    /// The QaulRouterBehaviour timed out, i.e. no response was received within the
    /// configured QaulRouterBehaviour timeout.
    Timeout,
    /// The QaulRouterBehaviour failed for reasons other than a timeout.
    Other { error: Box<dyn std::error::Error + Send + 'static> }
}

impl fmt::Display for QaulRouterBehaviourFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QaulRouterBehaviourFailure::Timeout => f.write_str("QaulRouterBehaviour timeout"),
            QaulRouterBehaviourFailure::Other { error } => write!(f, "QaulRouterBehaviour error: {}", error)
        }
    }
}

impl Error for QaulRouterBehaviourFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            QaulRouterBehaviourFailure::Timeout => None,
            QaulRouterBehaviourFailure::Other { error } => Some(&**error)
        }
    }
}

/// Protocol handler that handles sending QaulRouterBehaviour information to the remote at a regular period
/// and answering user information requests.
///
/// If the remote doesn't respond, produces an error that closes the connection.
pub struct QaulRouterBehaviourHandler {
    /// Configuration options.
    config: QaulRouterBehaviourConfig,
    /// The timer used for the delay to the next RoutingInfo as well as
    /// the connection timeout.
    timer: Delay,
    /// Outbound RoutingInfo failures that are pending to be processed by `poll()`.
    pending_errors: VecDeque<QaulRouterBehaviourFailure>,
    /// The number of consecutive send failures that occurred.
    /// Every successful connection resets this counter to 0.
    failures: u32,
    /// The outbound QaulRouterBehaviour state.
    outbound: Option<QaulRouterBehaviourState>,
    /// The inbound user information request handler, i.e. if there is an inbound
    /// substream, this is always a future that waits for the
    /// next inbound user information request to be answered.
    inbound: Option<QaulRouterResponseFuture>,
}

impl QaulRouterBehaviourHandler {
    /// Builds a new `QaulRouterBehaviourHandler` with the given configuration.
    pub fn new(config: QaulRouterBehaviourConfig) -> Self {
        QaulRouterBehaviourHandler {
            config,
            timer: Delay::new(Duration::new(0, 0)),
            pending_errors: VecDeque::with_capacity(2),
            failures: 0,
            outbound: None,
            inbound: None,
        }
    }
}

impl ProtocolsHandler for QaulRouterBehaviourHandler {
    type InEvent = Void;
    type OutEvent = QaulRouterBehaviourResult;
    type Error = QaulRouterBehaviourFailure;
    type InboundProtocol = protocol::QaulRouterBehaviour;
    type OutboundProtocol = protocol::QaulRouterBehaviour;
    type OutboundOpenInfo = ();
    type InboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<protocol::QaulRouterBehaviour, ()> {
        SubstreamProtocol::new(protocol::QaulRouterBehaviour, ())
    }

    fn inject_fully_negotiated_inbound(&mut self, stream: NegotiatedSubstream, (): ()) {
        self.inbound = Some(protocol::recv_routing_table(stream).boxed());
    }

    fn inject_fully_negotiated_outbound(&mut self, stream: NegotiatedSubstream, (): ()) {
        self.timer.reset(self.config.timeout);
        self.outbound = Some(QaulRouterBehaviourState::RoutingTable(protocol::send_routing_table(stream).boxed()));
    }

    fn inject_event(&mut self, _: Void) {}

    fn inject_dial_upgrade_error(&mut self, _info: (), error: ProtocolsHandlerUpgrErr<Void>) {
        self.outbound = None; // Request a new substream on the next `poll`.
        self.pending_errors.push_front(
            match error {
                // Note: This timeout only covers protocol negotiation.
                ProtocolsHandlerUpgrErr::Timeout => QaulRouterBehaviourFailure::Timeout,
                e => QaulRouterBehaviourFailure::Other { error: Box::new(e) },
            })
    }

    fn connection_keep_alive(&self) -> KeepAlive {
        if self.config.keep_alive {
            KeepAlive::Yes
        } else {
            KeepAlive::No
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<
        ProtocolsHandlerEvent<
            protocol::QaulRouterBehaviour, 
            (), 
            QaulRouterBehaviourResult, 
            Self::Error
        >
    > {
        // Process inbound QaulRouterBehaviour.
        if let Some(fut) = self.inbound.as_mut() {
            match fut.poll_unpin(cx) {
                Poll::Pending => {},
                Poll::Ready(Err(e)) => {
                    log::debug!("Inbound QaulRouterBehaviour error: {:?}", e);
                    self.inbound = None;
                }
                Poll::Ready(Ok(stream)) => {
                    // A QaulRouterBehaviour from a remote peer has been processed, wait for the next.
                    self.inbound = Some(protocol::recv_routing_table(stream).boxed());
                    
                    //return Poll::Ready(ProtocolsHandlerEvent::Custom(Ok(QaulRouterBehaviourSuccess::InfoRequest { users: UserRequest { table: } })))
                }
            }
        }

        loop {
            // Check for outbound messaging failures.
            if let Some(error) = self.pending_errors.pop_back() {
                log::debug!("QaulRouterBehaviour failure: {:?}", error);

                self.failures += 1;

                // Note: For backward-compatibility, with configured
                // `max_failures == 1`, the first failure is always "free"
                // and silent. This allows peers who still use a new substream
                // for each message to have successful message exchanges with peers
                // that use a single substream, since every successful message
                // resets `failures` to `0`, while at the same time emitting
                // events only for `max_failures - 1` failures, as before.
                if self.failures > 1 || self.config.max_failures.get() > 1 {
                    if self.failures >= self.config.max_failures.get() {
                        log::debug!("Too many failures ({}). Closing connection.", self.failures);
                        return Poll::Ready(ProtocolsHandlerEvent::Close(error))
                    }

                    return Poll::Ready(ProtocolsHandlerEvent::Custom(Err(error)))
                }
            }

            // Continue outbound messages.
            match self.outbound.take() {
                Some(QaulRouterBehaviourState::RoutingTable(mut _qaulRoutingBehaviour)) => {},
                // -------------------
                // my intervention:
                // -------------------
                // Some(QaulRouterBehaviourState::RoutingTable(mut qaulRoutingBehaviour)) => match qaulRoutingBehaviour.poll_unpin(cx) {
                //     Poll::Pending => Poll::Pending,
                //     //Poll::Ready(Ok((stream, rtt))) => {
                //     // Poll::Ready(Ok(_)) => {
                //     //     // self.failures = 0;
                //     //     // self.timer.reset(self.config.interval);
                //     //     // self.outbound = Some(QaulRouterBehaviourState::Idle(stream));
                //     //     // return Poll::Ready(
                //     //     //     ProtocolsHandlerEvent::Custom(
                //     //     //         Ok(QaulRouterBehaviourSuccess::InfoRequest { users: UserRequest {} })
                //     //     //     )
                //     //     // )
                //     // },
                //     Poll::Ready(Err(e)) => {
                //         self.pending_errors.push_front(QaulRouterBehaviourFailure::Other {
                //             error: Box::new(e)
                //         });
                //     },
                //     Poll::Ready(_) => {}
                // },
                // -------------------
                // original Ping version
                // -------------------
                // Some(PingState::Ping(mut ping)) => match ping.poll_unpin(cx) {
                //     Poll::Pending => {
                //         if self.timer.poll_unpin(cx).is_ready() {
                //             self.pending_errors.push_front(PingFailure::Timeout);
                //         } else {
                //             self.outbound = Some(PingState::Ping(ping));
                //             break
                //         }
                //     },
                //     Poll::Ready(Ok((stream, rtt))) => {
                //         self.failures = 0;
                //         self.timer.reset(self.config.interval);
                //         self.outbound = Some(PingState::Idle(stream));
                //         return Poll::Ready(
                //             ProtocolsHandlerEvent::Custom(
                //                 Ok(PingSuccess::Ping { rtt })))
                //     }
                //     Poll::Ready(Err(e)) => {
                //         self.pending_errors.push_front(PingFailure::Other {
                //             error: Box::new(e)
                //         });
                //     }
                // },
                Some(QaulRouterBehaviourState::Idle(stream)) => match self.timer.poll_unpin(cx) {
                    Poll::Pending => {
                        self.outbound = Some(QaulRouterBehaviourState::Idle(stream));
                        break
                    },
                    Poll::Ready(Ok(())) => {
                        self.timer.reset(self.config.interval);
                        self.outbound = Some(QaulRouterBehaviourState::RoutingTable(protocol::send_routing_table(stream).boxed()));
                    },
                    Poll::Ready(Err(e)) => {
                        return Poll::Ready(ProtocolsHandlerEvent::Close(
                            QaulRouterBehaviourFailure::Other {
                                error: Box::new(e)
                            }))
                    }
                }
                Some(QaulRouterBehaviourState::OpenStream) => {
                    self.outbound = Some(QaulRouterBehaviourState::OpenStream);
                    break
                }
                None => {
                    self.outbound = Some(QaulRouterBehaviourState::OpenStream);
                    let protocol = SubstreamProtocol::new(protocol::QaulRouterBehaviour, ());
                    return Poll::Ready(ProtocolsHandlerEvent::OutboundSubstreamRequest {
                        protocol
                    })
                }
            }
        }

        Poll::Pending
    }
}
