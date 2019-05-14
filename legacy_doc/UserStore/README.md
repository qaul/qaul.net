# Qaul User Store

The user store is a submodule inside libqaul which handles user identities on the network. It can track a user by their fingerprint id, their username and even their IP address which is quickly searchable via a collection of hashtables.

All persistent user storage is done in the `.local/qaul` directory (or something similiar?) and has roughly the following structure to it.

<pre>
├── identities
│   ├── foobar.pri
│   └── foobar.pub
├── known_users.db
└── pubkeys
    ├── abcdefg.pub
    ├── hijklmnop.pub
    ├── qrstuvw.pub
    └── xyz.pub
</pre>

The identities folder is used by the crypto keystore which handles encrypted private keys as persistent storage. The pubkeys folder contains unencrypted public keys from other users that have been marked with some form of trust level.

The `known_users.db` is a simple SQL database which contains metadata about users. Alternatively that metadata can be stored in json (or similar) in a third directory if that is more convenient.