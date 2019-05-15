# Technical Documentation

This is meant for people who want to write some code.

Fundamentally the qaul.net re-write features a few very well defined abstraction layers
to separate functionality and make things easier to maintain.

Check the illustration below for a broad overview.

![](./technical/overview.jpg)

##### Note

In a lot of the design documents, the term `node` will be used.
This being a distributed networking application,
one might think that the term refers to a computer or device on the network.
This is a false assumption.

A `node` in the sense of `qaul.net` is a user,
i.e. an entity on the network with a cryptographic identity.
This is because multiple users could be operating on the network via the same physical hardware,
which should not be exposed to anyone _outside_ of this hardware.

## Design layers

Following is a table listing the different layers in qaul.net and linking to their design docs.

| Component | Description |
|---------------------|------------------------------------|
|    Routing Layer    | Handles network abstractions, frame routing and provides an API to interact with incoming messages (`routing-core`, `net-links` and `net-persistence`) |
| Service Layer | Provides persistent storage for users, messages, cryptographic identities, handles trust and verification and network services such as sending messages, making calls and sharing files |
| API layer | A slim layer on top of the qaul.net service library to expose all of it's functionality to non-Rust code consumers (http, ...?) |
| GUI layer | A cross-platform GUI to interact with all of the services on various devices and platforms |