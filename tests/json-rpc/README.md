# JSON RPC Communication with qaul-http-RPC Interface

The scripts in this folder can be used to test all the functions of the JSON RPC interface.

The JSON RPC interface is documented in the http-api guide: 
https://docs.qaul.net/http-api/json-rpc/_intro.html


## Structure of the Tests

Every function has it's own test script in the `src` sub-folder. The script is named after the function and can be used as a reference to see how the function is used. In order for these scripts to work they need to be called with the correct command line values.

In order to use the tests as easily as possible all the scripts in the main folder are high level scripts that call the function scripts in the sub folder with the correct input values.


## Prerequisites

To be able to run the test scripts you need to have the following programs installed:

* curl: a CLI tool for transferring data specified with URL syntax
* jp: a CLI JSON parser


## Usage

The scripts expect a locally running qaul.net daemon. In order to be able to test the sending as well as the receiving of messages, we use the `multinode-test` binary. 
The `multinode-test` set's up several qaul.net instances locally that communicate directly with each other. In this set up the sending as well as the receiving of messages can be tested. The instances can be accessed via different port numbers.
Instance **A** listens on port `9900` on localhost. Instance **B** listens on port `9901`.


First you need to start the qaul.net daemon:

```bash
cargo run --bin multinode-test webgui/dist
```


Run the high level test scripts:

```bash
# test all user functions
./user-crud.sh

## Chat Service:
# test chat room functions
./chat-room-crud.sh

# send a chat message
./chat-message-create.sh
```




## How to write a test

Each high level test should setup as much of the context as it needs.  If a test
is designed to run after a specific test, then it should call that
test.  Ultimately some code-paths will be tested more than once this
way.

Start your test by sourcing a required test, so for most basic
operations you would start by sourcing `bootstrap-users`.  If your
tests has a higher-order dependency (like a created chat room) source
that test instead.  The environment variables `A_{ID,TOKEN}` and
`B_{ID,TOKEN}` should be available to all tests.

```bash
source ./bootstrap-users.sh
```

Also: include at least `set -e` in any script to prevent it from
running after encountering an error to more accurately pinpoint a
problem.


### Per test docs

Following is a list of tests (please try to keep it up to date), with
comments and notes for tests that require them.

#### Users

This test will create a user, modify this user, list all users on the instance and delete the created user.

```bash
./users_crud.sh
```

### Chat Rooms

Only create a chat room

```bash
./chat-room-create.sh
```

Create a chat room, list all chat rooms for the authenticated user, modify the chat room and get the chat room.

```bash
./chat-room-crud.sh
```


#### Send a Message

Creates a chat room and sends a message from node A to node B, it the receives the message on node B.

```bash
./chat-message-create.sh
```

#### List all Chat Rooms

This will create a chat room, send a message and then list it's data back.

```bash
./chat-room-list.sh
```

