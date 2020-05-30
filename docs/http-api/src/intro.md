# UI API Documentation

**There are several ways to communicate with the qaul.net daemon. They all have the same functionality and communicate via JSON over http. If you want to create a UI client for qaul.net this guide documents how to interact with the daemon.**


## [JSON-RPC API]

The [JSON-RPC API] uses a single http endpoint for the communication with qaul daemon. It is our main API for UI's.

[Read more >>](./json-rpc/_intro.md)


## [HTTP-API]

The [HTTP-API] is the main entry point for the Web-GUI. 
It uses the http protocol for the communcation and communicates with the EmberJS REST module.

[Read more >>](./http-api/_intro.md)


[JSON-RPC API]: ./json-rpc/_intro.md
[HTTP-API]: ./http-api/_intro.md
