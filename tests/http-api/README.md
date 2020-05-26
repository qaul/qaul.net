# qaul.net WebGUI HTTP-API Interface test scripts

The scripts in this folder can be used to manually test the 
qaul.net http-api interface that delivers the data to the GUI.

## Prerequisites

To be able to run the test scripts you need to have the following programs installed:

* [HTTPie](https://httpie.org/): This program is a user friendly `curl` replacement. You can find more information here: https://httpie.org/
* jp: a CLI JSON parser

## Usage


```bash
# test all user functions
./users-crud.sh

# test authentication functions
./authentication.sh

## Chat Service:
# test chat-rooms functions
./chat-rooms-crud.sh

# send a chat-message
./chat-messages-create.sh
```
