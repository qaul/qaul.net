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

    pub fn push(&mut self, name: String, data: Data) {
        self.files.insert(name, data);
    }

    pub fn get(&self, name: &str) -> &Data {
        self.files.get(name).unwrap()
    }

    pub fn all(&self) -> impl Iterator<Item = (&String, &Data)> {
        self.files.iter()
    }

    pub fn pop(&mut self, name: &str) -> Data {
        self.files.remove(name).unwrap()
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
