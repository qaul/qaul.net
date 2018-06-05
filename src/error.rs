//! Error handling for routers

/// Errors related to the routing core
pub enum RouteError {
    /// Indicates that a feature isn't implemented for a certain type
    FeatureNotSupported,
    ///
    #[doc(hidden)]
    __NonExhaustive,
}
