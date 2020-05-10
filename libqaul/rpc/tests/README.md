# RPC Interface Tests

This folder contains tests for the entire RPC
interface with the goal of having automated tests 
for each RPC call.

Each Service has it's own test file.

The `tests.rs` file contains the testing environment
for all the tests. It initializes the RPC & libqaul 
stack for that the tests can send a JSON String to 
the RPC interface.

The `harness.rs` file contains sets a local multi node
test environment based on the `ratman-harness` crate.
With it sending an recieving on interconnected nodes can be
tested.
