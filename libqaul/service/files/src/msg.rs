use libqaul::messages::Message;
use crate::File;

impl From<Message> for File {
    fn from(msg: Message) -> Self {
        Self {
            name: Some(msg.id.to_string()), // TODO: how to get name here?
            id: msg.id,
            data: Some(msg.payload),
            owner: msg.sender,
        }
    }
}