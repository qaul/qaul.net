# alexandria [![][ci-badge]][ci-url] [![][irc-badge]][irc-url]

[ci-badge]: https://git.open-communication.net/qaul/alexandria/badges/master/pipeline.svg
[ci-url]: https://git.open-communication.net/qaul/alexandria/commits/master
[irc-badge]: https://img.shields.io/badge/IRC-%23qaul.net-1e72ff.svg
 [irc-url]: https://www.irccloud.com/invite?channel=%23qaul.net&hostname=irc.freenode.org&port=6697&ssl=1

An strongly typed, encrypted record-based embedded database that
supports multiple data payloads.

In alexandria all data is addressed via a user prefix and path.  Data
is encrypted per-user, and can only be accessed if that user has
previously been "opened".

**Notice:** alexandria should be considered experimental and not used
in production systems where data loss is unacceptable.


## Features

- Store key-value data in encrypted trees
- Query based on record header data (tags, id, path)
- Subscribe to updates based on path prefixes



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
