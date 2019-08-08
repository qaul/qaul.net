//! A module that synchronises a library with a disk

use crate::Alexandria;

pub(crate) trait Sync {
    fn write(&self, offset: String);
    fn read(&mut self, offset: String);
}

impl Sync for Alexandria {
    fn write(&self, offset: String) {

    }

    fn read(&mut self, offset: String) {

    }
}
