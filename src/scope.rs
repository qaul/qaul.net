use crate::data::Data;
use crate::keys::KeyAttr;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A group of data
#[derive(Debug)]
pub struct Scope {
    pub(crate) attrs: ScopeAttr,
    pub(crate) files: BTreeMap<String, Data>,
}

impl Scope {
    pub fn new(attrs: ScopeAttr) -> Self {
        Self {
            attrs,
            files: Default::default(),
        }
    }

    pub(crate) fn insert(&mut self, id: &str, data: Data) {
        self.files.insert(id.into(), data);
    }

    pub(crate) fn delete(&mut self, id: &str) -> Data {
        self.files.remove(id).unwrap()
    }

    pub(crate) fn update(&mut self, _id: &str, _data: Data) {
        unimplemented!()
    }
    
    pub(crate) fn read(&self, id: &str) -> Option<&Data> {
        self.files.get(id)
    }

    pub(crate) fn all(&self) -> impl Iterator<Item = (&String, &Data)> {
        self.files.iter()
    }
}

/// Scope attributes inside a namespace
///
/// Because a namespace consists of multiple scopes, it is possible to
/// give each scope seperate attributes, such as an offset, encryption
/// or requiring authentication.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScopeAttr {
    /// Hide this scope behind authenticated users
    pub ns_auth: bool,
    /// Provide an encryption key for at-rest storage
    ///
    /// In most cases this can just be set to `KeyAttr::Off`
    pub encryption: KeyAttr,
    /// Provide an on-disk offset to store files at
    ///
    /// This could be a path such as `"/home/downloads"` or
    /// `"/home/.local/cache"`
    pub offset: String,
}

impl ScopeAttr {
    /// Merge new attributes into this ScopeAttr
    pub(crate) fn merge(&mut self, other: Self) {
        // TODO: Thinking about this function, it kinda doesn't make
        // much sense, but I want to leave the `Delta` in place
        // nonetheless. There will be more attributes that can be
        // applied to a scope and maybe we can work with something
        // that is generated from a builder/factory pattern around
        // lots of Options. Until then, this is kinda pointless and
        // the `Delta::Update` on `modify_path` doesn't really make
        // much sense to use but there we go.
        self.ns_auth = other.ns_auth;
        self.encryption = other.encryption;
        self.offset = other.offset;
    }
}
