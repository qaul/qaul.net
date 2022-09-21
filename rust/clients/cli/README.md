# qaul-cli

**qaul RPC CLI client**

This CLI client starts libqaul in an own thread, and uses the protobuf RPC interface to communicate with libqaul.

## Run qaul-cli

Start at least two instances of this program. Either on different machines or start from different folders on the same machine.

You can run as many instances on as many machines as you like. the machines just need to be in the same network, or interconnected via the Internet overlay network.

### Start Program

```sh
# start the program
cargo run --bin=qaul-cli
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
  * `users online` - display all online users known to this router
  * `users verify {User ID}` - verify user with {User ID}
  * `users block {User ID}` - block user with {User ID}
  * `users secure {User ID}` - get the security number for a specific user  
* router
  * `router table list` - request and display routing table with per module connectivity per user.
  * `router neighbours list` - request and display neighbours list of all neighbouring nodes.
  * `router connections list` - request and display connections table, with all known connections per connection module.
* connections
  * `connections nodes list` - request a list of all statically configured peering nodes via the internet.
  * `connections nodes add {Multiaddress}` - add a new internet peering node, via it's multiaddress, e.g. `/ip4/144.91.74.192/tcp/9229`
  * `connections nodes remove {Multiaddress}` - remove an internet peering node.
  * `connections nodes active {Multiaddress}` - active an internet peering node.
  * `connections nodes deactive {Multiaddress}` - deactive an internet peering node.
* feed
  * `feed send {FeedMessage}` - sends the {FeedMessage} to the network and distributes it to all connected nodes
    * the message is signed and can be validated
    * at least one user needs to be created
  * `feed list` - displays all feed messages
    * `feed list {Feed Message ID}` - displays only feed messages newer than {Feed Message ID}
* group
  * `group create {Group Name}` - creates a new group
  * `group list` - list all available groups
  * `group info {Group ID}` - shows the group information
  * `group invite {Group ID} {User ID}` - invite a user to a group
    * `group invited` - list received pending invitations
    * `group accept {Group ID}` - accept group invitation
    * `group decline {Group ID}` - decline group invitation
  * `group remove {Group ID} {User ID}` - remove a group member from the group
  * `group rename {Group ID} {New Name}` - rename a group
* chat
  * `chat send {Group ID} {Chat Message}` - sends the {Chat Message} to the user with the ID {Group ID}
  * `chat conversation {Group ID}` - displays all messages of the conversation with the ID {Group ID}
* file sharing
  * `file send {Group ID} {File Path} {File Description}` - sends a file to the user with the ID {Group ID} and a {File Description} text.
  * `file history [{offset} {limit}]` - displays a paginated file history
    * The page {offset} and {limit} values are optional. The default values are an offset of 0 and 10 results.
* DTN - Delay Tolerant Networking
  * `dtn state` - display the state of the local DTN storage. Displays the number of messages and the used bytes.
  * `dtn config` - displays the DTN configuration: Max storage size & storage users
  * `dtn add {user ID}` - add a storage user to the DTN list
  * `dtn remove {user ID}` - remove a storage user
  * `dtn size {size in MB}` - set the maximal total storage size in mega bytes
* debug
  * all these commands are for debugging purposes only
  * `debug rpc sent` - displays the number of RPC messages sent to libqaul
  * `debug rpc queued` - displays the number of messages in the RPC queue to be processed.
    * This command will probably always return '0', as the messages are checked many times per second by this client.
  * `debug panic` - sends a debug message to libqaul that let's libqaul panic. This function is for testing the crash logger on flutter.
  * `debug heartbeat` - sends a heartbeat request message to libqaul, which is answered with a returning heartbeat message.
  * `debug log enable` - enable libqaul logging to file.
  * `debug log disable` - disable libqaul logging to file.
  * `debug path` - request the storage path from libqaul. This returns a path string with the location where all qaul related data is stored (configuration, databases, logs).
