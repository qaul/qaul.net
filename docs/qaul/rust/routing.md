# Qaul Routing Protocol

The qaul routing protocol is a distance vector routing protocol.
The distance vector is based on the calculated round trip time RTT per connection.

## Abbreviations Explanation

* HC - Hop Count
* RTT - Round Trip Time - measured in micro seconds
* LQ - Link Quality
* User ID - the qaul ID of a user
* Propagation ID - The increasing id of a propagation cycle of a route by it's host node.
* Host Node - the node where a user is hosted
* Neighbour Node - a node which is directly connected

## Communication Between Neighbours

Every five seconds each node sends a libp2p ping package to all it's neighbour nodes and measures the the round trip time RTT.

Every ten seconds each node sends the routing information to all it's neighbour nodes.

## User Entries

Each user has an entry in the routing propagation

* User ID
* Propagation ID
* RTT - Round Trip Time
* HC - Hop count

Propagation ID is an increasing number, that starts at 0 and is increased for every propagation cycle from the host node of the user. The propagation ID does not change over it's travel through the network. With it, we can decide, whether we have a new or an old information that is coming.

## Node Route Propagation

Each node propagates the routing information to all it's neighbour nodes every 10 seconds.

### Sending Routing Information

For all users where the node is the host node, it increases the propagation Id for every propagation cycle.
A propagation cycle duration is the time when all neighbours received the routing update.
At the moment this is every 10 seconds.

For all users where the node is the host node, information is:

* Propagation Id of the propagation cycle (increases by one every 10 seconds)
* Hop count = 0
* Round Trip Time = 0

### Receiving Routing Information

When a node receives the routing information it does the following for each user ID:

1) It checks whether the propagation ID is higher then the one stored for the user
   * If the propagation ID is lower then the current propagation ID, the incoming information is discarded.
   * If the propagation ID is equal to the current propagation ID, we check whether the following things:
     * did it arrive within 10 seconds since the first update?
     * if yes: add it to the connections list
   * If the propagation ID is newer then the current propagation ID, the ID is saved for this user entry and the information is added to the connections list
2) It calculates the link quality for this connection and adds it to the routing table.

### Link Quality - LQ

Out of the RTT and the HC, we calculate a link quality for each connection.
We calculate a penalty of 10 ms for each HC and add it to the RTT, this gives our new LQ value.

## Creating the Routing Table

Each second, a new routing table is generated out of the connection information.
When creating a new routing table the following steps are performed:

1) we loop through each connection table of each routing module separately.
2) We select for each user from each connection table the route that has the best LQ and write it into the routing table.
3) Old routing entries are deleted.

The routing table has therefore a routing entry for each connection module.

## Finding the best Route

Before sending a message to a user, the router searches the best route from it's routing table.
From all entries, it selects the best route according to the following system:

1) If there are entries from the LAN and/or the INTERNET modules present, it selects the route with the smallest RTT.
2) If only a BLE entry is present it selects the BLE route.
