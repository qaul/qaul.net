# qaul.net Security Overview

## Cryptographic Key

* Asymmetric Cryptography
* Key type: Ed25519

Each node and each user has their own key pair.

## qaul id

The qaul id is a multi-hash of the public key

* 38 bytes binary representation
* 52 character string, base58btc (base58 bitcoin) encoded

### Small id for BLE Identification Service

There is a smaller version of the qaul id of 16 bytes.
This version is used in situation where it is not feasible to
send the entire qaul id.

At the moment the small version is used in the BLE identification service.

The small version uses bytes 6-21 of the qaul ID.
(As bytes 0-5 of a qaul ID are always identical.)

The translation between the small id and qaul id is handled by
the BLE manager.

## Message Encryption & Signing

Each direct message is encrypted with the public / private keys of the
communicating users.

## Transport Encryption

### TCP

The TCP transport between nodes is encrypted via the noise protocol.
<http://www.noiseprotocol.org>

### BLE

The transport encryption on BLE is provided by the Bluetooth stack.
