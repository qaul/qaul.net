//! Alexandria namespace handling

use crate::{
    data::Data,
    delta::Delta,
    scope::{Scope, ScopeAttr},
};

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
    /// Modify a path in this particular namespace
    pub(crate) fn modify_path(&mut self, scope: &str, delta: Delta<ScopeAttr>) {
        match delta {
            Delta::Insert(attrs) => {
                self.scopes.insert(scope.into(), Scope::new(attrs));
            }
            Delta::Delete => {
                self.scopes.remove(scope);
            }
            Delta::Update(attrs) => self
                .scopes
                .get_mut(scope)
                .expect("Failed to update scope!")
                .attrs
                .merge(attrs),
        };
    }

    pub fn modify_record(&mut self, scope: &str, id: &str, delta: Delta<Data>) {
        let scope = self.scopes.get_mut(scope).unwrap();

        match delta {
            Delta::Insert(data) => {
                scope.insert(id, data);
            }
            Delta::Delete => {
                scope.delete(id);
            }
            Delta::Update(data) => {
                scope.update(id, data);
            }
        }
    }

    pub fn scopes(&self) -> impl Iterator<Item = (&String, &Scope)> {
        self.scopes.iter()
    }

    pub fn read(&self, scope: &str, id: &str) -> Option<&Data> {
        self.scopes.get(scope)?.read(id)
    }
}

/// A unique identifiable address in a library
///
/// Acts as a wrapper around three seperate path elements, one of
/// which is optional (namespace). It provides some easy to use
/// funcitons fo r creating new addresses for inserting or retrieving
/// data from a library.
pub enum Address<'a> {
    /// Refer to a record in a namespaced scope
    Ns(&'a str, &'a str, &'a str),
    /// Refer to a record in a root-namespaced scope
    Root(&'a str, &'a str),
    /// Refer to a scope directly (used in `Delta`s)
    Scope(Option<&'a str>, &'a str),
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

    /// Create an address to a scope
    pub fn scope(ns: Option<&'a str>, scope: &'a str) -> Self {
        Address::Scope(ns, scope)
    }
}
