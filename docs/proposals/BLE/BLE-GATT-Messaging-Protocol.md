# qaul BLE GATT Messaging Protocol

This document describes the BLE GATT messaging protocol used by qaul to reliably transfer arbitrarily large messages between two devices over Bluetooth Low Energy.

The protocol chops the encrypted binary messages from libqaul into chunks matching the MTU of the GATT protocol. It adds a 2 Byte header to each chunk and sends the chunks to the receiving device. The receiving device keeps track of the received chunks, requests missing chunks, assembles and acknowledges received messages.

A special challenge is the fact, that newest android operating systems show a different MAC address for each connection. As the outgoing and the incoming connection have different MAC addresses.

## Overview

BLE GATT characteristics have a small maximum transmission unit (MTU). On Android <14 the effective payload per GATT write is **20 bytes**; on Android >=14 it is up to **512 bytes**.

The qaul BLE GATT messaging protocol provides reliable, acknowledged delivery of messages that can be many kilobytes in size, by:

1. Splitting messages into small **chunks** that fit within a single GATT write.
2. Numbering chunks with a compact **2-byte header** so the receiver can detect gaps.
3. Using **flow control messages (FLC)** for identity exchange, missing chunk requests, and acknowledgments.
4. Supporting **large messages** (>18 KB) by splitting them into up to 4 **message parts**, each sent as an independent chunked message.

## GATT Service & Characteristics

```
Service UUID:       4db14399-0bd0-4445-9906-47d9c4791cff
Message Service:    4db14400-0bd0-4445-9906-47d9c4791cff  (reserved)
READ_CHAR:          4db14401-0bd0-4445-9906-47d9c4791cff  (stores qaul ID)
MSG_CHAR:           4db14402-0bd0-4445-9906-47d9c4791cff  (send/receive messages)
GD_CHAR:            4db14403-0bd0-4445-9906-47d9c4791cff  (unused/reserved)
```

- `READ_CHAR` holds the device's 8-byte **qaul ID** for remote devices to read.
- `MSG_CHAR` is the characteristic used for all chunked message and flow control traffic.

## Protocol Constants

| Constant | Value | Description |
|---|---|---|
| Chunk header size | 2 bytes | Header on every chunk |
| First chunk header size | 19 bytes | Extended header on chunk 0 of each message |
| Default chunk size | 20 bytes | Total chunk size (header + payload) |
| Message queue slots | 29 | Indexes 1–29, rotating |
| Max chunk index | 1023 | 10-bit field |
| MAX_MESSAGE_PART_SIZE | 18,342 bytes | Max size of one message part |
| Max message parts | 4 | Parts 0–3 for large messages |
| Large message queues | 15 | Indexes 1–15 |
| MAX_MISSING_GAP | 12 | Max gap before error |
| Max missing chunks per FLC | 9 | Batch size for missing chunk requests |

---

## Message Types

Flow Control Messages (FLC)

- FLC messages manage the communication between two BLE interfaces
- The following FLC messages exist
  - Request 8 Byte qaul id
  - send 8 Byte qaul id
  - request missing chunks
  - send ACK
  - request ACK

Message Chunks

- Chunk messages 
- Each Chunk message has a 2 Byte header with the following information:
  - message queue index
  - resend indicator
  - chunk index
- The header of the first chunk of each qaul message, has a 19 Byte header with the following info:
  - total message size
  - total amount of chunks
  - CRC of the message
  - 8 Byte qaul id of the sending node

## Chunk Header (2 Bytes)

Every chunk begins with a 2-byte header (FLC messages are interpreted differently):

```
 Bit:  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0
      [--- queue index ---] [R] [---------- chunk index -----------]
       5 bits                1    10 bits
```

| Field | Bits | Range | Description |
|---|---|---|---|
| Queue index | 15–11 | 0–31 | Message queue identifier. 0 = FLC message, 1–29 = data queue, 30–31 = reserved. |
| Resend indicator (R) | 10 | 0–1 | 0 = first transmission, 1 = retransmission of a missing chunk. |
| Chunk index | 9–0 | 0–1023 | Position of this chunk within the message. |

