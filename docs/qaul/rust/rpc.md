# RPC - Remote Process Communication - Libqaul to System & GUI Communication

Via the RPC system libqaul can be controlled from an application and a UI.

Libqaul uses **protobuf** format to define, encode, decode and exchange binary 
messages between libqaul and the UI applications.

<https://developers.google.com/protocol-buffers>


## Protocol Definitions

The RPC messages are defined in Protobuf `.proto` files in their modules in libqaul.

### UI Communication Messages

The UI communication has the prefix `qaul.rpc`.
The Files can be found here:

* RPC meta package `qaul.rpc`: `rust/libqaul/src/rpc/qaul_rpc.proto`
* Node information `qaul.rpc.node`: `rust/libqaul/src/node/node.proto`
* User Accounts functions `qaul.rpc.user_accounts`: `rust/libqaul/src/node/user_accounts.proto`
* Users functions `qaul.rpc.users`: `rust/libqaul/src/router/users.proto`
* Router functions `qaul.rpc.router`: `rust/libqaul/src/router/router.proto`
* Feed Service `qaul.rpc.feed`: `rust/libqaul/src/services/feed/feed.proto`
* Connections module information `qaul.rpc.connections`: `rust/libqaul/src/connections/connections.proto`

### System Communication

The communication with the OS platform modules.
The system communication has the prefix `qaul.sys`.
The files can be found here:

* BLE communication `qaul.sys.ble`: `rust/libqaul/src/connections/ble/manager/ble.proto`

## RPC package Definitions For Different Programming Languages

The `.proto` files can automatically be translated into most programming language.
All currently used languages by qaul are already pre-created.
You can find them in the folder `rust/libqaul/src/rpc/generated`.

* cpp
* java
* kotlin
* rust
* swift
* dart (is already copied into the flutter folder `qaul_ui/lib/rpc/generated`)

## Build the Files

In order to build the source files for the programming languages you can use the execute the following

* Rust files are automatically builded when libqaul is built via cargo.
* Shell scripts to build files for Java, Kotlin & Dart can be found `rust/libqaul/src/rpc/build_scripts`.


## RPC Protobuf UI Communication

`QaulRpc` meta message

* All messages of the modules are put as binary data into the `data` field of `QaulRpc` message.
* The enum field `module` declares to which module the message belongs.

`Node` module messages

* Get information about the node (e.g. the node ID)
  * Send node information request: set `get_node_info` field to true
  * Receive node information: `NodeInformation` message

`UserAccounts` module messages

* Check if a default user account has been created & receive it
  * Send request: set `get_default_user_account` field to true
  * Receive `DefaultUserAccount` message
* Create a default user account
  * Send `CreateUserAccount` message

`Users` module messages

* Get the list of all known users
  * Send user list request: send `UserRequest` message
  * Receive `UserList` message

`Router` module messages

* Get the routing table list
* Get neighbours nodes list
* Get connections list

`Feed` module messages

* Receive feed message list
  * Send `FeedMessageRequest` (the last received message can be set)
  * Receive `FeedMessageList`
* Send feed message
  * Send `SendMessage`

`Connections` modules messages

* Internet module peer nodes
  * Get peer nodes list
  * Add a peer node
  * Remove a peer node