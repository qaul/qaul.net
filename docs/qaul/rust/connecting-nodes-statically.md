# Connecting Nodes Statically

The Internet module of qaul can interconnect nodes statically over networks.
You can run for example a server in the Internet and interconnect qaul devices
over it, or you can interconnect them in your local network.

## Multi-Addresses

qaul uses so called multi-addresses. A multi-address is a string that contains all
all the information to connect to another node.
The multi-address of the qaul community hub is `/ip4/144.91.74.192/udp/9229/quic-v1`.
The meaning of this multi-address is the following `/{NETWORK_PROTOCOL}/{NETWORK_ADDRESS}/{TRANSPORT_PROTOCOL}/{NETWORK_PORT}/{QUIC_PROTOCOL}`

* NETWORK_PROTOCOL
  * the network protocol used
  * the node is connected via the IPv4
* NETWORK_ADDRESS
  * the network address of the node.
  * the node has the IP address `144.91.74.192`
* TRANSPORT_PROTOCOL
  * the transport protocol
  * the node connects via the tcp protocol
* NETWORK_PORT
  * the network port the node listens to
* QUIC_PROTOCOL
  * The QUIC protocol is based on UDP

A default configuration address of a qaul node is `/ip4/0.0.0.0/udp/0/quic-v1`.
The zeroed values mean, the node listens on every address and it chooses
a random free port where it starts to listen on.

## Configuring a Static Node

In order for the a node to be approachable by others you need to fix the network port to a specific number.
By default qaul nodes shall listen on port `9229`.

Your new internet configuration looks like this: `/ip4/0.0.0.0/udp/9229/quic-v1`

## Configuring the Client

Now you can enter the address to the node you want to connect to.
You need to know the network port and the network address and put that into the peers list of your configuration file.

[qaul Configuration file](qaul/rust/configuration.md)
