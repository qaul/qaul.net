# qaul.net Security Overview

## Identification

Each entity in qaul has an [Ed25519] keypair and a [qaul ID] which is a hash of the public key.

### Nodes

Each Node has an [Ed25519] keypair and a [qaul ID].

### Users

Each User can be addressed via it's [qaul ID] which is a hash of it's public [Ed25519] key.

### Message Identification

Each message is signed with a [Sha256] cryptographic hash by the sending user.
Via this hash the identity of the sending user can be verified.

## Transport Encryption

Every connection between two nodes is encrypted.

### LAN & Internet Connections

The TCP transport channel of the LAN & Internet connections is via the [Noise Protocol] `Noise_XX_25519_ChaChaPoly_Sha256`.

`Noise_XX_25519_ChaChaPoly_Sha256` splits down to the following parts:

* [Noise Protocol] - A crypto protocol based on [Diffie-Hellman] key agreement.
* [XX] - Identifiable handshake, where the static public keys of the nodes are exchanged within the handshake.
* [25519][Curve25519] - Using [Curve25519] elliptic curve keys and X25519 [Diffie-Hellman].
* [ChaChaPoly] - Using the [ChaCha20][ChaChaPoly] stream cipher with the [Poly1305] message authentication code.
* [Sha256] - Cryptographic hash.

This is a strong encryption with forward secrecy.

### BLE Connection

The transport encryption on BLE connections is provided by the Bluetooth stack.

## End to End Messaging Encryption

Each direct message to another user, has a strong end to end encryption, using the [Noise Protocol] `Noise_KK_25519_ChaChaPoly_Sha256`.

`Noise_KK_25519_ChaChaPoly_Sha256` splits down to the following parts:

* [Noise Protocol] - A crypto protocol based on [Diffie-Hellman] key agreement.
* [KK] - Handshake in which both sides know the other parties static public key.
* [25519][Curve25519] - Using [Curve25519] elliptic curve keys and X25519 [Diffie-Hellman].
* [ChaChaPoly] - Using the [ChaCha20][ChaChaPoly] stream cipher with the [Poly1305] message authentication code.
* [Sha256] - Cryptographic hash.

This protocol provides **zero-RTT** encryption, meaning that already the first message of a cryptographic hand-shake can contain an encrypted payload.

After the first handshake, this protocol provides strong forward secrecy and allows a secure delay tolerant communication.

[Ed25519]: <https://en.wikipedia.org/wiki/EdDSA#Ed25519>
[Curve25519]: <https://en.wikipedia.org/wiki/Curve25519>
[Noise Protocol]: <https://noiseprotocol.org/noise.html>
[XX]: <https://noiseexplorer.com/patterns/XX/>
[KK]: <https://noiseexplorer.com/patterns/KK/>
[ChaChaPoly]: <https://en.wikipedia.org/wiki/ChaCha20-Poly1305>
[Poly1305]: <https://en.wikipedia.org/wiki/Poly1305>
[Sha256]: <https://en.wikipedia.org/wiki/SHA-2>
[Diffie-Hellman]: <https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange>
[qaul ID]: qaulId.md
