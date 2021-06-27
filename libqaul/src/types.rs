use serde::{Serialize, Deserialize};
use crate::services::page::PageResponse;
use crate::services::feed::FeedMessage;

#[derive(Debug, Serialize, Deserialize)]
pub enum QaulMessageType {
    Page(PageResponse),
    Feed(FeedMessage)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QaulMessage {
    pub sender: String,
    pub receiver: String,
    pub data: QaulMessageType,
}

pub enum EventType {
    Message(QaulMessage),
    Cli(String),
}
