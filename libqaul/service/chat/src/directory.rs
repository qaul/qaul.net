use crate::{tags, Error, Result, Room, RoomDiff, RoomId, ASC_NAME};
use async_std::sync::Arc;
use bincode;
use libqaul::{helpers::Tag, services::MetadataMap, users::UserAuth, Qaul};

/// Keeps track of known rooms via the service metadata API
pub(crate) struct RoomDirectory {
    qaul: Arc<Qaul>,
}

impl RoomDirectory {
    pub(crate) fn new(qaul: Arc<Qaul>) -> Self {
        Self { qaul }
    }

    async fn get_inner(&self, user: UserAuth) -> MetadataMap {
        let mut map_result = self
            .qaul
            .services()
            .query(user, ASC_NAME, Tag::empty(tags::ROOM_LIST))
            .await
            .unwrap();
        map_result.reverse();
        map_result
            .pop()
            .unwrap_or_else(|| MetadataMap::new(tags::ROOM_LIST))
    }

    /// Get all known rooms for a user
    pub(crate) async fn get_all(&self, user: UserAuth) -> Vec<Room> {
        let meta = self.get_inner(user).await;

        meta.iter()
            .map(|(_, v)| bincode::deserialize(v).unwrap())
            .collect()
    }

    /// Get just one room, by Id
    pub(crate) async fn get(&self, user: UserAuth, id: RoomId) -> Result<Room> {
        let meta = self.get_inner(user).await;
        meta.iter().fold(Err(Error::NoSuchRoom), |opt, (id_, vec)| {
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
    pub(crate) async fn insert(&self, user: UserAuth, room: &Room) {
        self.qaul
            .services()
            .save(
                user.clone(),
                ASC_NAME,
                self.get_inner(user)
                    .await
                    .add(room.id.to_string(), bincode::serialize(room).unwrap()),
                Tag::empty(tags::ROOM_LIST),
            )
            .await
            .unwrap();
    }

    /// Apply a diff to a room
    pub(crate) async fn apply_diff(&self, user: UserAuth, diff: &RoomDiff) {
        let mut room = self.get(user.clone(), diff.id).await.unwrap();
        room.apply(diff);
        self.insert(user, &room).await;
    }
}
