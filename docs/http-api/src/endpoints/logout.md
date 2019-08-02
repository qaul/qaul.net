# `logout`
Logs a user out. It is important to do this so qaul can lock the user's data.

## Methods
- `GET`

## Arguments
None

## Returns
[`success`](/entities/success.html)

## Errors

### Method Not Allowed
**Status:** 405 _Method Not Allowed_

This endpoint only accepts `GET` requests and this request was of a different type

### Not Logged In
**Status:** 401 _Unauthorized_

No authenticating information was povided with the request and as such the 
endpoint doesn't know what user to log out
