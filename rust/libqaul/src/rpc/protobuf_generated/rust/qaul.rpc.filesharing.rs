/// File sharing service RPC message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharing {
    /// message type
    #[prost(oneof="file_sharing::Message", tags="1, 2, 3")]
    pub message: ::core::option::Option<file_sharing::Message>,
}
/// Nested message and enum types in `FileSharing`.
pub mod file_sharing {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// request for sending file
        #[prost(message, tag="1")]
        SendFileRequest(super::SendFileRequest),
        /// file histories request
        #[prost(message, tag="2")]
        FileHistory(super::FileHistoryRequest),
        ///file histories response
        #[prost(message, tag="3")]
        FileHistoryResponse(super::FileHistoryResponse),
    }
}
/// Send File Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendFileRequest {
    /// file path name to send
    #[prost(string, tag="1")]
    pub path_name: ::prost::alloc::string::String,
    /// conversation id to receive file
    #[prost(bytes="vec", tag="2")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// description
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
/// File History Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHistoryRequest {
    /// offset
    #[prost(uint32, tag="1")]
    pub offset: u32,
    /// limit
    #[prost(uint32, tag="2")]
    pub limit: u32,
}
/// File History Entry
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHistoryEntry {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
    /// file name
    #[prost(string, tag="2")]
    pub file_name: ::prost::alloc::string::String,
    /// file extension
    #[prost(string, tag="3")]
    pub file_ext: ::prost::alloc::string::String,
    /// file size
    #[prost(uint32, tag="4")]
    pub file_size: u32,
    /// file description
    #[prost(string, tag="5")]
    pub file_descr: ::prost::alloc::string::String,
    /// time
    #[prost(uint64, tag="6")]
    pub time: u64,
    /// sent/recv
    #[prost(bool, tag="7")]
    pub sent: bool,
    /// peer id
    #[prost(string, tag="8")]
    pub peer_id: ::prost::alloc::string::String,
}
/// File History Response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHistoryResponse {
    /// offset
    #[prost(uint32, tag="1")]
    pub offset: u32,
    /// limit
    #[prost(uint32, tag="2")]
    pub limit: u32,
    /// limit
    #[prost(uint64, tag="3")]
    pub total: u64,
    /// histories
    #[prost(message, repeated, tag="4")]
    pub histories: ::prost::alloc::vec::Vec<FileHistoryEntry>,
}
