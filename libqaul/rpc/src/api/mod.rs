//! API wrapper structures

pub mod contacts;
pub mod files;
pub mod messages;
pub mod users;

#[feature(chat)]
pub mod chat;

mod envelope;
pub use envelope::{Envelope, EnvelopeType, Request, Response};

mod responder;
pub use responder::Responder;

use async_trait::async_trait;
use libqaul::Qaul;

/// Apply an RPC structure to a libqaul instance
///
/// This trait is used to attach a new function to the qaul.net state
/// holder, without having to rely on feature flags to libqaul.
#[async_trait]
pub trait QaulExt {
    async fn apply<Response, Request>(&self, r: Request) -> Response
    where
        Response: Send + Sync,
        Request: Send + Sync + QaulRpc<Response = Response>;
}

#[async_trait]
impl QaulExt for Qaul {
    async fn apply<R, T>(&self, r: T) -> R
    where
        R: Send + Sync,
        T: Send + Sync + QaulRpc<Response = R>,
    {
        r.apply(self).await
    }
}

/// The RPC trait that get's access to the libqaul state
#[async_trait]
pub trait QaulRpc {
    type Response;
    async fn apply(self, qaul: &Qaul) -> Self::Response;
}
