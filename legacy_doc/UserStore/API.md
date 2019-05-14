# UserStore API documentation


All interactions with the userstore API start by calling `qluser_store_initialise(...)` which sets up the userstore module to handle future transactions. If the user store is already initialised it will simply return a bad error code and be done.

After that you can create users with `qluser_store_adduser(const char*, const char*)` and afterwards query created users with `qluser_store_getby_fp(...)` which will return a pointer to the user struct that is stored in the table structure. **Do not free this user unless you want everything to break and a fire to burn your house down and murder your kittens and children**! When performing add-operations on a user, the fingerprint is used as a search index.

This means that if you only know a user by a different metric (ip or username) you need to search for the full user struct first so that you can use the fingerprint as the key for an add-operation.

This might seem cumbersome in comparison to just using the user-struct as an index metric but it allows you to grab the fingerprint off a public message and immediately perform add/ lookup operations with it instead of having to do a different query first.

All other API functions are documented `libqaul/user/qluser_store.h`