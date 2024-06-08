/// Chat file RPC message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatFile {
    /// message type
    #[prost(oneof = "chat_file::Message", tags = "1, 2, 3, 4")]
    pub message: ::core::option::Option<chat_file::Message>,
}
/// Nested message and enum types in `ChatFile`.
pub mod chat_file {
    /// message type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// send file request
        ///
        /// this messages sends a file from UI to libqaul
        #[prost(message, tag = "1")]
        SendFileRequest(super::SendFileRequest),
        /// send file response
        ///
        /// response message from libqaul to the UI about
        /// the result of the send file request
        #[prost(message, tag = "2")]
        SendFileResponse(super::SendFileResponse),
        /// file history request
        ///
        /// request a paginated list of
        #[prost(message, tag = "3")]
        FileHistory(super::FileHistoryRequest),
        /// file history response
        ///
        /// delivers the requested list of
        #[prost(message, tag = "4")]
        FileHistoryResponse(super::FileHistoryResponse),
    }
}
/// Send File Request
///
/// UI requests libqaul to send a file
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendFileRequest {
    /// file path with file name to send
    #[prost(string, tag = "1")]
    pub path_name: ::prost::alloc::string::String,
    /// group id to receive file
    #[prost(bytes = "vec", tag = "2")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    /// file description text to be sent in the message
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// Send File Response
///
/// sends the result of the file send request to the UI
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendFileResponse {
    /// was the file processing successful
    ///
    /// a success does not mean the file has been sent,
    /// but that it was successfully scheduled for sending.
    #[prost(bool, tag = "1")]
    pub success: bool,
    /// error reason
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    /// file ID (only present if the sending was a success)
    #[prost(uint64, tag = "3")]
    pub file_id: u64,
}
/// File History Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHistoryRequest {
    /// offset
    #[prost(uint32, tag = "1")]
    pub offset: u32,
    /// limit
    #[prost(uint32, tag = "2")]
    pub limit: u32,
}
/// File History Entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHistoryEntry {
    /// file id
    #[prost(uint64, tag = "1")]
    pub file_id: u64,
    /// file name (without extension)
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
    /// time
    #[prost(uint64, tag = "6")]
    pub time: u64,
    /// sender id
    #[prost(string, tag = "7")]
    pub sender_id: ::prost::alloc::string::String,
    /// group id
    #[prost(string, tag = "8")]
    pub group_id: ::prost::alloc::string::String,
}
/// File History Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHistoryResponse {
    /// offset
    #[prost(uint32, tag = "1")]
    pub offset: u32,
    /// limit
    #[prost(uint32, tag = "2")]
    pub limit: u32,
    /// limit
    #[prost(uint64, tag = "3")]
    pub total: u64,
    /// histories
    #[prost(message, repeated, tag = "4")]
    pub histories: ::prost::alloc::vec::Vec<FileHistoryEntry>,
}
