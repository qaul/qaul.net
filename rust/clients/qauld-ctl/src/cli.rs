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
}

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
        offset: u64,
        /// page offset
        #[arg(short, long, default_value = "10")]
        limit: u64,
    },
}
