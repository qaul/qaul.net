# `login`
Allows a user to authenticate with qaul.

## Methods
- `POST`

## Arguments
Expects a [`user_auth`](/entities/user_auth.html) entity

## Returns
Depending on the arguments `grant_type` field, either a 
[`user_grant`](/entities/user_grant.html) entity (for `token` grants) or a 
[`success`](/entities/success.html) entity (for `cookie` grants).

## Errors
### Conversion Error
**Status:** 400 _Bad Request_

There was something wrong with the primary data of the request. A number of things 
can cause this: the primary data can have the incorrect type, the primary data could
be missing a required attrubte, or the `grant_type` attribute could contain an invalid
value. Check the `detail` field of the error for more information.

### Invalid Identity
**Status:** 400 _Bad Request_

The id field of the primary data failed to decode, check the 
[`user_auth`](/entities/user_auth.html) entity for information about its contents.

### Method Not Allowed
**Status:** 405 _Method Not Allowed_

This endpoint expects a `POST` request and what it was provided was not a `POST`
request

### Multiple Data 
**Status:** 400 _Bad Request_

The request provided an array of data instead a singular data

### No Attributes
**Status:** 400 _Bad Request_

The primary data had no attributes at all

### Not Authorized
**Status:** 401 _Unauthorized_

This likely occured because the secret is wrong

### No Data
**Status:** 400 _Bad Request_

The request did not contain primary data

### Unknown User
**Status:** 404 _Not Found_

The decoded identity did not point to a real user
