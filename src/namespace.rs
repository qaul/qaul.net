use crate::scope::{Scope, ScopeAttrs};
use std::collections::BTreeMap;

/// Alexandria library namespace
///
/// Addressed via an ID and contains a series of scopes
#[derive(Default)]
pub struct Namespace {
    scopes: BTreeMap<String, Scope>,
}

impl Namespace {
    pub fn create_scope(&mut self, name: String, attrs: ScopeAttrs) {
        self.scopes.insert(name, Scope::new(attrs));
    }
}
