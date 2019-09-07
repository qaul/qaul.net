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
//! key-value data (usually encoded in json), as well as binary large
//! objects (that 24GB copy of Hackers we all have, but don't
//! entirelty understand the origins of) while also providing an easy
//! hook-based interface to encrypt and decrypt data on write and
//! read.
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
//! Every zone has a namespace (i.e. `lib:spacekookie`), followed by a
//! scope (i.e. `lib:spacekookie/secure` would refer to an encrypted
//! scope). Permissions are set per scope, so it's possible to define
//! a zone for sharing (`lib:spacekookie/share`) and it's also
//! entirely possible to not bind a scope to a namespace (such as a
//! global `lib:/share` scope).
//!
//! ## Persistence
//!
//! So far all of these concepts exist in memory. But `alexandria` is
//! a _persistence_ module, meaning it stores things for ever (if WD
//! drive failure statistics are to be believed...).

pub mod data;
pub mod keys;
pub mod namespace;
pub mod scope;
pub mod store;
pub mod user;

use data::{Data, Value};
use namespace::{Address, Namespace};
use scope::ScopeAttr;
use std::collections::BTreeMap;
use std::{fs::create_dir_all, path::Path};
use store::Storable;

/// Primary access point to the great library
#[derive(Default, Debug)]
pub struct Alexandria {
    /// A map from Namespace-name -> Namespace. If the key is `None`,
    /// the namespace is "root"
    data: BTreeMap<Option<String>, Namespace>,
    /// A map of keys, from user-id to key value.
    keys: BTreeMap<String, String>,
}

impl Alexandria {
    /// Create a new library in memory without any paths
    ///
    /// **Not implemented yet**
    ///
    /// While this operation is practically free in theory, please
    /// keep in mind that async workers _will_ be started with this
    /// call, to syndicate in-memory state with the persistence layer.
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
    pub fn create_path(&mut self, path: String, attrs: ScopeAttr) {
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

    /// Insert some data into an address position
    ///
    /// Previous data will be overwritten and should be checked
    /// manually for existence first. The `Address` is a unique
    /// identifiable path in a Library, pointing to an optional
    /// namespace, a scope and ultimately a data id.
    pub fn insert(&mut self, addr: Address, data: Data) {
        let (ns, scope, name) = match addr {
            Address::Ns(ns, scope, name) => (Some(ns.into()), scope, name),
            Address::Root(scope, name) => (None, scope, name),
        };

        self.data
            .get_mut(&ns)
            .map(|ns| ns.insert(scope, name, data))
            .expect("Failed to operate on non-existing Namespace");
    }

    /// Sync state with disk (**remove before `1.0.0`**!)
    ///
    /// This function exists as a work-around to avoid having to model
    /// internal workers and listeners and being able to debug
    /// `alexandria` as a stateless system for 5 minutes before it all
    /// goes to shit.
    #[deprecated]
    pub fn sync(&mut self) {
        self.data.iter().for_each(|(_, ns)| {
            ns.scopes().for_each(|(_, scope)| {
                // First make sure the path offset exists
                let offset = &scope.attrs.offset;
                let offset_path = Path::new(offset);
                if !offset_path.exists() {
                    create_dir_all(&offset_path).unwrap();
                }

                // Then write all scope entries
                scope.all().for_each(|(id, data)| {
                    data.write(offset.as_str(), id.as_str());
                });
            });
        });
    }
}
