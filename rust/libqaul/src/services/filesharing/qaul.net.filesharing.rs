/// Filesharing service network message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingContainer {
    #[prost(oneof="file_sharing_container::Message", tags="1, 2")]
    pub message: ::core::option::Option<file_sharing_container::Message>,
}
/// Nested message and enum types in `FileSharingContainer`.
pub mod file_sharing_container {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        FileInfo(super::FileSharingInfo),
        #[prost(message, tag="2")]
        FileData(super::FileSharingData),
    }
}
/// FileSharing Message Content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingInfo {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
    /// file name
    #[prost(string, tag="2")]
    pub file_name: ::prost::alloc::string::String,
    /// file extension
    #[prost(string, tag="3")]
    pub file_extension: ::prost::alloc::string::String,
    /// file size
    #[prost(uint32, tag="4")]
    pub file_size: u32,
    /// file description
    #[prost(string, tag="5")]
    pub file_descr: ::prost::alloc::string::String,
    /// start index
    #[prost(uint32, tag="6")]
    pub start_index: u32,
    /// message count
    #[prost(uint32, tag="7")]
    pub message_count: u32,
}
/// FileSharing Data Message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingData {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
    /// start index
    #[prost(uint32, tag="2")]
    pub start_index: u32,
    /// message count
    #[prost(uint32, tag="3")]
    pub message_count: u32,
    /// package data
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
