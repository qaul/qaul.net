# `logout`
Logs a user out. It is important to do this so qaul can lock the user's data.

## Arguments
None

## Returns
[`success`](/entities/success.html)

## Errors

### Not Logged In
**Status:** 401 _Unauthorized_

No authenticating information was povided with the request and as such the 
endpoint doesn't know what user to log out
