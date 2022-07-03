#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingContainer {
    #[prost(oneof="file_sharing_container::Message", tags="1, 2, 3, 4, 5, 6")]
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
        #[prost(message, tag="3")]
        Confirmation(super::FileSharingConfirmation),
        #[prost(message, tag="4")]
        ConfirmationInfo(super::FileSharingConfirmationFileInfo),
        #[prost(message, tag="5")]
        Completed(super::FileSharingCompleted),
        #[prost(message, tag="6")]
        Canceled(super::FileSharingCanceled),
    }
}
/// FileSharing Message Content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingInfo {
    /// file name
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
    /// file extension
    #[prost(string, tag="2")]
    pub file_extension: ::prost::alloc::string::String,
    /// file size
    #[prost(uint32, tag="3")]
    pub file_size: u32,
    /// file description
    #[prost(string, tag="4")]
    pub file_descr: ::prost::alloc::string::String,
    /// size per package 
    #[prost(uint32, tag="5")]
    pub size_per_package: u32,
    /// file id
    #[prost(uint64, tag="6")]
    pub file_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingData {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
    /// package sequence
    #[prost(uint32, tag="2")]
    pub sequence: u32,
    /// file size
    #[prost(uint32, tag="3")]
    pub file_size: u32,
    /// size per package 
    #[prost(uint32, tag="4")]
    pub size_per_package: u32,
    /// package data
    #[prost(bytes="vec", tag="6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingConfirmationFileInfo {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingConfirmation {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
    /// package sequence
    #[prost(uint32, tag="2")]
    pub sequence: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingCompleted {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSharingCanceled {
    /// file id
    #[prost(uint64, tag="1")]
    pub file_id: u64,
}
