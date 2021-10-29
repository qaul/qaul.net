# qaul-rpc-cli

**qaul RPC CLI client**

This CLI client starts libqaul in an own thread, and uses the protobuf RPC interface to communicate with libqaul.


## Run qaul-rpc-cli

Start at least two instances of this program. Either on different machines or start from different folders on the same machine.

You can run as many instances on as many machines as you like. the machines just need to be in the same network, or interconnected via the Internet overlay network.


**Start Program**

```sh
# start the program
cargo run --bin qaul-rpc-cli
```

Once the program is running, one can enter the commands documented in the CLI Manual below.


## CLI Commands when the Program is Running

The following commands are available:

* node
  * `node info` - prints the local node id
* user accounts
  * `account default` - get's and displays the default user account
  * `account create {User Name}` - create a new user account with the name {User Name}
* users - Functions for all users known by your node
  * `users list` - display all users known to this router
  * `users verify {User ID}` - verify user with {User ID}
  * `users block {User ID}` - block user with {User ID}
* router
  * `router table list` - request and display routing table with per module connectivity per user.
  * `router neighbours list` - request and display neighbours list of all neighbouring nodes.
  * `router connections list` - request and display connections table, with all known connections per connection module.
* connections
  * `connections nodes list` - request a list of all statically configured peering nodes via the internet.
  * `connections nodes add {Multiaddress}` - add a new internet peering node, via it's multiaddress, e.g. `/ip4/144.91.74.192/tcp/9229`
  * `connections nodes remove {Multiaddress}` - remove an internet peering node.
* feed
  * `feed send {FeedMessage}` - sends the {FeedMessage} to the network and distributes it to all connected nodes
    * the message is signed and can be validated
    * at least one user needs to be created
  * `feed list` - displays all feed messages
* debug
  * all these commands are for debugging purposes only
  * `debug rpc sent` - displays the number of RPC messages sent to libqaul
  * `debug rpc queued` - displays the number of messages in the RPC queue to be processed.
    * This command will probably always return '0', as the messages are checked many times per second by this client.
