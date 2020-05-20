# JSON-RPC Interface Tests

This folder contains tests for the entire RPC
interface with the goal of having automated tests 
for each RPC call.

Each Service has it's own test file.

The `harness.rs` file contains sets a local multi node
test environment based on the `ratman-harness` crate.
With it sending an recieving on interconnected nodes can be
tested.

You can run the tests as following:

```bash
# move to the rpc directory
cd libqaul/rpc

# run all tests in silence
cargo test

# run all tests and display debug messages
cargo test -- --nocapture

# run only the tests of the file users.rs
cargo test --test users -- --nocapture
```