When `queueIndex == 0`, the chunk is a **flow control message** and the header is interpreted differently (see below).


## First Chunk Header (19 Bytes)

Chunk 0 of every message carries an extended 19-byte header:

```
Offset  Size   Field
──────  ────   ─────
 0–1    2 B    Standard chunk header (queue index, resend, chunk index = 0)
 2      1 B    Large message indicator
 3–4    2 B    Message size (big-endian, max 65535)
 5–6    2 B    Total chunk count (big-endian)
 7–10   4 B    CRC32 checksum of the full message payload
11–18   8 B    Sender's qaul ID
```

If the message fits in a single part (not a large message), the large message indicator byte is `0x00`.

After the 19-byte header, the remaining bytes of chunk 0 carry message payload:
```
first chunk payload = chunkSize − 19 bytes
```

With a 20-byte chunk size, the first chunk carries **1 byte** of payload. Subsequent chunks carry `chunkSize − 2` bytes of payload (18 bytes at default chunk size).

**Total chunk count formula:**
```
totalChunks = ceil((messageSize - (chunkSize - 19)) / (chunkSize - 2)) + 1
```

---

## Large Message Indicator Byte

Messages larger than **18,342 bytes** are split into up to 4 parts. Each part is sent as a separate chunked message with this indicator byte set in its first chunk header:

```
 Bit:   7   6   5   4   3   2   1   0
       [-- queue index --] [total] [part]
        4 bits              2 bits  2 bits
```

| Field | Bits | Description |
|---|---|---|
| Queue index | 7–4 | Large message queue index (1–15). 0 = not a large message. |
| Parts total | 3–2 | Total number of parts (value as-is, not minus one). |
| Part number | 1–0 | Index of this part (0–3). |

When this byte is `0x00`, the message is a normal (non-large) message.

---

## Flow Control Messages (FLC)

Flow control messages have queue index `0` in the 2-byte header. The FLC uses a simpler 1-byte type scheme where the first byte of the chunk determines the message type:

### FLC Message Types

| Type | Value | Payload | Description |
|---|---|---|---|
| REQUEST_QAUL_ID | `0x00` | none | Request the peer to send its qaul ID |
| SEND_QAUL_ID | `0x01` | 8 bytes (qaul ID) | Send own qaul ID to the peer |
| MISSING_CHUNKS | `0x02` | N × 2 bytes (chunk identifiers) | Request retransmission of missing chunks |
| ACK_SUCCESS | `0x03` | 1 byte (queue index) | Acknowledge successful reception of a message |
| ACK_ERROR | `0x04` | 1 byte (queue index) + 1 byte (error code) | Report reception error for a message |
| MISSING_ACK | `0x05` | 1 byte (queue index) | Request re-send of a missing ACK |

### FLC Wire Formats

**REQUEST_QAUL_ID** (1 byte):
```
[0x00]
```

**SEND_QAUL_ID** (9 bytes):
```
[0x01] [8 bytes qaul ID]
```

**MISSING_CHUNKS** (1 + N×2 bytes, max N=9):
```
[0x02] [high₁ low₁] [high₂ low₂] ... [highₙ lowₙ]
```
Each 2-byte chunk identifier uses the same bit layout as the standard chunk header (queue index in bits 15–11, chunk index in bits 9–0), allowing a single FLC to request chunks from different message queues.

**ACK_SUCCESS** (2 bytes):
```
[0x03] [queue_index]
```

**ACK_ERROR** (3 bytes):
```
[0x04] [queue_index] [error_code]
```

**MISSING_ACK** (2 bytes):
```
[0x05] [queue_index]
```

---

## Sending Priority

When preparing chunks to send, the protocol follows this priority order:

