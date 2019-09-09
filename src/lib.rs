//! # Alexandria data store library
//!
//! A multi-payload, zone-encrypting, journaled persistence module,
//! built with low-overhead applications in mind.
//!
//! - Stores data in namespaces and scopes
//! - Key-value stores and lazy blobs
//! - Supports per-scope asymetric encryption key
//! - Uses transaction Deltas for journal and concurrency safety
//! - Integrates into OS persistence layers (storing things on spinning
//!   rust or zappy quantum tunnels)
//!
//! `alexandria` provides an easy to use database interface with
//! transactions, merges and dynamic queries, ensuring that your
//! in-memory representation of data never get's out-of-sync with your
//! on-disk representation. Don't burn your data.
//!
//! ## Payload types
//!
//! `alexandria` supports key-value stores, encoded as `json` on the
//! wire format, and lazy blobs, meaning that they exist as blobs on
//! disk, and are only fetched when absolutely needed (you know, that
//! 24GB copy of Hackers we all have, but don't entirely understand
//! the origins of).
//!
//! Both `KV` and `Blob` payloads can use encryption at rest.
//!
//! ## Namespaces & Scopes
//!
//! `alexandria` also has a users concept, allowing you to construct
//! permissive layers, optionally backed by encrpted
//! storage. Referring to a location in an `alexandria` library
//! requires an `Address`, which consists of an optional namespace, a
//! scope and data ID.
//!
//! We use the following notation in documentation and external
//! queries: `lib:</namespace?>/<scope>/<ID>`.
//!
//! Each scope has metadata attributes that allow `alexandria` to
//! handle encryption, access, and on-disk offset management. What
//! that means is that a scope `lib:/me/downloads` might be saved into
//! `/home/me/downloads`, while the scope `lib:/me/secret_chat` is
//! saved into `/home/me/.local/share/chat_app/secret/`.
//!
//! ## Questions?
//!
//! Check out the `examples` directory first, there's some cool
//! ones in there (I've been told by...someone).
//!
//! `alexandria` is developed as part of
//! [qaul.net][website]. We have a [mailing list][list] and
//! an [IRC channel][irc]! Please come by and ask us questions!  (the
//! issue tracker is a bad place to ask questions)
//!
//! [website]: https://qaul.net
//! [list]: https://lists.sr.ht/~qaul/community/
//! [irc]: https://irccloud.com/freenode/#qaul.net

mod data;
mod delta;
mod keys;
mod namespace;
mod scope;
mod store;
mod user;

// === API EXPORTS ===
pub use crate::{
    data::{Data, Value},
    delta::Delta,
    keys::KeyAttr,
    namespace::Address,
    scope::ScopeAttr,
    user::User,
};

use crate::{namespace::Namespace, store::Storable};
use std::collections::BTreeMap;
use std::{fs::create_dir_all, path::Path};

/// Primary context structure for Alexandria library
///
/// Each function is pretty well documented. For more big picture
/// context, maybe look at the crate docs or the accompanying book.
#[derive(Default, Debug)]
pub struct Library {
    /// A map from Namespace-name -> Namespace. If the key is `None`,
    /// the namespace is "root"
    data: BTreeMap<Option<String>, Namespace>,
    /// A map of keys, from user-id to key value.
    keys: BTreeMap<String, String>,
}

impl Library {
    /// Create a new library in memory without any paths
    ///
    /// **Not implemented yet:**
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

    /// Modify a path from a cannonically described address
    ///
    /// This function can create, modify (change `ScopeAttr`) and
    /// delete paths. A path endpoint in this case is a `Scope`, which
    /// holds `Data`.
    ///
    /// In documentation and debug information, addresses are
    /// expressed as follows: `lib:<namespace?>/<scope>/<data_id?>`,
    /// where `namespace?` is optional and `data_id` can be omitted
    /// when using `Address::Scope`.
    ///
    /// `lib` is a hardcoded prefix to differentiate it from other
    /// filesystem paths, and can sometimes be found on wire formats.
    pub fn modify_path(&mut self, addr: Address, delta: Delta<ScopeAttr>) {
        let (ns, scope) = match addr {
            Address::Scope(ns, scope) => (ns.map(|s| s.into()), scope),
            _ => panic!("Invalid address!"),
        };

        // FIXME: This is kinda ugly
        if !self.data.contains_key(&ns) {
            self.data.insert(ns.clone(), Namespace::default());
        }

        let ns = self.data.get_mut(&ns).expect("Failed to load namespace");
        ns.modify_path(scope, delta);
    }

    /// Modify a record from a canonically described address
    ///
    /// Operations performed depend on the `Delta` that is provided,
    /// and can either insert new records, delete existing or update
    /// existing records. Check the documentation for `Delta` to see
    /// all side-effects that can occur when using `Insert` vs
    /// `Update`.
    ///
    /// This method works in a very similar way to `modify_path`, but
    /// will only accept `Address::Ns` and `Address::Root`.
    pub fn modify_record(&mut self, addr: Address, delta: Delta<Data>) {
        let (ns, scope, id) = match addr {
            Address::Ns(ns, scope, id) => (Some(ns.into()), scope, id),
            Address::Root(scope, id) => (None, scope, id),
            _ => panic!("Invalid address!"),
        };

        self.data
            .get_mut(&ns)
            .map(|ns| ns.modify_record(scope, id, delta))
            .expect("Failed to operate on non-existing Namespace");
    }

    /// Read from a canonically described address
    pub fn read(&self, addr: Address) -> Option<&Data> {
        let (ns, scope, id) = match addr {
            Address::Ns(ns, scope, id) => (Some(ns).map(|s| s.into()), scope, id),
            Address::Root(scope, id) => (None, scope, id),
            _ => panic!("Invalid address!"),
        };

        self.data.get(&ns)?.read(scope, id)
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
