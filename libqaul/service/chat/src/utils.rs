use crate::{Chat, Room, RoomMeta, RoomId};
use libqaul::users::UserAuth;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Create a map of rooms from a simple vector
pub(crate) fn room_map(vec: Vec<Room>) -> BTreeMap<RoomId, Room> {
    vec.into_iter().fold(BTreeMap::new(), |mut map, room| {
        map.insert(room.id, room);
        map
    })
}

/// Convert an internal network room to a local chat room
pub(crate) fn get_chat_room(room: Room, unread: usize) -> RoomMeta {
    RoomMeta {
        id: room.id,
        users: room.users,
        name: room.name,
        unread,
        create_time: room.create_time
    }
}

/// Get the amount of unread messages for a room id
pub(crate) fn get_unread_message_count(serv: &Arc<Chat>, user: UserAuth, room: RoomId) -> usize {
    0
}