1. **Qaul ID** — `SEND_QAUL_ID` is sent as the very first message on a new connection.
2. **FLC messages** — ACKs, ID requests, and other flow control messages.
3. **Missing chunk requests** — `MISSING_CHUNKS` FLCs for chunks the receiver needs.
4. **Missing chunks to resend** — Actual chunk data retransmissions requested by the peer.
5. **New message chunks** — The next queued message is chunked and sent.

---

## Send Queue Architecture

Each peer connection maintains a `SendQueue` with:

- **29 message queue slots** (indexes 1–29, rotating). When index exceeds 29, it wraps to 1.
- **FLC queue** (`flcToSend`) — Priority queue for flow control messages.
- **Missing chunks to request** — Map of chunk identifiers the local receiver needs.
- **Missing chunks to send** — Map of chunk identifiers the remote receiver has requested.
- **Messages to send** — FIFO queue of `(message, messageId, largeMessageIndicator)` triples.

### SendQueueMessage States

```
EMPTY → QUEUED → SENDING → SENT → SUCCESS
                              ↓
                           MISSING → (resend) → SENT
                              ↓
                            ERROR
```

| State | Meaning |
|---|---|
| `EMPTY` | Slot is free, waiting for new message |
| `QUEUED` | Message queued, not yet sending |
| `SENDING` | Chunks are being transmitted |
| `SENT` | All chunks transmitted, waiting for ACK |
| `MISSING` | Peer reported missing chunks |
| `SUCCESS` | ACK_SUCCESS received |
| `ERROR` | ACK_ERROR received or connection lost |

---

## Receive Queue Architecture

Each peer connection maintains a `ReceiveQueue` with:

- **29 receive queue slots** (indexes 1–29), each a `ReceiveQueueMessage`.
- **Large message queues** — Map for tracking multi-part large messages.

### ReceiveQueueMessage States

```
RECEIVING → WAITING_ON_MISSING → RECEIVED
                ↓
              ERROR
```

| State | Meaning |
|---|---|
| `RECEIVING` | Actively receiving chunks |
| `WAITING_ON_MISSING` | Last chunk arrived, waiting for gap fills |
| `RECEIVED` | All chunks received, CRC valid, ACK_SUCCESS sent |
| `ERROR` | Validation failed, ACK_ERROR sent |

### Missing Chunk Detection

When a chunk arrives with index `i` and the last received index was `j`, if `i > j + 1`, all indexes in the gap `(j+1 .. i-1)` are added to the missing chunks list. Missing chunk requests are encoded as 2-byte identifiers and batched into MISSING_CHUNKS FLC messages (up to 9 per FLC).

When a retransmitted chunk arrives (resend indicator = 1), it is matched against the missing list and removed.

---

## Connection Manager

Android BLE MAC addresses can change between connections, and incoming/outgoing connections may use different MAC addresses for the same device. The `ConnectionManager` bridges this by:

- Mapping `SendQueue` and `ReceiveQueue` using the **qaul ID** (8 bytes, stored as Long) as the unique key, not the BLE MAC address.
- Routing received FLC results (ACKs, missing chunk requests) from the ReceiveQueue to the correct SendQueue via `SendQaulIdQueue`.

### SendQaulIdQueue

Each qaul ID has a `SendQaulIdQueue` that collects:
- ACKs to forward to the SendQueue
- Missing chunk indexes to request
- Missing chunk indexes to resend
- ACKs received for sent messages

---

## Complete Message Flow

### 1. Identity Handshake

On every new connection, both sides exchange qaul IDs:

```
  Device A                          Device B
     │                                 │
     │── SEND_QAUL_ID [8B qaul_id] ──→│
     │                                 │
     │←── SEND_QAUL_ID [8B qaul_id] ──│
     │                                 │
```

### 2. Normal Message Transmission

