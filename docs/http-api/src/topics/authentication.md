# Authentication

Qaul supports two forms of authentication for the HTTP Api. In the first a cookie
is set on the client, which if present in future requests will authenticate the 
client. The second provides the client with a token. The client then provides the token
in it's `Authorization` header on future requests.

## Obtaining a grant

The first step of dealing with authenticated qaul endpoints is to obtain a grant.
This is done through the [`login`](/endpoints/login.html) endpoint by sending a
[`user_auth`](/entities/user_auth.html`) entity. Below is an example of the body of
such a request:

```
{
	"data": {
		"id": "0000000000000000",
		"type": "user_auth",
		"attributes": {
			"secret": "my super secret password",
			"grant_type": "token",
		}
	}
}
```

## Cookie grant
As long as the client supports cookies the `cookie` grant type will simply
add a cookie to the client, requiring no futher action.

## Token grant
A token grant will return a [`user_grant`](/entities/user_grant.html) entity.
The `id` field of this entity contains the authentication token. The client must
take this token and include it in future requests in the `Authorization` header
as follows: `Authorization: Bearer <token>`.

## Authentication Errors
If included authentication information in the request will be checked for EVERY
request, even to endpoints that don't require authentication. These errors can
therefore occur on any endpoint.

### Differing Logins
**Status:** 400 _Bad Request_

There is both an `Authorization` header and a `bearer` token which both contain
valid auth tokens but the auth tokens they contain differ. Probably you should only
be sending one or the other but if you want to use both authentication schemes they both
need to contain the same token.

### Invalid Login Cookie
**Status:** 400 _Bad Request_

The `bearer` cookie contains an auth token which is either not currently or
never was valid.

### Invalid Login Token 
**Status:** 400 _Bad Request_

The `Authorization` header provided an auth token which is either not currently 
or never was valid.

## Returning a grant
To return a grant simply visit the [`logout`](/endpoints/logout.html) endpoint.
Be sure to do this when you're done so qaul can clean up the user's data!
