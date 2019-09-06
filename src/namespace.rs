use crate::scope::{Scope, ScopeAttr};
use std::collections::BTreeMap;
use std::iter::Iterator;

/// Alexandria library namespace
///
/// Addressed via an ID and contains a series of scopes
#[derive(Default, Debug)]
pub struct Namespace {
    scopes: BTreeMap<String, Scope>,
}

impl Namespace {
    pub fn create_scope(&mut self, name: String, attrs: ScopeAttr) {
        self.scopes.insert(name, Scope::new(attrs));
    }

    pub fn scopes(&self) -> impl Iterator<Item = (&String, &Scope)> {
        self.scopes.iter()
    }
}

/// A unique identifiable address in a library
///
/// Acts as a wrapper around three seperate path elements, one of
/// which is optional (namespace). It provides some easy to use
/// funcitons fo r creating new addresses for inserting or retrieving
/// data from a library.
pub enum Address<'a> {
    Ns(&'a str, &'a str, &'a str),
    Root(&'a str, &'a str),
}

impl<'a> Address<'a> {
    /// Create an address to namespaced data
    pub fn with_namespace(ns: &'a str, scope: &'a str, id: &'a str) -> Self {
        Address::Ns(ns, scope, id)
    }

    /// Create an address to root-namespace data
    pub fn root(scope: &'a str, id: &'a str) -> Self {
        Address::Root(scope, id)
    }
}
