# qaul.net services

Following is a list of services that are bundled in with qaul.net, and
what they can do for you.

**Important** because we're still in development, some of these
services don't exist yet!  If you find one that doesn't and you think
you would have fun implementing it, get in touch!

## Feed

* Service ID: `net.qaul.feed`
* Crate name: `qaul-feed`

Public message feed, similar to micro-blogging applications such as
mastodon or twitter.  Users can compose short-medium length messages
that get shared with the whole network.  You have the choice of
filtering by "all messages" and "only following", where you can only
display messages sent by friends or people you trust.


## Messaging

* Service ID: `net.qaul.messaging`
* Crate name: `qaul-messaging`

Private (1-to-1 or groups) text messaging, that can optionally (by
default) be encrypted. Conversations can either be displayed as a feed
(like chat), or threaded (like e-mail).  Files can be sent either
in-line (if the payload is small enough), or via file-announce
messages that use the "files" service.

## Filesharing

* Service ID: `net.qaul.files`
* Crate name: `qaul-files`

Filesharing via announce-links similar to how torrents get announced
on trackers.  You can announce a file to either a group of people, a
singe user, or the whole network.  Optionally this service can be
configured to download all files that were publicly announced to allow
servers to replicate a "public archive" that users can have access to,
if the original source of a file disappears.


## Voices

* Service ID: `net.qaul.voices`
* Crate name: `qaul-voices`

Integrates with various platform features to allow voice call
streaming over the qaul.net, as well as preparing audio messages that
can be inlined into messages, or sent to many people via announce
links.


## Radio

* Service ID: `net.qaul.radio`
* Crate name: `qaul-radio`

Similarly to voices, it integrates into platform features to provide
audio capture and playback, but for one-to-many streams.  This way
people can broadcast themselves into the network, while others can
tune into a program, without being able to respond.  Similar to files,
this service can be configured to automatically archive radio
broadcasts for community servers.
