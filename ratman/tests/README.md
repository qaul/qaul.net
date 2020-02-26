# Ratman tests

This directory contains tests for the Ratman userspace router.  Some
send static data, while others can pull in larger test frameworks to
generate data to send.

When you write a test, please add it to the list below, with a short
description.

- [announce](./announce.rs) a test with three static nodes, sending
  announcements.
- [very_simple_chat](./very_simple_chat.rs) an example of how to send
  messages with payloads via Ratman
