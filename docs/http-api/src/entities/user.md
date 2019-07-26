# `user`
A qaul.net user

## Id
The user's identity encoded in Base64 with the URL-safe character set.

## Attributes
### `display_name: String` _(optional)_
A human readable display-name (like @foobar)
### `real_name: String` _(optional)_
A human's preferred call-sign ("Friends call me foo")
### `bio: {String}` _(optional)_
Key-value pairs of things the user deems interesting about themselves.
This could be stuff like "gender", "preferred languages" or whatever.
### `services: [String]` _(optional)_
The set of services this user runs.
### `avatar: String` _(optional)_
A users profile pictures encoded in Base64 with the URL-safe character set.

