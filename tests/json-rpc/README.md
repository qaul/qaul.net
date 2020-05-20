# JSON RPC Communication with qaul-http-RPC Interface

The scripts in this folder can be used to 

## Usage

Make sure you have a test server running for these tests.  While in
the cargo workspace you can just run `cargo run --bin multinode-test
<PATH>`, where `PATH` is the path to the assembled webgui (FIXME:
currently that's not the case - it will accept any path because the
webgui isn't used).

Note: if you want to run multiple tests in succession manually, make
sure to do it in the same shell.  Some of them export env variables
for other tests, which will break if you run them in separate shell
instances.

## How to write a test

Each test should setup as much of the context as it needs.  If a test
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

#### Set User Name

```bash
./users_modify.sh
```

#### Get a List of all Users

```bash
./users_list.sh
```

### Chat-Rooms

#### Create a Chat-Room

Creates a chat room

```bash
./chat-rooms-create.sh
```

#### Send a Message

Creates a chat room and sends a message into it

```bash
./chat-messages-create.sh
```

#### List all Chat-Rooms

This will create a chat room, send a message and then list it's data back.

```bash
./chat-rooms-list.sh
```

