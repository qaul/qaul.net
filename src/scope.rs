
use crate::keys::KeyAttr;
use crate::data::Data;
use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

/// A group of data
pub struct Scope {
    pub(crate) attrs: ScopeAttrs,
    pub(crate) files: BTreeMap<String, Data>,
}

impl Scope {
    pub fn new(attrs: ScopeAttrs) -> Self {
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

    pub fn pop(&mut self, name: &str) -> Data {
        self.files.remove(name).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScopeAttrs {
    pub ns_auth: bool,
    pub encryption: KeyAttr,
    pub offset: String,
}
