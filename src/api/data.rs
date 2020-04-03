//! Data handling

use crate::{
    data::{Record, Tag, Type},
    Diff, Id, Library, Path, Result,
};

pub struct Data<'a> {
    inner: &'a Library,
    id: Option<Id>,
}

impl<'a> Data<'a> {
    pub fn drop(&'a self) -> &'a Library {
        self.inner
    }

    /// Insert a new record into the library
    ///
    /// You need to have a valid and active user session to do so,
    /// the `path` must not collide with an existing record, and the
    /// data must be valid for the selected type.
    pub fn insert<D>(&self, path: Path, tags: Vec<Tag>, t: Type, data: D) -> Result<()>
    where
        D: Into<Diff>,
    {
        Ok(())
    }

    pub fn delete(&self, path: Path) -> Result<()> {
        Ok(())
    }

    /// Update a record in-place
    pub fn update<D>(&self, path: Path, diff: D) -> Result<()>
    where
        D: Into<Diff>,
    {
        let diff: Diff = diff.into();

        Ok(())
    }

    pub fn query(&self, path: Path) -> Result<Record> {
        unimplemented!()
    }
}
