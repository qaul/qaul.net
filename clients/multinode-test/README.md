# Local Multinode Test

This test set's up a local 3-node qaul.net test network with two 
WebGUI interface servers.

The network looks as follows: 

```NoRun
Node A <-> Middle Node <-> Node B
```

## Start Test

To start the test invoike the program from the shell with the relative path to the built Ember UI

```bash
# General Example
cargo run --bin local-multinode-test <local-path-to-webgui>

# If you invoke it from the top level project folder
# and you built EmberJS webGUI before you can do
cargo run --bin local-multinode-test webgui/dist
```

Then open the user interface of the two nodes in your web browser:

* Node A: http://127.0.0.1:9900
* Node B: http://127.0.0.1:9901
