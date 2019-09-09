# alexandria [![][ci-badge]][ci-url] [![][irc-badge]][irc-url]

[ci-badge]: https://git.open-communication.net/qaul/alexandria/badges/master/pipeline.svg
[ci-url]: https://git.open-communication.net/qaul/alexandria/commits/master
[irc-badge]: https://img.shields.io/badge/IRC-%23qaul.net-1e72ff.svg
[irc-url]: https://www.irccloud.com/invite?channel=%23qaul.net&hostname=irc.freenode.org&port=6697&ssl=1

A multi-payload, zone-encrypting, journaled persistence module, built
with low-overhead applications in mind.

- Stores data in namespaces and scopes
- Key-value stores and lazy blobs
- Supports per-scope asymetric encryption key
- Uses transaction Deltas for journal and concurrency safety
- Integrates into OS persistence layers (storing things on spinning
  rust or zappy quantum tunnels)

`alexandria` provides an easy to use database interface with
transactions, merges and dynamic queries, ensuring that your in-memory
representation of data never get's out-of-sync with your on-disk
representation. Don't burn your data.

## Payload types

`alexandria` supports key-value stores, encoded as `json` on the wire
format, and lazy blobs, meaning that they exist as blobs on disk, and
are only fetched when absolutely needed (you know, that 24GB copy of
Hackers we all have, but don't entirely understand the origins of).

Both `KV` and `Blob` payloads can use encryption at rest.

## Namespaces & Scopes

`alexandria` also has a users concept, allowing you to construct
permissive layers, optionally backed by encrpted storage. Referring to
a location in an `alexandria` library requires an `Address`, which
consists of an optional namespace, a scope and data ID.

We use the following notation in documentation and external queries:
`lib:</namespace?>/<scope>/<ID>`.

Each scope has metadata attributes that allow `alexandria` to handle
encryption, access, and on-disk offset management. What that means is
that a scope `lib:/me/downloads` might be saved into
`/home/me/downloads`, while the scope `lib:/me/secret_chat` is saved
into `/home/me/.local/share/chat_app/secret/`.

## Questions?

Check out the `examples` directory first, there's some cool ones in
there (I've been told by...someone).

`alexandria` is developed as part of [qaul.net][website]. We have a
[mailing list][list] and an [IRC channel][irc]! Please come by and ask
us questions!  (the issue tracker is a bad place to ask questions)

[website]: https://qaul.net
[list]: https://lists.sr.ht/~qaul/community/
[irc]: https://irccloud.com/freenode/#qaul.net

## License

`alexandria` is free software and part of [qaul.net][qaul.net]. You
are free to use, modify and redistribute the source code under the
terms of the GNU General Public License 3.0 or (at your choice) any
later version. For a full copy of the license, see `LICENSE` in the
source directory attached.
