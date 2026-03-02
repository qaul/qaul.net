# qaul Messaging Protobuf Structure

This document describes the protobuf messaging structure used in qaul for network communication between users.

## Overview

The qaul messaging system uses a layered architecture with cryptographic signatures and encryption. Messages flow through several layers:

```TXT
Container (signed)
  └── Envelope (sender/receiver routing)
        └── EnvelopPayload
              └── Encrypted / DTN
                    └── Messaging (actual content)
```

## Core Message Types

### 1. Container (`messaging.proto`)

The outermost wrapper for all network messages. Contains a cryptographic signature for message authentication.

```protobuf
message Container {
    bytes signature = 1;    // Signed by sending user
    Envelope envelope = 2;  // Message envelope
}
```

### 2. Envelope (`messaging.proto`)

Routing information identifying sender and receiver.

```protobuf
message Envelope {
    bytes sender_id = 1;    // qaul ID of the sender
    bytes receiver_id = 2;  // qaul ID of the receiver
    bytes payload = 3;      // Encoded EnvelopPayload
}
```

### 3. EnvelopPayload (`messaging.proto`)

The payload within an envelope, which can be either encrypted data or a DTN (Delay-Tolerant Networking) message.

```protobuf
message EnvelopPayload {
    oneof payload {
        Encrypted encrypted = 1;  // Encrypted message data
        bytes dtn = 2;            // DTN message
    }
}
```

## Encryption Layer

### CryptoState

Defines the current state of the cryptographic session:

| State | Value | Description |
|-------|-------|-------------|
| `NONE` | 0 | No encryption |
| `HANDSHAKE` | 1 | Crypto session is in handshake state |
| `TRANSPORT` | 2 | Crypto session is in transport state (fully established) |

### Encrypted Message

```protobuf
message Encrypted {
    CryptoState state = 1;    // Current crypto session state
    uint32 session_id = 2;    // Crypto session identifier
    repeated Data data = 3;   // One or more Data messages (max 64KB each)
}
```

### Data Chunk

Individual encrypted data chunks with unique nonces:

```protobuf
message Data {
    uint64 nonce = 1;  // Unique nonce, increments for each package
    bytes data = 2;    // Encrypted data slice (max 64KB)
}
```

## Application Message Layer

### Messaging (`messaging.proto`)

The unified message container for all application-level messages:

```protobuf
message Messaging {
    oneof message {
        Confirmation confirmation_message = 1;      // Delivery confirmation
        DtnResponse dtn_response = 2;               // DTN response
        CryptoService crypto_service = 3;           // Crypto handshake messages
        RtcStreamMessage rtc_stream_message = 4;    // RTC streaming
        GroupInviteMessage group_invite_message = 5; // Group invitations
        CommonMessage common_message = 6;           // Regular messages
    }
}
```

### CommonMessage

The primary message type for user content:

```protobuf
message CommonMessage {
    bytes message_id = 1;   // Unique message identifier
    bytes group_id = 2;     // Target group ID
    uint64 sent_at = 3;     // Timestamp when sent

    oneof payload {
        ChatMessage chat_message = 4;   // Text chat
        FileMessage file_message = 5;   // File transfer
        GroupMessage group_message = 6; // Group management
        RtcMessage rtc_message = 7;     // Real-time communication
    }
}
```

### Message Confirmation

Delivery acknowledgment sent back to the sender:

```protobuf
message Confirmation {
    bytes signature = 1;     // Reference to original message ID
    uint64 received_at = 2;  // Timestamp of receipt
}
```

## DTN (Delay-Tolerant Networking)

Supports store-and-forward messaging for disconnected networks.

### DTN Container

```protobuf
message Dtn {
    oneof message {
        bytes container = 1;       // Message container for storage
        DtnResponse response = 2;  // Response to DTN request
    }
}
```

### DTN Response

```protobuf
message DtnResponse {
    ResponseType response_type = 1;  // ACCEPTED or REJECTED
    bytes signature = 2;              // Message signature reference
    Reason reason = 3;                // Rejection reason (if applicable)
}
```

**Response Types:**
| Type | Value | Description |
|------|-------|-------------|
| `ACCEPTED` | 0 | Message accepted for storage |
| `REJECTED` | 1 | Message rejected |