```
  Sender                            Receiver
     │                                 │
     │── Chunk 0 (19B hdr + payload) ─→│  First chunk with metadata
     │── Chunk 1 (2B hdr + payload)  ─→│
     │── Chunk 2 (2B hdr + payload)  ─→│
     │      ...                        │
     │── Chunk N (2B hdr + payload)  ─→│  Last chunk
     │                                 │
     │←── ACK_SUCCESS [queue_idx] ─────│  All chunks received, CRC valid
     │                                 │
```

### 3. Missing Chunk Recovery

```
  Sender                            Receiver
     │                                 │
     │── Chunk 0                     ─→│
     │── Chunk 1                     ─→│
     │── Chunk 2                     ──X  (lost)
     │── Chunk 3                     ──X  (lost)
     │── Chunk 4                     ─→│  Gap detected: 2,3 missing
     │      ...                        │
     │── Chunk N                     ─→│
     │                                 │
     │←── MISSING_CHUNKS [2, 3] ───────│  Request missing chunks
     │                                 │
     │── Chunk 2 (R=1)              ─→│  Resend with resend indicator
     │── Chunk 3 (R=1)              ─→│
     │                                 │
     │←── ACK_SUCCESS [queue_idx] ─────│  Complete, CRC valid
     │                                 │
```

### 4. Large Message Transmission

Messages larger than 18,342 bytes are split into up to 4 parts:

```
  Sender                            Receiver
     │                                 │
     │  Part 0 (chunks 0..N)         ─→│  largeMsg indicator: queue=1, total=3, part=0
     │←── ACK_SUCCESS                ──│
     │                                 │
     │  Part 1 (chunks 0..M)         ─→│  largeMsg indicator: queue=1, total=3, part=1
     │←── ACK_SUCCESS                ──│
     │                                 │
     │  Part 2 (chunks 0..K)         ─→│  largeMsg indicator: queue=1, total=3, part=2
     │←── ACK_SUCCESS                ──│  Receiver assembles final message
     │                                 │
```

Each part is independently chunked and ACK'd. Once all parts are received, the receiver concatenates them into the final message.

---

## Validation

When all chunks of a message have been received, the receiver:

1. **Assembles** chunks in order (0 to totalChunks-1).
2. **Checks message size** — assembled size must match the `messageSize` field from chunk 0.
3. **Validates CRC32** — computed CRC32 of the assembled payload must match the checksum from chunk 0.
4. On success: sends `ACK_SUCCESS` and delivers the message.
5. On failure: sends `ACK_ERROR` with an error code.

---

## Example: 100-byte message with 20-byte chunks

```
Message size:     100 bytes
Chunk size:       20 bytes
First chunk payload: 20 - 19 = 1 byte
Other chunk payload: 20 - 2  = 18 bytes
Remaining after first: 100 - 1 = 99 bytes
Additional chunks: ceil(99 / 18) = 6
Total chunks:      7

Chunk 0: [2B header][1B lgMsg=0x00][2B size=100][2B chunks=7][4B CRC32][8B qaulId][1B payload]
Chunk 1: [2B header][18B payload]
Chunk 2: [2B header][18B payload]
Chunk 3: [2B header][18B payload]
Chunk 4: [2B header][18B payload]
Chunk 5: [2B header][18B payload]
Chunk 6: [2B header][9B payload]   (last chunk, partial)
```

---

## Source Files

| File | Purpose |
|---|---|
| `SendQueue.kt` | Send queue management, chunking, large message splitting |
| `ReceiveQueue.kt` | Receive queue, chunk assembly, CRC validation |
| `FlcCreate.kt` | Factory for creating FLC messages |
| `ConnectionManager.kt` | Maps qaul IDs to send/receive queues across MAC addresses |
| `QueueModels.kt` | Data models: FlowControlMessageType enum, queue message classes |
| `BleService.kt` | GATT server/client setup, service & characteristic UUIDs |
| `BleActor.kt` | GATT client connection & chunk write scheduling |
| `BLEUtils.kt` | Byte conversion utilities |
