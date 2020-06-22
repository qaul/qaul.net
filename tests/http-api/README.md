# qaul.net WebGUI HTTP-API Interface Test Scripts

The scripts in this folder can be used to manually test the 
qaul.net http-api interface that delivers the data to the GUI.

The http-api is documented in the http-api guide: 
https://docs.qaul.net/http-api/http-api/_intro.html


## Structure of the Tests

Every function has it's own test script in the `src` sub-folder. The script is named after the function and can be used as a reference to see how the function is used. In order for these scripts to work they need to be called with the correct command line values.

In order to use the tests as easily as possible all the scripts in the main folder are high level scripts that call the function scripts in the sub folder with the correct input values.


## Prerequisites

To be able to run the test scripts you need to have the following programs installed:

* [HTTPie](https://httpie.org/): This program is a user friendly `curl` replacement. You can find more information here: https://httpie.org/
* jp: a CLI JSON parser


## Usage

The scripts expect a locally running qaul.net daemon. In order to be able to test the sending as well as the receiving of messages, we use the `multinode-test` binary. 
The `multinode-test` set's up several qaul.net instances locally that communicate directly with each other. In this set up the sending as well as the receiving of messages can be tested. The instances can be accessed via different port numbers.
Instance **A** listens on port `9900` on localhost. Instance **B** listens on port `9901`.


First you need to start the qaul.net daemon:

```bash
cargo run --bin multinode-test webgui/dist
```


Use the scripts:

```bash
# test all user functions
./user-crud.sh

# test authentication functions
./authentication.sh

## Chat Service:
# test chat room functions
./chat-room-crud.sh

# send a chat message
./chat-message-create.sh
```
