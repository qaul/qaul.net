# netmod-udp

The netmod-udp endpoint is the main endpoint for udp capable IP
networks, such as LAN-networks, existing Wifi networks, etc. .
Network discovery features are implemented via broadcast addresses,
and a special UDP handshake packet.

This crate also handles the NAT required to go from a ratman routing
ID, to a local IP address.  It does however not implement IP range
discovery.  See libqaul-proxy for that.