**Rejection Reasons:**
| Reason | Value | Description |
|--------|-------|-------------|
| `NONE` | 0 | No specific reason |
| `USER_NOT_ACCEPTED` | 1 | User is not accepted |
| `OVERALL_QUOTA` | 2 | Overall storage quota reached |
| `USER_QUOTA` | 3 | User-specific quota reached |

---

## Chat File Transfer (`chatfile_net.proto`)

Protocol for transferring files within chat conversations.

### ChatFileContainer

```protobuf
message ChatFileContainer {
    oneof message {
        ChatFileInfo file_info = 1;  // File metadata
        ChatFileData file_data = 2;  // File content chunk
    }
}
```

### ChatFileInfo

Metadata about a file being transferred:

```protobuf
message ChatFileInfo {
    uint64 file_id = 1;           // Unique file identifier
    string file_name = 2;         // Original filename
    string file_extension = 3;    // File extension
    uint32 file_size = 4;         // Total file size in bytes
    string file_description = 5;  // Optional description
    uint32 start_index = 6;       // DEPRECATED
    uint32 message_count = 7;     // Total number of chunks
    uint32 data_chunk_size = 8;   // Size of each data chunk
}
```

### ChatFileData

Individual file data chunk:

```protobuf
message ChatFileData {
    uint64 file_id = 1;       // Reference to file
    uint32 start_index = 2;   // Chunk start index
    uint32 message_count = 3; // Number of messages in this chunk
    bytes data = 4;           // Chunk data
}
```

---

## Group Management (`group_net.proto`)

Protocol for managing group conversations and membership.

### GroupContainer

```protobuf
message GroupContainer {
    oneof message {
        InviteMember invite_member = 1;  // Group invitation
        ReplyInvite reply_invite = 2;    // Invitation response
        GroupInfo group_info = 3;        // Group status update
        RemovedMember removed = 4;       // Member removal notification
    }
}
```

### GroupInfo

Complete group information:

```protobuf
message GroupInfo {
    bytes group_id = 1;              // Unique group identifier
    string group_name = 2;           // Display name
    uint64 created_at = 3;           // Creation timestamp
    uint32 revision = 4;             // Group revision number
    repeated GroupMember members = 5; // Member list
}
```

### GroupMember

Individual member information:

```protobuf
message GroupMember {
    bytes user_id = 1;              // User's qaul ID
    GroupMemberRole role = 2;       // User or Admin
    uint64 joined_at = 3;           // Join timestamp
    GroupMemberState state = 4;     // Invited or Activated
    uint32 last_message_index = 5;  // Last seen message index
}
```

### Member Roles

| Role | Value | Description |
|------|-------|-------------|
| `User` | 0 | Regular group member |
| `Admin` | 255 | Group administrator |

### Member States

| State | Value | Description |
|-------|-------|-------------|
| `Invited` | 0 | User has been invited but not yet active |
| `Activated` | 1 | User has accepted and is active |

### Invitation Flow

1. **InviteMember** - Contains full `GroupInfo` to invite a new member
2. **ReplyInvite** - Response with `group_id` and `accept` boolean
3. **RemovedMember** - Notification when a member is removed (contains `group_id`)

---

## Message Flow Diagram

```TXT
┌─────────────────────────────────────────────────────────────────┐
│                         SENDER                                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  1. Create Application Message (ChatMessage, FileMessage, etc.) │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. Wrap in CommonMessage (add message_id, group_id, timestamp) │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. Wrap in Messaging union type                                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. Encrypt → Encrypted (with session_id, nonce, data chunks)   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. Wrap in EnvelopPayload                                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  6. Create Envelope (sender_id, receiver_id, payload)           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  7. Sign and wrap in Container (signature, envelope)            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    [ Network Transport ]
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         RECEIVER                                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  1. Verify signature from Container                             │
│  2. Extract Envelope, identify sender/receiver                  │
│  3. Decrypt EnvelopPayload                                      │
│  4. Process Messaging content                                   │
│  5. Send Confirmation back to sender                            │
└─────────────────────────────────────────────────────────────────┘
```


## Related Files

- `protobuf/proto_definitions/services/messaging/messaging.proto` - Core messaging structures
- `protobuf/proto_definitions/services/chat/chatfile_net.proto` - File transfer protocol
- `protobuf/proto_definitions/services/group/group_net.proto` - Group management protocol