# Technical Documentation

Welcome to the qaul.net technical documentation!

This section is aimed at two different types of people:

1. People wanting to contribute to qaul.net and it's libraries
2. People wanting to write their own apps for a qaul network

This page will offer a short introduction to the project structure to
get you started.  On each section in this document there are pages and
chapters that go into more detail than is required here.


## Introduction

Fundamentally, qaul.net is a highly distributed system.  As such, it
is not accountable to a single set of rules across all devices that
are part of this system.  It is important to make the distinction
between "qaul.net", the application, "qaul network", the distributed
network of devices, and other components provided by the project that
can be used to write decentralised applications.

The primary component in this ecosystem is called libqaul.  It
provides an abstraction layer for a distributed network of devices,
the users that interact with each other and the messages and data that
can be exchanged.  The API of this library is called the "service API"
in other parts of the docs.

Built on top of it are services (or apps), that provide more specific
functionality, such as text messaging or file sharing.  qaul.net (the
app) is merely a GUI and collection of a few different services all
running on the same libqaul instance on one device.


## Layers

The following sections will outline the different layers present in
qaul.net (the application), then there are pages for more details on
how to interact with each of these layers.


### Services & apps

As mentioned in the introduction it is possible to build applications
(or "services") with libqaul, that can interact with each other on a
distributed network.  These services provide high level functionality
such as sending text messages, a collaborative public feed (similar to
mastodon), and voice calls.

Following is a list of all the services that come bundled in qaul.net
by default (more details [here][services]).

- feed
- messages
- files
- voices
- radio

[services]: ./qaul.net/services.html

The reason why qaul.net is built in such a modular way is to allow you
to write your own services, with their own functionality and UI, that
gets to interact with an existing service and user ecosystem.  libqaul
also helps you to build your application in a way where it doesn't
rely on central servers on a network and keeping your users' data
safe.


### libqaul

#### Service API

The interface to libqaul is called the service API, and a versatile
abstraction over a decentralised network.  It handles local user
authentication, network user discovery, binary payload messages,
contact data, and even encrypted file storage at rest.

The API itself is available as an async Rust library and ffi C
interface, with some optional IPC add-ons:

- [http/json:api] - this is how the qaul.net GUI is hooked up
- [socket-ipc] - using a cap'n proto ipc protocol over unix sockets
- [android-ipc] - implementing an Android specific ipc interface

[http/json:api]: https://docs.qaul.net/http-api/
[socket-ipc]: ./libqaul/ipc/socket.html
[android-ipc]: ./libqaul/ipc/android.html

The idea behind the variety of IPC interfaces is that your application
can bundle it's own copy of libqaul, to provide the network backends
required to join a mesh network, however it can also connect with a
running instance if one is available, seeing as only one daemon can
concurrently have access to the networking backends.

This way resources, user profiles and social graphs can be shared.
Furthermore, it's possible for your application to use services that
are bundled in with a qaul.net application stack, meaning that there's
a lot of functionality that you don't have to replicate in your own
apps.


#### Internals

The internals of libqaul are intirely written in Rust, and hook into
various storage and networking abstractions.  libqaul primarily uses
two libraries, also written as part of this project, to do it's job:
alexandria, and ratman.

### ratman
