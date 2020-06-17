# netmod-tcp

A tcp (layer 3) internet overlay for ratman networks.  Using
netmod-tcp requires a router daemon to be configured.  Check the
examples to see how.


## Static peers

A tcp-netmod endpoint can be configured to act as a static peer
bouncer, meaning that only pre-configured clients with static IP
addresses will be added to the session.  New handshakes will be
ignored.  This is useful when building general infrastructure that
shouldn't communicate with too many other peer machines.

Additionally, the endpoint can set the "DO_NOT_ADVERTISE" flag, which
means that other endpoints will not try to make it handshake with
other endpoints.  This is enabled by default and can be turned off for
debugging or profiling reasons.


## Dynamic handshakes

The alternative run-mode is "dynamic handshakes", which means that a
seed-list of peers can then introduce an endpoint to new endpoints if
it thinks they should know about each other (the parameters being: it
knowing both endpoints, neither setting "DO_NOT_ADVERTISE", and
non-trivial packet numbers flowing from one to the other).

## Current Testing Methods

Working using qaul-hubd rn:

`cargo run -p qaul-hubd -- --peers clients/hubd/peers.txt --port 9001` on one and
`cargo run -p qaul-hubd -- --peers clients/hubd/peers2.txt --port 9000` on the other.