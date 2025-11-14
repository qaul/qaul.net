# qaul-cli Usage

The `qaul-cli` command line program was written for testing purposes.
It is easy to extend for further test cases.
You can find the documentation of all the possible cli commands in the README.md file of the qaul-cli client
[rust/clients/cli](https://github.com/qaul/qaul.net/tree/main/rust/clients/cli)

Following are some simple recipes on how to use it.

## Start qaul-cli and Create a User Account

qaul-cli creates all configuration files, the data bases and all
further files in the directory from which the program is invoked.
It is therefore necessary to create an own folder for each qaul-cli
instance to be invoked.

Let's start qaul-cli and create a user account for Alice from your terminal.

```
# move into the repository's rust directory
cd rust/clients

# create a folder for alice
mkdir alice

# move into the newly created folder
cd alice

# start the qaul-cli program
cargo run --bin=qaul-cli

# create a new user account for Alice
account create Alice

# to stop the program press `ctlr` + 'c'
```

After you run this commands, you can find all create files in the folder `rust/clients/alice`.

## Communicate in a One-on-One chat with another User

Each chat conversation in qaul happens in a specific chat group.
Every chat group has a unique UUID.
Chat groups for direct conversations are set up automatically in qaul
and have preset group IDs.

Here is how you chat between two users: `Alice` and `Bob`.
Start two qaul-cli instances from two different terminals in two different
folders, one for alice and one for bob.

The users discover each other automatically within the same network
and exchange their user identity information.

In order to see the users IDs and group type in the command:

`users list`

This will display all users present. And for each user the preset group ID
which is used for the one-on-one chat with that user is displayed.

This is the example user list for Alice:

```
All known Users
No. | User Name | User Id | Veryfied | Blocked | Connectivity
    | Group ID | Public Key
1 | Alice | "12D3KooWAih9wpC3xKkkJvgXhUKsrUJpzmZvthGLu4NnrSGD6sJ5" | N | N | Online
   | 0d67ab7c-c120-e7af-0d67-ab7cc120e7af | uKzCgcLmRVFu3B74saY5gDzWSXHs1BrwU1mUT7KCva9
  Connections: module | hc | rtt | via
      LOCAL | 0 | 0 | 12D3KooWAtZZKwQW4VweTs6CQPGnZsAaEwynursSM695Kg4adtwx
2 | Bob | "12D3KooWLv63ptZbP7LxnAqM8iHpZf1MCNJKYwsDnYraYAV8nPyC" | N | N | Online
   | 0d67ab7c-c120-e7af-a4e7-d09eb699cea9 | C6it5kytCD5UNHKvW7YUnrvWi3FgXFnjpxVZABLEtTFG
  Connections: module | hc | rtt | via
      LAN | 1 | 652 | 12D3KooWMttX5JCkvmeA8u7jpJDRg5VxhRTqaQc9MBJKYeU26iSo

```

The group ID for a chat with Bob is `0d67ab7c-c120-e7af-a4e7-d09eb699cea9`.

Alice starts a chat with Bob and writes him a chat message saying "hi 👋":

```
# chat send {Group ID} {Chat Message}
chat send 0d67ab7c-c120-e7af-a4e7-d09eb699cea9 hi 👋
```

Bob sends a chat message back:

`chat send 0d67ab7c-c120-e7af-a4e7-d09eb699cea9 Hello Alice how are you doing?`

Display the conversation via the following command:

`chat conversation 0d67ab7c-c120-e7af-a4e7-d09eb699cea9`

## Create a Chat Group for multiple Users and Invite another User

You can create chat groups for multiple users manually.
The process is then a bit more complex, as requires the following steps:

1) Create a group with a group name.
2) Invite a user into the group.
3) The invited users then can accept or deny the invitation.

Here is how Alice creates a chat group for her Friends and invites Bob into it.

Alice creates the new group "Friends"

`group create Friends`

This creates the group and displays the newly created group ID:

```
group create Friends
====================================
Group was created or updated
	id: b430ec23-094e-41f5-a02c-539fc6df7140
```

Alice now invites Bob into the group (Bob's user ID is `12D3KooWLv63ptZbP7LxnAqM8iHpZf1MCNJKYwsDnYraYAV8nPyC`):

```
# group invite {Group ID} {User ID}
group invite b430ec23-094e-41f5-a02c-539fc6df7140 12D3KooWLv63ptZbP7LxnAqM8iHpZf1MCNJKYwsDnYraYAV8nPyC
```

Bob displays the received invitation via the following command:

`group invites`

Bob can now accept or decline the invitation into the chat group. He accepts of course:

```
# group accept {Group ID}
group accept b430ec23-094e-41f5-a02c-539fc6df7140
```

He writes a message into the group:

`chat send b430ec23-094e-41f5-a02c-539fc6df7140 hello Friends!`

Display the groups chat conversation:

`chat conversation b430ec23-094e-41f5-a02c-539fc6df7140`
