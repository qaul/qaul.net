# netmod-udp

The following crate provides a netmod endpoint, overlayed via udp.
Network discovery features are implemented via broadcast addresses,
and a specias UDP handshake packet.

This crate also handles the NAT required to go from a ratman routing
ID, to a local IP address.  It does however not implement IP range
discovery.  See libqaul-proxy for that.

