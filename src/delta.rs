use crate::utils::{Id, Path, TagSet};

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
            rec_id: self.rec_id,
            action: self.action,
            tags: self.tags.unwrap_or_else(|| TagSet::empty()),
            path: self.path.unwrap(),
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
#[derive(Clone, Debug)]
pub(crate) struct Delta {
    pub(crate) user: Option<Id>,
    pub(crate) rec_id: Option<Id>,
    pub(crate) path: Path,
    pub(crate) tags: TagSet,
    pub(crate) action: DeltaType,
}

#[derive(Clone, Debug)]
pub(crate) enum DeltaType {
    Insert,
    Update,
    Delete,
}
