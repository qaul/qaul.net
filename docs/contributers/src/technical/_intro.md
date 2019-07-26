# Technical Documentation

Welcome to the qaul.net technical documentation!

This section is aimed at two different types of people:

1. People wanting to contribute to `qaul.net` and it's libraries
2. People wanting to write network services using `libqaul` and `R.A.T.M.A.N.`

This page will offer a short introduction to the structure of the ecosystem
which should be relevant to both groups 😉

##### Note

In a lot of the design documents, the term `node` will be used.
This being a distributed networking application,
one might think that the term refers to a computer or device on the network.
This is a false assumption.

A `node` in the sense of `qaul.net` is a user,
i.e. an entity on the network with a cryptographic identity.
This is because multiple users could be operating on the network via the same physical hardware,
which should not be exposed to anyone _outside_ of this hardware.

When referring to a physical machine we will use the term _"device"_!

## API Overview

At the heart of `qaul.net` sits `libqaul`, which in turn consists of multiple smaller libraries.
`libqaul` provides a service API that allows an external program (called "services") to interact with a qaul network.
This service can run as an external process.

Provided by `libqaul` are several "core" services that will be useful to any application using `libqaul`.

- `messaging`: provides simple text messaging capabilities
- `files`: provides simple file-sharing capabilities

More "low level" functions are available via the main service API:

- Creating users or logging in
- Managing a user's contact book
- Sending messages with arbitrary payloads
- Getting the state of the network
- (In case a user is an admin) Configuring network backends
- etc.

This means that you can simply bundle `libqaul` into your application.
If your app is running on a system that already has `qaul.net` (or any other `libqaul` enabled service!)
installed, it will instead connect to the existing backend.
This means that many `libqaul` enabled applications can share the same network and users.

The following graphic demonstrates the concept.

![](/assets/apis.svg)

**Note:** In this (qaul.net internal) setup the `"core"` service is special.
          It acts more like an API shim than an actual service and simply exposes the `libqaul` service API
          as an HTTP interface that is easier for the qaul.net UI to consume.

## libqaul

The insides of `libqaul` consist of a few smaller (and larger) libraries.
When we consider their design, we assign them "layers" for easy classification.

| Component            | Description
|----------------------|------------------------------
| [Routing Layer]      | Handles network abstractions, frame routing and provides an API to interact with incoming messages
| [Service Layer]      | Provides persistent storage for users, services, cryptographic identities, trust and management functions
| [API Layer]            | A few very slim API-shims that either map to HTTP or platform native IPC mechanisms (i.e. Android intents or UNIX sockets)
| [Application Layer]  | An extenal network service which utilises the service API to provide different features to an application
| [Graphics layer]     | Displaying it all to a user, probably the entry-point to the end-user application

[Routing Layer]: /technical/routing/_intro.html
[Service Layer]: /technical/libqaul/_intro.html
[API Layer]: /technical/api/_intro.html
[Application Layer]: /technical/services/_intro.html
[Graphics layer]: /technical/webgui/_intro.html

As the author of a network service utilising `libqaul` you don't need to understand all that much about it's internal design.
If you are interested in contributing to it, check the [libqaul](/technical/libqaul/_intro.html) chapter!
