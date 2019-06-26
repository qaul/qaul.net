//! Device access API

/// Change this?
pub type ServiceID = String;

/// Represents an authenticated service
pub struct Service {
    /// 
    pub name: String
}

pub fn register(s: Service) {}

pub fn revoke(id: ServiceID) {}
