use crate::{data::TagSet, Id, Path};

pub(crate) struct DeltaBuilder {
    user: Option<Id>,
    path: Option<Path>,
    rec_id: Option<Id>,
    tags: Option<TagSet>,
    action: DeltaType,
}

impl DeltaBuilder {
    pub(crate) fn new(user: Option<Id>, action: DeltaType) -> Self {
        Self {
            action,
            user,
            path: None,
            rec_id: None,
            tags: Some(TagSet::empty()),
        }
    }

    pub(crate) fn path(&mut self, path: &Path) {
        self.path = Some(path.clone());
    }

    pub(crate) fn rec_id(&mut self, rec_id: Id) {
        self.rec_id = Some(rec_id);
    }

    pub(crate) fn tags(&mut self, tags: &TagSet) {
        self.tags = Some(tags.clone());
    }

    pub(crate) fn make(self) -> Delta {
        Delta {
            user: self.user,
            path: self.path.unwrap(),
            rec_id: self.rec_id,
            tags: self.tags.unwrap(),
            action: self.action,
        }
    }
}

/// A transaction to the active dataset of a library
///
/// A delta is atomic, touches one field of one record, and can reside in the hot
/// cache before being fully commited.  It is authenticated against an
/// active user before being cached.
///
/// The `path` segment is constructed via the
pub(crate) struct Delta {
    pub(crate) user: Option<Id>,
    pub(crate) path: Path,
    pub(crate) rec_id: Option<Id>,
    pub(crate) tags: TagSet,
    pub(crate) action: DeltaType,
}

pub(crate) enum DeltaType {
    Insert,
    Update,
    Delete,
}
