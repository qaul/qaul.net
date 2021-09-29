# RPC - Remote Process Communication - Libqaul to System & GUI Communication

Via the RPC system libqaul can be controlled from an application and a UI.

Libqaul uses **protobuf** format to define, encode, decode and exchange binary 
messages between libqaul and the UI applications.

<https://developers.google.com/protocol-buffers>


## Protocol Definitions

The RPC messages are defined in Protobuf `.proto` files in their modules in libqaul.

* RPC meta package `rust/libqaul/src/rpc/qaul_rpc.proto`
* Node information `rust/libqaul/src/node/node.proto`
* User Accounts functions `rust/libqaul/src/node/user_accounts.proto`
* Router functions `rust/libqaul/src/router/router.proto`
* Feed Service `rust/libqaul/src/services/feed/feed.proto`


## RPC package Definitions For Different Programming Languages

The `.proto` files can automatically be translated into most programming language.
All currently used languages by qaul.net are already pre-created.
You can find them in the folder `rust/libqaul/src/rpc/protobuf_generated`.


## Build the Files

In order to build the source files for the programming languages you can use the execute the following

* Rust files are automatically builded when libqaul is built via cargo.
* Shell scripts to build files for Java, Kotlin & Dart can be found `rust/libqaul/src/rpc/protobuf_build_scripts`.
