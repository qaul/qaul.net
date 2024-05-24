# Configuring qaul

The configuration of a node is saved into a configuration file `config.yaml`.
This file is read at startup.
On the first startup of qaul, the file is created.
Whenever a change is made in the program, it is written into the configuration file.

The following things are configured in the configuration file:

* Node ID & Node Keys
* User Accounts
  * User ID
  * User Keys
* LAN Connection Module
  * addresses and port to listen to
* Internet Connection Module
  * addresses and port to listen to

## Example Configuration

Following is an example configuration file with comments.

```yaml
---
# node configuration
node:
  # indicates whether this node has been initialized
  # after the first startup this value needs always to be 1
  initialized: 1
  # the node id
  id: 12D3KooWFppUW6GydZgQgvZkb4fWz7updgU3UNribotUhMzFHxG8
  # the public and private key of this node
  keys: kJEfgqsB64mqApq1fj+ENTXlMX8bR+g6naIs/6WvfChZQ9202gdz2FUvmaDyhwddiwi/HUv1UzQn2xkmLL6CKQ==

# LAN Connection Module Configuration
lan:
  active: true
  # the multi address to configure the listening address 
  # and port of the LAN module
  # 0.0.0.0 - the module is listening on all addresses.
  # 0 - the port number 0 zero means the lan module chooses a random free port
  listen: [/ip4/0.0.0.0/udp/0/quic-v1, /ip6/::/udp/0/quic-v1]

# Internet Connection Module Configuration
internet:
  active: true
  # a list of all nodes the Internet module connects to.
  peers:
    # connect to node on the IP address 144.91.74.192
    # on port 9229, via a UDP/quic connection
    - /ip4/144.91.74.192/udp/9229/quic-v1
  do_listen: false
  # multi address configuring the port the internet module listens on
  # 0.0.0.0 - the module is listening on all addresses.
  # 9229 - the port number 9229 zero means the module listens on port 9229 for incoming connections
  listen: [/ip4/0.0.0.0/udp/9229/quic-v1, /ip6/::/udp/9229/quic-v1]

# User Accounts Configuration
# It contains a list with all the user accounts registered on this node
user_accounts:
  # first user account entry
  - name: Test User
    # user id
    id: 12D3KooWSRJX1aWUUJo82DaizXCivDN3mGQ69QR1yQqFNRU8UaEw
    # public and private key of the user
    keys: EMzXKCvOnOqjfKx+vwzaGOnPKKwhvu0nW4m4Nzx5nof2rjEAjU8u3vdD1yNo3j3FVg3qjV2VgiP3XkNo3Wz21A==
```

## Configuration File Location

The location of the configuration file depends on the system and the
binary you are running.

When running the rust CLI binaries the configuration file is created and
read from the working directory you are starting the program from.

The flutter applications have a specific location where they save and load the
configuration file from.
This location depends of the OS the app is running on and should be the default
location for configuration files.

* Linux
  * `~/.config/qaul`
* MacOS
  * `~/Library/Containers/net.qaul.qaulApp/Application Support/net.qaul.qaul`
* Windows
  * `C:\Users\USERNAME\AppData\Roaming\qaul\qaul\config`
* Android
  * `/data/user/0/net.qaul.qaul/files`
