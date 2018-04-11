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
#[derive(Default, Serialize, Deserialize)]
pub(crate) struct User {
    /// An ID to refer to this user as the database does
    id: u64,
    /// The fingerprint token
    fp_token: Option<String>,
    /// Fingerprint token avatar (png encoding)
    fp_token_render: Option<Vec<u8>>,
    /// The user/ display name of this User
    username: Option<String>,
    /// Password when changing it. Never read!
    password: Option<String>,
    /// The bio text for this user
    bio: Option<String>,
    /// Trust level set for this user (-255 to 255)
    trust: Option<i8>,
    /// Is this user set as "favourite" in contact book?
    starred: Option<bool>,
    /// The age of this user
    age: Option<u8>,
    /// This users gender (we're a social network, y'all!)
    gender: Option<String>,
    /// An optional picture avatar
    avatar: Option<Vec<u8>>,
}

/// A wrapper around multiple users to emulate groups
/// inside qaul.net
#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Group {
    /// Group ID in the databsse
    id: u64,
    /// Members in the group by their database ID
    members: Vec<u64>,
}

/// A specifier if an owner type is a [[User]] or [[Group]]
#[derive(Serialize, Deserialize)]
pub(crate) enum UserType {
    /// A single user
    User,
    /// A group of users
    Group,
}

impl Default for UserType {
    fn default() -> UserType {
        UserType::User
    }
}


/// Describes the lifecycle of a file in qaul.net
#[derive(Serialize, Deserialize)]
pub(crate) enum DownloadStatus {
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

impl Default for DownloadStatus {
    fn default() -> DownloadStatus {
        DownloadStatus::Unknown
    }
}

/// A file as represented in qaul.net
///
/// All fields must be provided in a transaction as it's
/// not possible to have
#[derive(Default, Serialize, Deserialize)]
pub(crate) struct File {
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

/// Describe a network interface available on the device
#[derive(Default, Serialize, Deserialize)]
pub(crate) struct NetInterface {
    id: u64,
    name: String,
    shared: (bool, u64),
}

/// Represents a node on the qaul network
///
/// Optionally all users known from that node can
/// be listed.
#[derive(Default, Serialize, Deserialize)]
pub(crate) struct NetworkNode {
    /// A node ID used for the graph renderer
    id: u64,
    /// The IP of a node (simply rendered)
    ip: String,
    /// Optionally: all known users from a node
    users: Option<Vec<u64>>,
}

/// Represents the known network graph.
#[derive(Default, Serialize, Deserialize)]
pub(crate) struct NtworkGraph {
    /// All nodes in the backend
    nodes: Vec<u64>,
    /// All connections between nodes
    connections: Vec<(u64, u64)>,
}
