use crate::{Room, RoomId};
use std::collections::BTreeMap;

/// Create a map of rooms from a simple vector
pub(crate) fn room_map(vec: Vec<Room>) -> BTreeMap<RoomId, Room> {
    vec.into_iter().fold(BTreeMap::new(), |mut map, room| {
        map.insert(room.id, room);
        map
    })
}
