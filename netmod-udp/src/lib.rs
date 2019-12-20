//! netmod-udp is a UDP overlay for RATMAN

use identity::Identity;
use std::{collections::BTreeMap, net::IpAddr};

/// Represents an IP network tunneled via UDP
pub struct Endpoint {
    nat: BTreeMap<Identity, IpAddr>,
}

impl Endpoint {
    /// Create a new UDP endpoint handler
    pub fn new() -> Self {
        Self {
            nat: Default::default(),
        }
    }
}
