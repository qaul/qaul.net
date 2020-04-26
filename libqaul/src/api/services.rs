//! Service interconnect interface

use crate::{
    error::Result,
    helpers::TagSet,
    services::{MetadataMap, ServiceEvent},
    users::UserAuth,
    Qaul,
};

/// Manage service sessions and related metadata
///
/// Services are external applications using libqaul as a basis to
/// communicate on a distributed network.  For a service to start
/// using all libqaul functions it should register itself via this
/// API.  This will unlock internal storage and better
/// subscription support.
///
/// Some applications might be quite security critical, and so,
/// there needs to be a way to store data in a safe way for future
/// sessions, without offering metadata sidechannels from captured
/// devices.  This API is a solution to this problem.
///
/// In libqaul, all data is stored to disk encrypted, meaning that
/// conversations, keys and logs are safe from inspection.  To
/// allow services to hook into the same storage mechanism for
/// their own metadata, this API provides a view into a per-user,
/// per-service metadata map.  This way your service doesn't have
/// to re-implemented secure disk storage, or rely on easier
/// non-secure storage.
pub struct Services<'chain> {
    pub(crate) q: &'chain Qaul,
}

impl<'qaul> Services<'qaul> {
    /// Check if "god mode" is supported by this instance
    pub fn god_mode(&self) -> bool {
        true // TODO: make configurable
    }

    /// Add an external service to the qaul service registry
    ///
    /// Registering a service means that future `Message` listeners
    /// can be allocated for this service, as well as enabling polling.
    ///
    /// Names of services need to be unique, so it's advised to
    /// namespace them on some other key, for example the application
    /// package name (such as `com.example.myapp`)
    pub async fn register<S: Into<String>, F: 'static>(&self, name: S, cb: F) -> Result<()>
    where
        F: Fn(ServiceEvent) + Send + Sync,
    {
        self.q.services.register(name.into(), cb).await
    }

    /// Remove an external service from the qaul service registry
    ///
    /// Calling this function will disable the ability to poll for
    /// messages, as well as deleting all already registered message
    /// listeners already existing for this service.
    ///
    /// Will return `Error::NoService` if no such service name could
    /// be found.
    pub async fn unregister<S: Into<String>>(&self, name: S) -> Result<()> {
        self.q.services.unregister(name.into()).await
    }

    /// Save some piece of metadata, for a particular user and service
    ///
    /// This function can be used to save a piece of metadata with a
    /// set of tags.  The name of a MetadataMap needs to be unique and
    /// will be overridden by this call.  The search tags can be used
    /// to identity different classes of data, but can also be left
    /// empty.
    pub async fn save<S, T>(
        &self,
        user: UserAuth,
        service: S,
        data: MetadataMap,
        tags: T,
    ) -> Result<()>
    where
        S: Into<String>,
        T: Into<TagSet>,
    {
        let serv = service.into();
        let (id, _) = self.q.auth.trusted(user)?;
        self.q.services.check(&serv).await?;
        self.q
            .services
            .store()
            .save(id, serv, data, tags.into())
            .await
    }

    /// Delete a particular key from the service metadata store
    ///
    /// Will only error on access failure, not if the key didn't
    /// previously exist.
    pub async fn delete<S, K>(&self, user: UserAuth, service: S, key: K) -> Result<()>
    where
        S: Into<String>,
        K: Into<String>,
    {
        let serv = service.into();
        let (id, _) = self.q.auth.trusted(user)?;
        self.q.services.check(&serv).await?;
        self.q.services.store().delete(id, serv, key.into()).await;
        Ok(())
    }

    /// Make a query into the service metadata store via a set of tags
    ///
    /// Each entry in the store can further be associated with a name.
    /// If your query doesn't provide a tag filter all entries for the
    /// service/user combination will be returned.
    pub async fn query<S, T>(&self, user: UserAuth, service: S, tags: T) -> Result<Vec<MetadataMap>>
    where
        S: Into<String>,
        T: Into<TagSet>,
    {
        let serv = service.into();
        let (id, _) = self.q.auth.trusted(user)?;
        self.q.services.check(&serv).await?;
        Ok(self.q.services.store().query(id, serv, tags.into()).await)
    }
}
