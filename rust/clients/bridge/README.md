# qaul-matrix-bridge

**qaul matrix bridge**

This CLI client starts libqaul in an own thread, and uses the protobuf RPC interface to commu icate with libqaul. Also in another separate thread, It starts a relay-bot file which interconnects qaul with matrix using matrix-sdk in rust and RuMa which stands for Rust Matrix.

## Start qaul-matrix-bridge

Go to Matrix and create a public room. Invite `@qaul-bot` to that room. Make sure the encryption is turned off.

```rust
cargo run --bin=qaul-matrix-bridge
``` 

This will initiate a login for the qaul-bot in the matrix and start a daemon thread for listening from matrix.

Send any message in the matrix room. You can see it coming to qaul communication via `feed list` command in qaul-cli

## Matrix Commands when the Program is Running

The following commands are available:

* `!qaul` : Gives an acknowledgement that the bot is working by responding.
