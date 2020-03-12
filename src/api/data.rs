//! Data handling

use crate::Library;

pub struct Data<'a> {
    inner: &'a Library,
}

impl<'a> Data<'a> {
    pub fn drop(&'a self) -> &'a Library {
        self.inner
    }

    pub fn insert(&self) {}

    pub fn delete(&self) {}

    pub fn update(&self) {}

    pub fn query(&self) {}
}
