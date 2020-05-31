use crate::{
    error::{Error, Result},
    tags, FileId, FileMeta, ASC_NAME,
};
use async_std::{
    sync::{channel, Arc, Receiver, Sender},
    task,
};
use bincode;
use libqaul::{helpers::Tag, services::MetadataMap, users::UserAuth, Qaul};

pub(crate) type DirectoryRef = Arc<FileDirectory>;

/// Keeps track of known rooms via the service metadata API
pub(crate) struct FileDirectory {
    qaul: Arc<Qaul>,
    notify: (Sender<FileId>, Receiver<FileId>),
}

impl FileDirectory {
    pub(crate) fn new(qaul: Arc<Qaul>) -> Arc<Self> {
        Arc::new(Self {
            qaul,
            notify: channel(1),
        })
    }

    async fn get_inner(&self, user: UserAuth) -> MetadataMap {
        let mut map_result = self
            .qaul
            .services()
            .query(user, ASC_NAME, Tag::empty(tags::FILE_LIST))
            .await
            .unwrap();
        map_result.reverse();
        map_result
            .pop()
            .unwrap_or_else(|| MetadataMap::new(tags::FILE_LIST))
    }

    /// Get all known rooms for a user
    pub(crate) async fn get_all(&self, user: UserAuth) -> Vec<FileMeta> {
        let meta = self.get_inner(user).await;

        meta.iter()
            .map(|(_, v)| bincode::deserialize(v).unwrap())
            .collect()
    }

    /// Get just one room, by Id
    pub(crate) async fn get(&self, user: UserAuth, id: FileId) -> Result<FileMeta> {
        let meta = self.get_inner(user).await;
        meta.iter().fold(Err(Error::NoSuchFile), |opt, (id_, vec)| {
            opt.or_else(|prev| {
                if id_ == &id.to_string() {
                    Ok(bincode::deserialize(vec).unwrap())
                } else {
                    Err(prev)
                }
            })
        })
    }

    /// Insert a room to the directory (overrides)
    pub(crate) async fn insert(&self, user: UserAuth, meta: &FileMeta) {
        self.qaul
            .services()
            .save(
                user.clone(),
                ASC_NAME,
                self.get_inner(user)
                    .await
                    .add(meta.id().to_string(), bincode::serialize(meta).unwrap()),
                Tag::empty(tags::FILE_LIST),
            )
            .await
            .unwrap();

        let file_id = meta.id();
        let sender = self.notify.0.clone();
        task::spawn(async move { sender.send(file_id).await });
    }

    /// Delete database information about a file for whatever reason
    pub(crate) async fn delete(&self, user: UserAuth, id: &FileId) {
        self.qaul
            .services()
            .save(
                user.clone(),
                ASC_NAME,
                self.get_inner(user).await.delete(id.to_string()),
                Tag::empty(tags::FILE_LIST),
            )
            .await
            .unwrap();
    }

    pub(crate) async fn poll_new(self: &Arc<Self>) -> FileId {
        self.notify.1.recv().await.unwrap()
    }
}
