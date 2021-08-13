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
* router
  * `router users list` - display all users known to this router
* feed
  * `feed send {FeedMessage}` - sends the {FeedMessage} to the network and distributes it to all connected nodes
    * the message is signed and can be validated
    * at least one user needs to be created
  * `feed list` - displays all feed messages
