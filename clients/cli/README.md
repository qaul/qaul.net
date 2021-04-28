# qaul-cli

**qaul CLI client**

This client can: 
* discover other peers in the same network
* distribute and display feed-messages via mdns to all other peers in the network
* create, save and distribute information pages

## Run this Example

Start two the same machine, from within the folders `node1` & `node2`. The pages are stored in the file `pages.json`.

**Start Node 1**

```sh
# move into the node1 one folder
cd node1
# start the program
RUST_LOG=info cargo run
```

Once the program is running, one can enter the commands documented in the CLI Manual below.


**Start Node 2**

```sh
cd node2
RUST_LOG=info cargo run
```

**More Nodes**

You can run as many instances on as many machines as you like. the machines just need to be in the same network.


## CLI Commands when the Program is Running

There are several commands:

* qaul network
  * `q ls` - list all peers
* feed service
  * `f {FeedMessage}` - sends the {FeedMessage} into the network and displays it on all nodes
* page service
  * `p ls` - list local pages
  * `p ls all` - list all public pages from all known peers
  * `p ls p {peerId}` - list all public pages from the given peer
  * `p create Title|Description|Content` - create a new page with the given data, use the pipe symbol `|` to separate the fields
  * `p publish {pageId}` - publish the page with the given page ID
