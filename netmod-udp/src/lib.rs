//! netmod-udp is a UDP overlay for RATMAN

use identity::Identity;
use std::collections::{BTreeMap, BTreeSet};

/// Represents an IP network tunneled via UDP
pub struct Endpoint {
    nat: BTreeMap<String, BTreeSet<Identity>>,
}
