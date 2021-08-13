# qaul-cli

**qaul CLI client**

This client can:

* discover other nodes in the same network via mdns
* connect to remote nodes via the internet overlay network.
* list the network structure & display router state
* create user accounts
* publish, distribute and display feed-messages via floodsub protocol to all other nodes in the network.


## Run qaul-cli

Start at least two instances of this program. Either on different machines or start from different folders on the same machine.

You can run as many instances on as many machines as you like. the machines just need to be in the same network, or interconnected via the Internet overlay network.


**Start Program**

```sh
# start the program
RUST_LOG=info cargo run
```

Once the program is running, one can enter the commands documented in the CLI Manual below.


## CLI Commands when the Program is Running

The following commands are available:

* user accounts
  * `user list` - list all local user accounts
  * `user create {User Name}` - create a new user account with the name {User Name}
* feed service
  * `feed send {FeedMessage}` - sends the {FeedMessage} to the network and distributes it to all connected nodes.
    * the message is signed and can be validated.
    * at least one user needs to be created.
* router info
  * `router table list` - display routing table for all currently reachable users
  * `router users list` - display all users known to this router
  * `router neighbours list` - display all neighbour nodes per interface
  * `router connections list` - display connection discovery table
    * out of this information the routing table is generated
  * `router info list` - display scheduler info
    * Each neighbour receives routing info updates.
      This list displays the scheduler information.
* connection modules info
  * `modules info` - display information of the connections modules
    * list all neighbour nodes a module has a running tcp connection.


## Further Configuration

On the first startup of the program, a configuration file `config.toml` 
is generated in the local folder. 
You can configure your node by editing it.


### Connect to Nodes in the Internet

You can run nodes online, to interconnect local networks and create an internet overlay network.

To connect to one, you need to add it's address to the peers list of the `[internet]`  module in the configuration file `config.toml`. For example, for the IP address `144.91.74.192` on port `9229` it looks like this:

```toml
[internet]
peers = ["/ip4/144.91.74.192/tcp/9229"]
```
