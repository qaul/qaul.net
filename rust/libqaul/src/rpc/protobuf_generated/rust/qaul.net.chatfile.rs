/// Chat file sending container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatFileContainer {
    #[prost(oneof = "chat_file_container::Message", tags = "1, 2")]
    pub message: ::core::option::Option<chat_file_container::Message>,
}
/// Nested message and enum types in `ChatFileContainer`.
pub mod chat_file_container {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Chat File Info Message
        #[prost(message, tag = "1")]
        FileInfo(super::ChatFileInfo),
        /// Chat File Data Message
        #[prost(message, tag = "2")]
        FileData(super::ChatFileData),
    }
}
/// Chat File Info Message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatFileInfo {
    /// file id
    #[prost(uint64, tag = "1")]
    pub file_id: u64,
    /// file name
    #[prost(string, tag = "2")]
    pub file_name: ::prost::alloc::string::String,
    /// file extension
    #[prost(string, tag = "3")]
    pub file_extension: ::prost::alloc::string::String,
    /// file size
    #[prost(uint32, tag = "4")]
    pub file_size: u32,
    /// file description
    #[prost(string, tag = "5")]
    pub file_description: ::prost::alloc::string::String,
    /// DEPRECATED: What is this used for?
    /// start index
    #[prost(uint32, tag = "6")]
    pub start_index: u32,
    /// message count
    #[prost(uint32, tag = "7")]
    pub message_count: u32,
    /// file data chunk size
    #[prost(uint32, tag = "8")]
    pub data_chunk_size: u32,
}
/// Chat File Data Message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatFileData {
    /// file id
    #[prost(uint64, tag = "1")]
    pub file_id: u64,
    /// start index
    #[prost(uint32, tag = "2")]
    pub start_index: u32,
    /// message count
    #[prost(uint32, tag = "3")]
    pub message_count: u32,
    /// package data
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
