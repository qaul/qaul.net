# Cookies

There are a number of errors related to the processing of cookies. As these errors 
can appear on any endpoint they are documented here.

## Errors

### Cookie Parse Error
**Status:** 400 _Bad Request_

There are three reasons this can happen: There could be a cookie in the header that is simple a name with no value, there could be a cookie with not name, or there could be a cookie that failed to decode as valid UTF-8. The detail field will help you tell which of these has occured. If this happened in normal operation (as in if you have not messed with your cookies), it is a bug, please report it. Clearing your cookies will resolve the issue, though note that this will leave any users logged in with a cookie grant unlocked.
