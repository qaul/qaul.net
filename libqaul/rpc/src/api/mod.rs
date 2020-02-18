//! API wrapper structures

pub mod request;
pub mod response;
pub mod responder;

pub mod contacts;
pub mod users;
pub mod messages;
pub mod files;
#[feature(chat)]
pub mod chat;

use async_trait::async_trait;
use libqaul::Qaul;

/// An extension allowing `QaulRPC` object to be applied directly
/// to an instance of `Qaul`
#[async_trait]
pub trait QaulExt {
    async fn apply<R, T>(&self, r: T) -> R
    where
        R: Send + Sync,
        T: Send + Sync + QaulRPC<Response = R>;
}

#[async_trait]
impl QaulExt for Qaul {
    async fn apply<R, T>(&self, r: T) -> R
    where
        R: Send + Sync,
        T: Send + Sync + QaulRPC<Response = R>,
    {
        r.apply(self).await
    }
}

/// A trait for objects that can modify an instance of `Qaul`
#[async_trait]
pub trait QaulRPC {
    type Response;
    async fn apply(self, qaul: &Qaul) -> Self::Response;
}
