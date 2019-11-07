# Technical Documentation

Welcome to the qaul.net technical documentation!

This section is aimed at two different types of people:

1. People wanting to contribute to `qaul.net` and it's libraries
2. People wanting to write network services using `libqaul` and `R.A.T.M.A.N.`

This page will offer a short introduction to the structure of the
ecosystem which should be relevant to both groups 😉

## Introduction

Fundamentally, `qaul.net` is a distributed system. As such, it deals
with a lot of moving parts, spread over a network and not accountable
to a single set of rules. The core component of `qaul.net`, titled
`libqaul`, is a library that aims to make the creation of distributed
systems, functioning independent of fixed infrastructure or internet
backbones, as easy as possible.

It is what `qaul.net`, and the core services it provides, is built
on. In a lot of the design documents, the term `node` is used. Being a
distributed system, one might think that the term refers to a computer
or device on the network. This is a false assumption.

A `node` in the sense of `qaul.net` is a user, i.e. an entity on the
network with a cryptographic identity.  This is because multiple users
could be operating on the network via the same physical hardware,
which should not be exposed to anyone _outside_ of this hardware.

When referring to a physical machine we will use the term _"device"_!

## API Overview

`libqaul` provides a "Service API", which exposes core components of
an application workflow, giving developers a glimpse into a wider
network of devices. An external program (called a "service") might use
the `send_message` function provided by `libqaul` to implement group
chats or announcement channels. `qaul.net` (the app you install) comes
with four services by default: `messaging`, `filesharing`, `voices`,
and `prefs`. Each of them is, in the sense of `libqaul`, an external
application, that happens to get access to some internal state.

Anything these services do, you can do in your own application!

The "low level" service API consists of the following basic
capabilities:

- Creating users, logging in, verifying existing tokens
- Managing a user's contact book (adding, deleting, setting trust
  levels)
- Read information about the local keystore (what keys are available)
- Sending messages to one, multiple or all users on a network
- Managing local files, stored, captured and downloaded by `libqaul`
- Get the current network state and configure network backends
- Send an interrupt with payload to another local service

To use `libqaul`, simple bundle it with your application and write
your app against it's API.

If a user is installing your app on a system that already has some
`libqaul`-enabled application running on it (say `qaul.net`, it is
possible for it to connect to the existing instance, sharing it's
networking and user concepts. In this case, your service can even send
messages to other services, to use their functionality.

The following graphic demonstrates the concept.

![](/assets/apis.svg)

**Note:** In this (qaul.net internal) setup the `"core"` service is
          special.  It acts more like an API shim than an actual
          service and simply exposes the `libqaul` service API as an
          HTTP interface that is easier for the qaul.net UI to
          consume.

## libqaul

The insides of `libqaul` consist of a few smaller (and larger)
libraries.  When we consider their design, we assign them "layers" for
easy classification.

| Component | Description
|----------------------|------------------------------
| [Routing Layer] | Handles network abstractions, frame routing and provides an API to interact with incoming messages
| [Service Layer] | Provides persistent storage for users, services, cryptographic identities, trust and management functions
| [API Layer] | A few very slim API-shims that either map to HTTP or platform native IPC mechanisms (i.e. Android intents or UNIX sockets)
| [Application Layer] | An extenal network service which utilises the service API to provide different features to an application
| [Graphics layer] | Displaying it all to a user, probably the entry-point to the end-user application

[Routing Layer]: /technical/routing/_intro.html
[Service Layer]: /technical/libqaul/_intro.html
[API Layer]: /technical/api/_intro.html
[Application Layer]: /technical/services/_intro.html
[Graphics layer]: /technical/webgui/_intro.html

As the author of a network service utilising `libqaul` you don't need
to understand all that much about it's internal design.  If you are
interested in contributing to it, check the
[libqaul](/technical/libqaul/_intro.html) chapter!
