# BLE GATT Messaging Diagrams

## Chunk Header (2 Bytes)

Every chunk begins with a 2-byte header (FLC messages are interpreted differently):

```mermaid
---
title: "Chunk Header (2 Bytes)"
---
packet-beta
  0-4: "queue index (5 bits)"
  5: "R (1)"
  6-15: "chunk index (10 bits)"
```

## Large Message Indicator Byte

Messages larger than **18,342 bytes** are split into up to 4 parts. Each part is sent as a separate chunked message with this indicator byte set in its first chunk header:

```mermaid
---
title: "Large Message Indicator Byte"
---
packet-beta
  0-3: "queue index (4 bits)"
  4-5: "total (2 bits)"
  6-7: "part (2 bits)"
```

## SendQueueMessage States

```mermaid
stateDiagram-v2
    [*] --> EMPTY
    EMPTY --> QUEUED
    QUEUED --> SENDING
    SENDING --> SENT
    SENT --> SUCCESS
    SENT --> MISSING
    MISSING --> SENT : resend
    MISSING --> ERROR
```

## ReceiveQueueMessage States

```mermaid
stateDiagram-v2
    [*] --> RECEIVING
    RECEIVING --> WAITING_ON_MISSING
    RECEIVING --> RECEIVED
    WAITING_ON_MISSING --> RECEIVED
    WAITING_ON_MISSING --> ERROR
```

## Complete Message Flow

### 1. Identity Handshake

On every new connection, both sides exchange qaul IDs:

```mermaid
sequenceDiagram
    participant A as Device A
    participant B as Device B

    A->>B: SEND_QAUL_ID [8B qaul_id]
    B->>A: SEND_QAUL_ID [8B qaul_id]
```

### 2. Normal Message Transmission

```mermaid
sequenceDiagram
    participant S as Sender
    participant R as Receiver

    S->>R: Chunk 0 (19B hdr + payload) — first chunk with metadata
    S->>R: Chunk 1 (2B hdr + payload)
    S->>R: Chunk 2 (2B hdr + payload)
    Note over S,R: ...
    S->>R: Chunk N (2B hdr + payload) — last chunk
    R->>S: ACK_SUCCESS [queue_idx] — all chunks received, CRC valid
```

### 3. Missing Chunk Recovery

```mermaid
sequenceDiagram
    participant S as Sender
    participant R as Receiver

    S->>R: Chunk 0
    S->>R: Chunk 1
    S-xR: Chunk 2 (lost)
    S-xR: Chunk 3 (lost)
    S->>R: Chunk 4 — gap detected: 2, 3 missing
    Note over S,R: ...
    S->>R: Chunk N
    R->>S: MISSING_CHUNKS [2, 3] — request missing chunks
    S->>R: Chunk 2 (R=1) — resend
    S->>R: Chunk 3 (R=1) — resend
    R->>S: ACK_SUCCESS [queue_idx] — complete, CRC valid
```

### 4. Large Message Transmission

Messages larger than 18,342 bytes are split into up to 4 parts:

```mermaid
sequenceDiagram
    participant S as Sender
    participant R as Receiver

    rect rgb(240, 248, 255)
        Note over S,R: Part 0 — largeMsg: queue=1, total=3, part=0
        S->>R: Chunks 0..N
        R->>S: ACK_SUCCESS
    end

    rect rgb(240, 255, 240)
        Note over S,R: Part 1 — largeMsg: queue=1, total=3, part=1
        S->>R: Chunks 0..M
        R->>S: ACK_SUCCESS
    end

    rect rgb(255, 248, 240)
        Note over S,R: Part 2 — largeMsg: queue=1, total=3, part=2
        S->>R: Chunks 0..K
        R->>S: ACK_SUCCESS
    end

    Note over R: Receiver assembles final message
```

Each part is independently chunked and ACK'd. Once all parts are received, the receiver concatenates them into the final message.
