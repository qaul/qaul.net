# `user_auth`
Used by an api consumer to authenticate itself for future requests.

## Id 
The Id of the authorizing user, Base64 encoded with the URL-safe character set.

## Attributes
### `secret: String`
The authorizing user's password
### `grant_type: String`
The type of grant to be returned has two possible values:
- `token`
- `cookie`

The `token` grant requests an authentication token be returned in a 
[`user_grant`](/entities/user_grant.html) entity. The `cookie` grant requests a 
cookie be set instead.
