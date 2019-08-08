//! # Alexandria persistence store
//!
//! A muli-payload (key-value store and binary large object),
//! zone-encryption (key registry and permissions),
//! persistence module (storing things on spinning rust
//! or zappy quantum tunnels).
//!
//! ## No but seriously
//!
//! `alexandria` provides multiple payload endpoints to store
//! key-value data (usually encoded in json), as well as
//! binary large objects (that 24GB copy of Hackers we all have)
//! while also providing an easy hook-based interface to
//! encrypt and decrypt data on write and read.
//!
//! This is fundamentally done by assigning user zones. A user
//! provides a public key which is then used to encrypt data
//! that is saved in their "secure" scope zone.
//!
//! `alexandria` also handles access management and sharing files
//! between users.
//!
//! ## Zones
//!
//! Every zone has a namespace (i.e. `lib:spacekookie`), followed
//! by a scope (i.e. `lib:spacekookie/secure` would refer to an
//! encrypted scope). Permissions are set per scope, so it's possible
//! to define a zone for sharing (`lib:spacekookie/share`) and
//! it's also entirely possible to not bind a scope to a namespace
//! (such as a global `lib:/share` scope).
//!
//! ## Persistence
//!
//! So far all of these concepts exist in memory. But `alexandria` is
//! a _persistence_ module, meaning it stores things for ever (if WD
//! drive failure statistics are to be believed...).
//!

pub mod data;
pub mod keys;
pub mod namespace;
pub mod scope;
pub mod user;

use namespace::Namespace;
use scope::ScopeAttrs;
use std::collections::BTreeMap;

/// Primary access point to the great library
#[derive(Default)]
pub struct Alexandria {
    data: BTreeMap<Option<String>, Namespace>,
    keys: BTreeMap<String, String>,
}

impl Alexandria {
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a pubkey to `alexandria`
    pub fn add_key(&mut self, id: String, pubkey: String) {
        self.keys.insert(id, pubkey);
    }

    /// Create a new path from an input string
    ///
    /// The scheme follows: `lib:<namespace?>/<scope>`,
    /// where `lib` is a hard-coded string representing a library.
    pub fn create_path(&mut self, path: String, attrs: ScopeAttrs) {
        if !path.starts_with("lib:") {
            panic!("Invalid path!");
        }

        let segs: Vec<&str> = path[4..].split('/').collect();
        let ns = match segs[0] {
            "" => None,
            ns => Some(ns.into()),
        };
        let scope = segs[1].into();

        self.data.insert(ns.clone(), Namespace::default());
        self.data
            .get_mut(&ns)
            .map(|ns| ns.create_scope(scope, attrs));
    }
}
