Crypto module
============

The following documents outlines how to use the crypto submodule inside qaul.net without causing horrible memory leaks :)

The module itself consists of several discrete parts with well-defined functionality scopes. However for the sake of using the crypto submodule from the outside there are only a few modules we need to consider.

### Arbiter

This module contains the main API to the crypto submodule including `start` and `stop` functions to safely initialise & de-initialise the memory space of the module. Always call these functions to avoid memory leaks!

The core principle of the ariter is user IDs. These IDs are assigned at creation or re-initialisation of a user and need to be provided whenever acting in the name of a certain identity.

Internal users (so identities on the same client) are managed via user IDs. External users (so identities a client meets on the network) are not! They should still be tracked in the keystore

### Helper

Contains useful utility functions as well as globally defined variables for field sizes and error codes.

It contains the two legacy encoding functions `QCry_HashToString` and `QCry_StringToHash`. This functionality could (or should!) in the future be handled via an Encoding module.

It also contains a few base64 encoding functions that are used in the crypto module internally.


## How to use

So after this quick overview, let's look at how to use the crypto module via the arbiter. For a lot mor detail about certain functions, please consult the [API docs](). This includes clean initialisation and de-initialisation.

Please also note that every call into the arbiter API should be checked for it's return value. Sometimes weird things can happen and errors are very verbosely documented!

```C
// Create a variable to store return
int ret = qcry_arbit_init(1, exp_path, NULL);
```

The NULL that is provided there can also be an array of `qcry_usr_id` objects that are defined in the qcry_keystore.h as follows:

```C
typedef struct qcry_usr_id {
    char    *fingerprint;
    char    *username;
} qcry_usr_id;
```

So what needs to be provided is a list of fingerprint strings (`\0` terminated) as well as usernames (also `\0` terminated)

Obviously calling the function with `NULL` initialises an empty keystore which means that all future keys need to be added manually.

After this we can start creating users as follows:

```C
int user;
ret = qcry_arbit_usrcreate(&user, "username", "passphrase", QCRY_KEYS_RSA);
```

The `user` variable holds the ID that the arbiter assigned to this (internal) user and needs to be referenced again at later to sign and decrypt messages.

You can retrieve information from the arbiter via the `getusrinfo` function. Following there is an example to retrieve the public key from the just-now created user.

```C
char *pubkey
ret = qcry_arbit_getusrinfo(&pubkey, user, QAUL_PUBKEY);
```

To add keys to the arbiter keystore without having created a key-pair locally (for example to add a public key given to the current client via the network) you can use the following function:

```C
ret = qcry_arbit_addkey(kookiekey, strlen(kookiekey) + 1, kookie_fp, "spacekookie");
```
