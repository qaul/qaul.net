// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.
//! CLI template for qauld-ctl

use clap::{Args, Parser, Subcommand};

/// qauld-ctl CLI: Control a running qauld daemon instance
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Explicit path to qauld sock. e.g /path/to/qauld.sock
    #[arg(short, long, env = "QAULD_SOCKET")]
    pub socket: Option<String>,
    /// Specify a directory to look for qauld.sock in
    #[arg(short, long)]
    pub dir: Option<String>,

    /// Specify if the output should be in JSON
    #[arg(short, long, default_value = "false")]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// node details
    Node(NodeArgs),
    /// User Accounts
    Account(AccountArgs),
    /// Functions for all users known by your node
    Users(UsersArgs),
    /// Feed
    Feed(FeedArgs),
    /// Group
    Group(GroupArgs),
    /// Chat
    Chat(ChatArgs),
    /// chat files
    File(ChatFileArgs),
    /// Router information
    Router(RouterArgs),
    /// Debug commands (libqaul-side diagnostics).
    ///
    /// These are intentionally low-overhead RPC round-trips used to
    /// validate the daemon is reachable and behaving. Ported from the
    /// legacy `qaul-cli` `debug` subcommands.
    Debug(DebugArgs),
    /// Manage statically-configured internet peer nodes
    ///
    /// Ported from the legacy `qaul-cli` `connections` subcommands.
    Connections(ConnectionsArgs),
    /// Delay-Tolerant Networking storage controls (V1)
    ///
    /// Ported from the legacy `qaul-cli` `dtn` subcommands. V2 source-
    /// routed DTN (custody routing) is a separate feature on the
    /// `feat/dtn-custody-routing` branch and is not in this build.
    Dtn(DtnArgs),
    /// Start an interactive shell session
    ///
    /// Reads commands from stdin in a REPL loop and dispatches them through
    /// the same RPC client used by single-shot mode. Useful for poking at a
    /// running qauld daemon without re-launching qauld-ctl per command.
    Shell(ShellArgs),
    /// Subscribe to live events from qauld
    ///
    /// Opens a long-running RPC and prints each event (chat messages,
    /// peers connecting/disconnecting, etc.) as it arrives. Run in a
    /// dedicated terminal alongside the shell. Stop with Ctrl-C.
    Subscribe(SubscribeArgs),
}

#[derive(Args, Debug)]
pub struct ShellArgs {}

