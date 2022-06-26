# qaul ID

libqaul uses PeerID from rust-libp2p as identifier as node ID's and user ID's.

The ID's exist in several forms:

* [PeerID] rust structure
* 38 byte binary representation
* [base58btc] (base58 bitcoin) encoded string 52 characters long

For BLE (Bluetooth Low Energy) connection module qaul uses a shortened part of the qaul ID.

## Binary qaul ID

The binary form of qaul id is 38 bytes long.
The first 6 bytes are always the same and represent the key type.
The hexadecimal values of the first 6 bytes are always: `0, 24, 8, 1, 12, 20`.

## Small qaul ID

The small qaul ID, is a 16 bytes slice from the qaul ID.
It is the slice from byte 7 to byte 22.
The first 6 bytes are left out, as they are always the equal, as they represent the key type.

[PeerID]: <https://docs.rs/libp2p/latest/libp2p/struct.PeerId.html>
[base58btc]: <https://en.bitcoinwiki.org/wiki/Base58>
