//! All internal types that are mapped onto the web interface
//! 
//! A lot of the functionality might seem duplicated. In fact
//! we are abstracting away internal complexity to single 
//! fields or values in order to make the web interface 
//! easier to use.
//! 

/// A user inside qaul.net
/// 
/// Not all information has to be provided in a transaction.
/// When writing to an API endpoint, only fields that are 
/// provided will be changed.
#[derive(Serialize, Deserialize)]
pub struct User {
    id: u64,
    username: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    trust: Option<i8>,
    fp_token: Option<String>,

    age: Option<u8>,
    gender: Option<String>,
    avatar: Option<Vec<u8>>,
}

/// A wrapper around multiple users to emulate groups
/// inside qaul.net
#[derive(Serialize, Deserialize)]
pub struct Group {
    /// Group ID in the databsse
    id: u64,
    /// Members in the group by their database ID
    members: Vec<u64>
}

/// A specifier if an owner type is a [[User]] or [[Group]]
#[derive(Serialize, Deserialize)]
pub enum UserType {
    User, Group
}

/// Describes the lifecycle of a file in qaul.net
#[derive(Serialize, Deserialize)]
pub enum DownloadStatus {
    /// If a file is unknown it's most likely that an error
    /// has occured. Usually it means that a file discovery
    /// wasn't properly logged in the database or there
    /// is a different bug inside the qaul core.
    /// 
    /// It might also mean that a file no longer exists on disk
    /// even though the database claims it does.
    Unknown,
    /// A file that the qaul core knows about but hasn't yet
    /// started downloading.
    Discovered,
    /// A file that is either downloaded or in the process of 
    /// being downloaded. Any value above `100` means that the
    /// file is safely stored on disk.
    Downloaded(u8),
}

/// A file as represented in qaul.net
/// 
/// All fields must be provided in a transaction as it's
/// not possible to have
#[derive(Serialize, Deserialize)]
pub struct File {
    /// The database ID
    id: u64,
    /// The ID of the file owner
    owner: Vec<(UserType, u64)>,
    /// Filename as per the filesystem
    filename: String,
    /// File extention per the filesystem
    extention: String,
    /// Binary blob that contains the file
    contents: Vec<u8>,
    /// Describes the status of this file
    status: DownloadStatus,
}
