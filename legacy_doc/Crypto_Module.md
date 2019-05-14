Crypto module
=============

The following documents outlines how to use the crypto submodule inside 
qaul.net.

The module itself consists of several discrete parts with well-defined 
functionality scopes. However, for the sake of using the crypto submodule 
from the outside there are only a few modules we need to consider.

Modules
-------

### Arbiter Module

The arbiter module contains the main API to the crypto submodule including 
`start` and `stop` functions to safely initialise & de-initialise the 
memory space of the module. 
Always call these functions to avoid memory leaks!

The core principle of the ariter is to manage user IDs. These IDs are 
assigned at creation or re-initialisation of a user and need to be 
provided whenever acting in the name of a certain identity.

Internal users (identities on the same client) are managed via user IDs. 
External users (identities a client meets on the network) are not! 
External users are identified via their public fingerprint.


### Helper Module

The helper module contains useful utility functions as well as globally 
defined variables for field sizes and error codes.

The helper module also contains the two legacy encoding functions 
`QCry_HashToString` and `QCry_StringToHash`. This functionality should 
in the future be handled via an encoding module.

The helper module also contains a few base64 encoding functions that are 
used by the crypto module internally.


How to use the Crypto Module
----------------------------

The crypto module is used via the arbiter. This includes clean 
initialisation and de-initialisation of the crypto module.
For more information about certain functions, please consult the [API docs](). 

Please also note that every call into the arbiter API should be checked 
for it's return value. Sometimes weird things happen and errors are very 
verbosely documented!

The following example initializes the crypto module with an empty keystore.

```C
// Create a variable to store return
int ret = qcry_arbit_init(1, exp_path, NULL);
```

To initialize the crypto module with all known keys, we can replace the
NULL of the last example with an array of `qcry_usr_id` objects that are 
defined in the qcry_keystore.h as follows:

```C
typedef struct qcry_usr_id {
    char    *fingerprint;
    char    *username;
} qcry_usr_id;
```

So what needs to be provided is a list of fingerprint strings (`\0` terminated) 
as well as usernames (also `\0` terminated).

Calling the function with `NULL` initialises an empty keystore 
which means that all future keys need to be added manually.


After the module was initialized, we can start creating users as follows:

```C
int user;
ret = qcry_arbit_usrcreate(&user, "username", "passphrase", QCRY_KEYS_RSA);
```

The `user` variable holds the ID that the arbiter assigned to this 
(internal) user and needs to be referenced again at later to sign and 
decrypt messages.

You can retrieve information from the arbiter via the `getusrinfo` 
function. Following there is an example to retrieve the public key from 
the just-now created user.

```C
char *pubkey
ret = qcry_arbit_getusrinfo(&pubkey, user, QAUL_PUBKEY);
```

To add keys to the arbiter keystore without having created a key-pair 
locally (for example to add a public key given to the current client via 
the network) you can use the following function:

```C
ret = qcry_arbit_addkey(kookiekey, strlen(kookiekey) + 1, kookie_fp, "spacekookie");
```
