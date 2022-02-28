# qaul ID

libqaul uses PeerID from rust-libp2p as identifier as node ID's and user ID's.

The ID's exist in several forms:

* PeerID rust structure
* Binary form
* bs58 encoded string (same encoding as bitcoins)

For BLE (Bluetooth Low Energy) connection module qaul uses a shortened part of the qaul ID.

## Binary qaul ID

The binary form of qaul id is a 40 bytes long.
The first 6 bytes are always the same and represent the key type.
The hexadecimal values of the first 6 bytes are always: `0, 24, 8, 1, 12, 20`.

## Small ID

The small ID, is a 16 bytes slice from the qaul ID.
It is the slice from byte 7 to byte 22.
The first 6 bytes are left out, as they are always the equal, as they represent the key type.
