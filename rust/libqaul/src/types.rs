use serde::{Serialize, Deserialize};
use crate::services::page::PageResponse;
use crate::services::feed::FeedMessageSendContainer;

#[derive(Debug, Serialize, Deserialize)]
pub enum QaulMessageType {
    Page(PageResponse),
    Feed(FeedMessageSendContainer)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QaulMessage {
    pub sender: String,
    pub receiver: String,
    pub data: QaulMessageType,
}
