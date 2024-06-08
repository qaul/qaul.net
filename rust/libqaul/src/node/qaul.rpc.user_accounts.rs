/// user account rpc message container
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAccounts {
    #[prost(oneof = "user_accounts::Message", tags = "1, 2, 3, 4")]
    pub message: ::core::option::Option<user_accounts::Message>,
}
/// Nested message and enum types in `UserAccounts`.
pub mod user_accounts {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(bool, tag = "1")]
        GetDefaultUserAccount(bool),
        #[prost(message, tag = "2")]
        CreateUserAccount(super::CreateUserAccount),
        #[prost(message, tag = "3")]
        DefaultUserAccount(super::DefaultUserAccount),
        #[prost(message, tag = "4")]
        MyUserAccount(super::MyUserAccount),
    }
}
/// create a new user on this node
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserAccount {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Session Information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultUserAccount {
    #[prost(bool, tag = "1")]
    pub user_account_exists: bool,
    #[prost(message, optional, tag = "2")]
    pub my_user_account: ::core::option::Option<MyUserAccount>,
}
/// Information about my user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyUserAccount {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub id_base58: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub key_type: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub key_base58: ::prost::alloc::string::String,
}