#[derive(Args, Debug)]
pub struct DebugArgs {
    #[command(subcommand)]
    pub command: DebugSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum DebugSubcmd {
    /// request the storage path from libqaul
    Path,
    /// send a heartbeat request; libqaul replies with a Heartbeat response
    Heartbeat,
    /// crash libqaul on purpose (for testing crash logging). Fire-and-forget.
    Panic,
    /// log subcommands (toggle libqaul's file logging)
    Log(LogArgs),
    // NOTE: `rpc sent` and `rpc queued` from qaul-cli read libqaul-side
    // counters in-process. They need a new Debug RPC message to work over
    // qauld's socket — deferred to a follow-up PR.
}

#[derive(Args, Debug)]
pub struct LogArgs {
    #[command(subcommand)]
    pub command: LogSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum LogSubcmd {
    /// enable libqaul logging to file. Fire-and-forget.
    Enable,
    /// disable libqaul logging to file. Fire-and-forget.
    Disable,
}

#[derive(Args, Debug)]
pub struct ConnectionsArgs {
    #[command(subcommand)]
    pub command: ConnectionsSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum ConnectionsSubcmd {
    /// node operations on the internet peer list
    Nodes(NodesArgs),
}

#[derive(Args, Debug)]
pub struct NodesArgs {
    #[command(subcommand)]
    pub command: NodesSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum NodesSubcmd {
    /// list all statically configured internet peer nodes
    List,
    /// add a new internet peer node
    Add {
        /// libp2p multiaddress (e.g. `/ip4/144.91.74.192/udp/9229/quic-v1`)
        #[arg(short, long)]
        address: String,
        /// human-readable name to attach to the node
        #[arg(short, long)]
        name: String,
    },
    /// remove an internet peer node
    Remove {
        /// libp2p multiaddress to remove
        #[arg(short, long)]
        address: String,
    },
    /// rename an internet peer node
    Rename {
        /// libp2p multiaddress of the node to rename
        #[arg(short, long)]
        address: String,
        /// new name to attach
        #[arg(short, long)]
        name: String,
    },
    /// activate an internet peer node (enable outbound dialing)
    Activate {
        #[arg(short, long)]
        address: String,
    },
    /// deactivate an internet peer node
    Deactivate {
        #[arg(short, long)]
        address: String,
    },
}

#[derive(Args, Debug)]
pub struct DtnArgs {
    #[command(subcommand)]
    pub command: DtnSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum DtnSubcmd {
    /// display DTN storage state (size used, message counts)
    State,
    /// display DTN configuration (max total size, allowed users)
    Config,
    /// add a storage user (allow this user to deposit DTN messages here)
    Add {
        /// base58-encoded user id
        #[arg(short, long)]
        user_id: String,
    },
    /// remove a storage user
    Remove {
        /// base58-encoded user id
        #[arg(short, long)]
        user_id: String,
    },
    /// set the maximum total DTN storage size in megabytes
    Size {
        /// max size in MB
        #[arg(short, long)]
        size: u32,
    },
}

#[derive(Args, Debug)]
pub struct SubscribeArgs {}

#[derive(Args, Debug)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub command: NodeSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum NodeSubcmd {
    /// prints the local node id
    Info,
}

#[derive(Args, Debug)]
pub struct AccountArgs {
    #[command(subcommand)]
    pub command: AccountSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum AccountSubcmd {
    /// get's and displays the default user account
    Default,
    /// create a new user account with the name {User Name}
    Create {
        /// Specify the username to create an account with
        #[arg(short, long)]
        username: String,
        /// Specify the password to create an account with
        #[arg(short, long)]
        password: Option<String>,
    },
    /// set or change password for the current user account
    Password {
        /// Specify the password to create or change
        #[arg(short, long)]
        password: String,
    },
    /// login to an existing user account
    Login {
        /// Specify the username to create an account with
        #[arg(short, long)]
        username: String,
        /// Specify the password to create an account with
        #[arg(short, long)]
        password: String,
    },
    /// logout from the current user session
    Logout,
    /// check current authentication status (logged in/out, session info)
    Status,
}

#[derive(Args, Debug)]
pub struct UsersArgs {
    #[command(subcommand)]
    pub command: UsersSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum UsersSubcmd {
    /// display all users known to this router
    List,
    /// display all online users known to this router
    Online,
    /// verify user with {User ID}
    Verify {
        /// Specify the User ID to verify
        #[arg(short, long)]
        user_id: String,
    },
    /// block user with {User ID}
    Block {
        /// Specify the User ID to block
        #[arg(short, long)]
        user_id: String,
    },
    /// get the security number for a specific user
    Secure {
        /// Specify the User ID to get security number
        #[arg(short, long)]
        user_id: String,
    },
    /// get detailed information for a single user by their {User ID}
    Get {
        /// Specify the User ID to get details for
        #[arg(short, long)]
        user_id: String,
    },
}

#[derive(Args, Debug)]
pub struct FeedArgs {
    #[command(subcommand)]
    pub command: FeedSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum FeedSubcmd {
    Send {
        /// sends the {FeedMessage} to the network and distributes it to all connected nodes
        ///  * the message is signed and can be validated
        ///  * at least one user needs to be created
        #[arg(short, long)]
        message: String,
    },
    /// displays all feed messages
    List {
        /// displays only feed messages newer than {Feed Message ID}
        #[arg(short, long)]
        feed_message_id: Option<u64>,
    },
    /// displays feed messages with pagination
    Page {
        /// page offset (0 = first page)
        #[arg(short, long, default_value = "0")]
        offset: u32,
        /// maximum number of messages to return
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
}

#[derive(Args, Debug)]
pub struct GroupArgs {
    #[command(subcommand)]
    pub command: GroupSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum GroupSubcmd {
    /// creates a new group
    Create {
        /// name of the group to create
        #[arg(short, long)]
        name: String,
    },
    /// list all available groups
    List,
    /// shows the group information
    Info {
        /// the group id
        #[arg(short, long)]
        id: String,
    },
    /// invite a user to a group
    Invite {
        /// the group id
        #[arg(short, long)]
        group_id: String,
        /// the user to add the group
        #[arg(short, long)]
        user_id: String,
    },
    /// list received pending invitations
    Invited,
    /// accept group invitation
    Accept {
        /// the group id
        #[arg(short, long)]
        group_id: String,
    },
    /// decline group invitation
    Decline {
        /// the group id
        #[arg(short, long)]
        group_id: String,
    },
    /// remove a group member from the group
    Remove {
        /// the group id
        #[arg(short, long)]
        group_id: String,
        /// the user to add the group
        #[arg(short, long)]
        user_id: String,
    },
    /// rename a group
    Rename {
        /// the group id
        #[arg(short, long)]
        group_id: String,
        /// the new name for the group
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Args, Debug)]
pub struct ChatArgs {
    #[command(subcommand)]
    pub command: ChatSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum ChatSubcmd {
    /// sends the {Chat Message} to the user with the ID {Group ID}
    Send {
        /// message to send
        #[arg(short, long)]
        message: String,
        /// the group id to send the message to
        #[arg(short, long)]
        group_id: String,
    },
    /// displays all messages of the conversation with the ID {Group ID}
    Conversation {
        /// the group id to get the conversations
        #[arg(short, long)]
        group_id: String,
        /// the index of the chat to get the messages from
        #[arg(short, long, default_value = "0")]
        index: u64,
    },
}

#[derive(Args, Debug)]
pub struct RouterArgs {
    #[command(subcommand)]
    pub command: RouterSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum RouterSubcmd {
    /// request and display routing table with per module connectivity per user
    Table,
    /// request and display neighbours list of all neighbouring nodes.
    Neighbours,
    /// request and display connections table, with all known connections per connection module.
    Connections,
}

#[derive(Args, Debug)]
pub struct ChatFileArgs {
    #[command(subcommand)]
    pub command: ChatFileSubcmd,
}

#[derive(Debug, Subcommand)]
pub enum ChatFileSubcmd {
    /// sends a file to the user with the ID {Group ID} and a {File Description} text.
    Send {
        /// the group id to send the file to
        #[arg(short, long)]
        group_id: String,

        /// the file path
        #[arg(short, long)]
        file: String,

        /// a description for the file to be sent
        #[arg(short, long)]
        description: String,
    },
    /// displays a paginated file history.
    /// The page {offset} and {limit} values are optional. The default values are an offset of 0 and 10 results.
    History {
        /// page offset
        #[arg(short, long, default_value = "0")]
        offset: u32,
        /// page offset
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
}
