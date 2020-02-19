//! API wrapper structures

pub mod contacts;
pub mod envelope;
pub mod files;
pub mod messages;
pub mod users;

#[feature(chat)]
pub mod chat;

use async_trait::async_trait;
use libqaul::Qaul;

/// Apply an RPC structure to a libqaul instance
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

/// A trait for objects that can modify an instance of libqaul
#[async_trait]
pub trait QaulRpc {
    type Response;
    async fn apply(self, qaul: &Qaul) -> Self::Response;
}
