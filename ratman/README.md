# Ratman

A modular router userspace frame router, implementing distance vector
routing, and desay tolerance.  It's basic design is inspired by
BATMAN, and it's written entirely in Rust (hence the name).

## Usage

To use Ratman, you need to create a Router.  This type exposes an
async API to interact with various network abstractions.  A networking
endpoint, as well as the basic datagram types are defined in the
ratman-netmod crate, and the routing identities are defined in the
ratman-identity crate.


## Netmod architecture

Because the router exists entirely in userspace, decoupled from any
kernel networking layer.  This means that the router is responsible
for also sending any payload to the appropriate driver.  A driver is
some binding to send data, via a channel, such as UDP, or in-memory
channel interfaces.


## Clocking

Generally, Ratman handles scheduling and internal clocking for you.
There's no need to call update functions to make poll's work, or to
actually dispatch messages.  During initialisation the constructor
spawns several long running tasks, that deal with various tasks in the
router stack in a loop.  The downside to this is that the user of the
library has no control over how this code is called.

This is where the Router API adds clock points, and the `clock`
submodule.  The idea is that an external program (i.e. you!)  can use
Barriers to set clock points for various tasks, that will internally
wait for the external clock intput.  This way it is possible to reduce
the data rate in low power mode, without having to teach Ratman what
this means.

It also means that you can manually clock step the router during tests
to check various states and invariants in the tests.

Check the documentation for the `Clockwork` type for more details.
