# Messenger Message Structure Comparison

## Overview

This document compares the message structures of four major messaging platforms:

- **Signal Messenger** — Uses Protocol Buffers (protobuf)
- **Telegram Messenger** — Uses TL (Type Language) schema with MTProto
- **Matrix Protocol** — Uses JSON events
- **WhatsApp** — Uses Protocol Buffers (protobuf)

---

## 1. Normal Text Message

### Signal

Signal uses Protocol Buffers. Text messages are wrapped in a `DataMessage`:

```protobuf
message DataMessage {
  optional string body = 1;              // The message text
  optional uint64 timestamp = 4;         // Message timestamp
  optional uint32 expireTimer = 5;       // Disappearing message timer
  optional bytes  profileKey = 6;        // Sender's profile key
}
```

Example (conceptual JSON representation):
```json
{
  "body": "Hello, World!",
  "timestamp": 1704067200000
}
```

---

### Telegram

Telegram uses TL schema. A basic message:

```tl
message#9815cec8 flags:# 
  out:flags.1?true 
  mentioned:flags.4?true 
  media_unread:flags.5?true 
  silent:flags.13?true 
  post:flags.14?true 
  id:int 
  from_id:flags.8?Peer 
  peer_id:Peer 
  date:int 
  message:string 
  entities:flags.7?Vector<MessageEntity>
  = Message;
```

Example (conceptual):
```json
{
  "id": 12345,
  "from_id": {"user_id": 123456789},
  "peer_id": {"chat_id": 987654321},
  "date": 1704067200,
  "message": "Hello, World!",
  "out": false
}
```

---

### Matrix

Matrix uses JSON events with type `m.room.message`:

```json
{
  "type": "m.room.message",
  "event_id": "$143273582443PhrSn:example.org",
  "room_id": "!636q39766251:example.com",
  "sender": "@alice:example.org",
  "origin_server_ts": 1704067200000,
  "content": {
    "msgtype": "m.text",
    "body": "Hello, World!"
  }
}
```

---

### WhatsApp

WhatsApp uses Protocol Buffers similar to Signal:

```protobuf
message Message {
  optional string conversation = 1;           // Simple text message
  optional ExtendedTextMessage extendedTextMessage = 6;  // With metadata
}

message ExtendedTextMessage {
  optional string text = 1;
  optional string matchedText = 2;
  optional string canonicalUrl = 4;
  optional string description = 5;
  optional string title = 6;
  optional ContextInfo contextInfo = 17;
}
```

Example (conceptual JSON):
```json
{
  "conversation": "Hello, World!"
}
```

---

## 2. Message with an Image

### Signal

```protobuf
message AttachmentPointer {
  optional fixed64 cdnId = 1;           // CDN identifier
  optional bytes   cdnKey = 15;         // CDN key
  optional string  contentType = 2;     // MIME type (e.g., "image/jpeg")
  optional bytes   key = 3;             // Encryption key
  optional uint32  size = 4;            // File size
  optional bytes   thumbnail = 5;       // Optional thumbnail
  optional bytes   digest = 6;          // SHA256 digest
  optional string  fileName = 7;        // Original filename
  optional uint32  width = 18;
  optional uint32  height = 19;
  optional string  caption = 20;
}

message DataMessage {
  repeated AttachmentPointer attachments = 2;
  optional string body = 1;             // Caption
}
```

---

### Telegram

```tl
messageMediaPhoto#695150d7 
  flags:# 
  spoiler:flags.3?true 
  photo:flags.0?Photo 
  ttl_seconds:flags.2?int 
  = MessageMedia;

photo#fb197a65 
  flags:# 
  has_stickers:flags.0?true 
  id:long 
  access_hash:long 
  file_reference:bytes 
  date:int 
  sizes:Vector<PhotoSize> 
  dc_id:int 
  = Photo;
```

Example:
```json
{
  "id": 12345,
  "message": "Check this out!",
  "media": {
    "_": "messageMediaPhoto",
    "photo": {
      "id": 5142579120000000000,
      "access_hash": -1234567890,
      "date": 1704067200,
      "sizes": [
        {"type": "s", "w": 90, "h": 90},
        {"type": "m", "w": 320, "h": 320},
        {"type": "x", "w": 800, "h": 800}
      ],
      "dc_id": 2
    }
  }
}
```

---

### Matrix

```json
{
  "type": "m.room.message",
  "sender": "@alice:example.org",
  "content": {
    "msgtype": "m.image",
    "body": "image.jpg",
    "info": {
      "mimetype": "image/jpeg",
      "size": 31037,
      "w": 394,
      "h": 398,
      "thumbnail_info": {
        "mimetype": "image/jpeg",
        "size": 5140,
        "w": 100,
        "h": 100
      },
      "thumbnail_url": "mxc://example.org/thumbnail123"
    },
    "url": "mxc://example.org/JWEIFJgwEIhweiWJE"
  }
}
```

---

### WhatsApp

```protobuf
message ImageMessage {
  optional string url = 1;              // CDN URL
  optional string mimetype = 2;         // "image/jpeg"
  optional string caption = 3;          // Optional caption
  optional bytes  fileSha256 = 4;       // File hash
  optional uint64 fileLength = 5;       // Size in bytes
  optional uint32 height = 6;
  optional uint32 width = 7;
  optional bytes  mediaKey = 8;         // Encryption key
  optional bytes  fileEncSha256 = 9;    // Encrypted file hash
  optional bytes  jpegThumbnail = 16;   // Inline thumbnail
  optional string directPath = 18;
  optional ContextInfo contextInfo = 17;
}
```

---

## 3. Message Containing a Document (PDF)

### Signal

```protobuf
message AttachmentPointer {
  optional string  contentType = 2;     // "application/pdf"
  optional string  fileName = 7;        // "document.pdf"
  optional uint32  size = 4;
  optional bytes   key = 3;             // Encryption key
  optional bytes   digest = 6;
  enum Flags {
    VOICE_MESSAGE = 1;
    BORDERLESS    = 2;
    GIF           = 4;
  }
  optional uint32 flags = 10;
}
```

---

### Telegram

```tl
messageMediaDocument#4cf4d72d 
  flags:# 
  nopremium:flags.3?true 
  spoiler:flags.4?true 
  document:flags.0?Document 
  ttl_seconds:flags.2?int 
  = MessageMedia;

document#8fd4c4d8 
  flags:# 
  id:long 
  access_hash:long 
  file_reference:bytes 
  date:int 
  mime_type:string 
  size:long 
  dc_id:int 
  attributes:Vector<DocumentAttribute> 
  = Document;

documentAttributeFilename#15590068 
  file_name:string 
  = DocumentAttribute;
```

---

### Matrix

```json
{
  "type": "m.room.message",
  "content": {
    "msgtype": "m.file",
    "body": "document.pdf",
    "filename": "document.pdf",
    "info": {
      "mimetype": "application/pdf",
      "size": 104857
    },
    "url": "mxc://example.org/ABCdef123"
  }
}
```

---

### WhatsApp

```protobuf
message DocumentMessage {
  optional string url = 1;
  optional string mimetype = 2;         // "application/pdf"
  optional string title = 3;            // Display title
  optional bytes  fileSha256 = 4;
  optional uint64 fileLength = 5;
  optional uint32 pageCount = 6;        // PDF page count
  optional bytes  mediaKey = 7;
  optional string fileName = 8;         // "document.pdf"
  optional bytes  fileEncSha256 = 9;
  optional string directPath = 10;
  optional bytes  jpegThumbnail = 16;   // Preview thumbnail
  optional ContextInfo contextInfo = 17;
}
```

---

## 4. Message with Formatted Text

### Signal

Signal supports a limited set of text styles via `BodyRange`:

```protobuf
message DataMessage {
  optional string body = 1;
  repeated BodyRange bodyRanges = 22;
}

message BodyRange {
  optional uint32 start = 1;            // Start position
  optional uint32 length = 2;           // Length of range
  
  enum Style {
    NONE = 0;
    BOLD = 1;
    ITALIC = 2;
    SPOILER = 3;
    STRIKETHROUGH = 4;
    MONOSPACE = 5;
  }
  optional Style style = 3;
  optional string mentionAci = 4;       // For @mentions
}
```

Example:
```json
{
  "body": "Hello World with bold text!",
  "bodyRanges": [
    {"start": 17, "length": 4, "style": "BOLD"}
  ]
}
```

---

### Telegram

Telegram uses `MessageEntity` for rich formatting:

```tl
messageEntityBold#bd610bc9 offset:int length:int = MessageEntity;
messageEntityItalic#826f8b60 offset:int length:int = MessageEntity;
messageEntityCode#28a20571 offset:int length:int = MessageEntity;
messageEntityPre#73924be0 offset:int length:int language:string = MessageEntity;
messageEntityUnderline#9c4e7e8b offset:int length:int = MessageEntity;
messageEntityStrike#bf0693d4 offset:int length:int = MessageEntity;
messageEntitySpoiler#32ca960f offset:int length:int = MessageEntity;
messageEntityTextUrl#76a6d327 offset:int length:int url:string = MessageEntity;
messageEntityMentionName#dc7b1140 offset:int length:int user_id:long = MessageEntity;
messageEntityCustomEmoji#c8cf05f8 offset:int length:int document_id:long = MessageEntity;
```

Example:
```json
{
  "message": "Hello World with bold text!",
  "entities": [
    {"_": "messageEntityBold", "offset": 17, "length": 4}
  ]
}
```

---

### Matrix

Matrix supports HTML formatting in `formatted_body`:

```json
{
  "type": "m.room.message",
  "content": {
    "msgtype": "m.text",
    "body": "Hello World with bold text!",
    "format": "org.matrix.custom.html",
    "formatted_body": "Hello World with <strong>bold</strong> text!"
  }
}
```

Supported HTML tags include: `<strong>`, `<em>`, `<del>`, `<code>`, `<pre>`, `<blockquote>`, `<a>`, `<span>` (for mentions/pills).

---

### WhatsApp

WhatsApp uses inline Markdown-style formatting in the message text:

- **Bold**: `*text*`
- **Italic**: `_text_`
- **Strikethrough**: `~text~`
- **Monospace**: ` ```text``` `

The client renders these; no separate format field exists in the protobuf.

---

## 5. Emoji Reactions to a Message

### Signal

```protobuf
message DataMessage {
  optional Reaction reaction = 11;
}

message Reaction {
  optional string emoji = 1;            // The emoji character
  optional bool   remove = 2;           // True to remove reaction
  optional string targetAuthorAci = 3;  // Author of target message
  optional uint64 targetSentTimestamp = 4;  // Target message timestamp
}
```

Example:
```json
{
  "reaction": {
    "emoji": "👍",
    "remove": false,
    "targetAuthorAci": "abc123-def456-...",
    "targetSentTimestamp": 1704067200000
  }
}
```

---

### Telegram

```tl
reactionEmoji#1b2286b8 emoticon:string = Reaction;
reactionCustomEmoji#8935fc73 document_id:long = Reaction;
reactionPaid#523da4eb = Reaction;

messageReactions#a339f0b 
  flags:# 
  min:flags.0?true 
  can_see_list:flags.2?true 
  results:Vector<ReactionCount> 
  recent_reactions:flags.1?Vector<MessagePeerReaction> 
  = MessageReactions;

reactionCount#a3d1cb80 
  flags:# 
  chosen_order:flags.0?int 
  reaction:Reaction 
  count:int 
  = ReactionCount;
```

Example:
```json
{
  "reactions": {
    "results": [
      {"reaction": {"emoticon": "👍"}, "count": 5, "chosen": true},
      {"reaction": {"emoticon": "❤️"}, "count": 3}
    ]
  }
}
```

---

### Matrix

Reactions use the `m.reaction` event type with relations:

```json
{
  "type": "m.reaction",
  "sender": "@alice:example.org",
  "content": {
    "m.relates_to": {
      "rel_type": "m.annotation",
      "event_id": "$target_event_id",
      "key": "👍"
    }
  }
}
```

---

### WhatsApp

```protobuf
message ReactionMessage {
  optional MessageKey key = 1;          // Reference to target message
  optional string text = 2;             // Emoji character
  optional string groupingKey = 3;
  optional int64  senderTimestampMs = 4;
}

message MessageKey {
  optional string remoteJid = 1;        // Chat ID
  optional bool   fromMe = 2;
  optional string id = 3;               // Message ID
  optional string participant = 4;      // Sender in group
}
```

---

## 6. Reply to a Message

### Signal

Signal uses `Quote` inside `DataMessage` for replies:

```protobuf
message DataMessage {
  optional Quote quote = 8;
}

message Quote {
  optional uint64           id = 1;             // Original message timestamp
  optional string           authorAci = 2;      // Original sender UUID
  optional string           text = 3;           // Quoted text
  repeated QuotedAttachment attachments = 4;    // Quoted attachments
  
  message QuotedAttachment {
    optional string            contentType = 1;
    optional string            fileName = 2;
    optional AttachmentPointer thumbnail = 3;
  }
}
```

Example:
```json
{
  "body": "I agree with this!",
  "quote": {
    "id": 1704067100000,
    "authorAci": "abc123-...",
    "text": "Original message text"
  }
}
```

---

### Telegram

```tl
messageReplyHeader#6eebcabd 
  flags:# 
  reply_to_scheduled:flags.2?true 
  forum_topic:flags.3?true 
  quote:flags.9?true 
  reply_to_msg_id:flags.4?int 
  reply_to_peer_id:flags.0?Peer 
  reply_from:flags.5?MessageFwdHeader 
  reply_to_top_id:flags.1?int 
  quote_text:flags.6?string 
  quote_entities:flags.7?Vector<MessageEntity>
  quote_offset:flags.10?int 
  = MessageReplyHeader;

message#9815cec8 ... 
  reply_to:flags.3?MessageReplyHeader 
  ... = Message;
```

Example:
```json
{
  "message": "I agree!",
  "reply_to": {
    "reply_to_msg_id": 12344,
    "quote_text": "Original message text"
  }
}
```

---

### Matrix

Matrix uses `m.relates_to` with `m.in_reply_to`:

```json
{
  "type": "m.room.message",
  "content": {
    "msgtype": "m.text",
    "body": "> <@bob:example.org> Original message\n\nI agree!",
    "format": "org.matrix.custom.html",
    "formatted_body": "<mx-reply><blockquote><a href=\"...\">In reply to</a> <a href=\"...\">@bob:example.org</a><br>Original message</blockquote></mx-reply>I agree!",
    "m.relates_to": {
      "m.in_reply_to": {
        "event_id": "$original_event_id"
      }
    }
  }
}
```

---

### WhatsApp

WhatsApp uses `ContextInfo` for replies:

```protobuf
message ContextInfo {
  optional string stanzaId = 1;         // ID of quoted message
  optional string participant = 2;       // Sender of quoted message
  optional Message quotedMessage = 3;   // Full quoted message content
  optional string remoteJid = 4;        // Chat where original was sent
  repeated string mentionedJid = 15;
  optional string conversionSource = 18;
  optional bytes  conversionData = 19;
}

message ExtendedTextMessage {
  optional string text = 1;
  optional ContextInfo contextInfo = 17;
}
```

---

## 7. Quoting Parts of a Message

### Signal

Signal's `Quote` can include partial text (the client typically shows the full quoted text):

```protobuf
message Quote {
  optional string text = 3;             // Can be partial/selected text
}
```

---

### Telegram

Telegram supports selective quoting with `quote_text` and `quote_offset`:

```tl
messageReplyHeader#6eebcabd 
  ...
  quote_text:flags.6?string             // Selected quote portion
  quote_entities:flags.7?Vector<MessageEntity>  // Formatting in quote
  quote_offset:flags.10?int             // Offset into original message
  = MessageReplyHeader;
```

---

### Matrix

Matrix doesn't have native partial quoting, but clients can include selected text in the HTML fallback:

```json
{
  "content": {
    "body": "> selected portion of original text\n\nMy response",
    "formatted_body": "<mx-reply>...<br>selected portion of original text</mx-reply>My response"
  }
}
```

---

### WhatsApp

WhatsApp includes the full `quotedMessage` in `ContextInfo`. Partial quoting is a client-side presentation choice.

---

## 8. GPS Location / Live Location

### Signal

```protobuf
message DataMessage {
  optional SharedContact contact = 9;   // Not used for location
  // Signal doesn't have a native location message type
  // Location sharing is done via external links/maps
}
```

Note: Signal doesn't have a built-in location sharing feature in the protocol.

---

### Telegram

**Static Location:**
```tl
messageMediaGeo#56e0d474 geo:GeoPoint = MessageMedia;

geoPoint#b2a2f663 
  flags:# 
  long:double 
  lat:double 
  access_hash:long 
  accuracy_radius:flags.0?int 
  = GeoPoint;
```

**Live Location:**
```tl
messageMediaGeoLive#b940c666 
  flags:# 
  geo:GeoPoint 
  heading:flags.0?int                   // Direction (0-359 degrees)
  period:int                            // Update duration in seconds
  proximity_notification_radius:flags.1?int 
  = MessageMedia;
```

Example:
```json
{
  "media": {
    "_": "messageMediaGeoLive",
    "geo": {
      "lat": 52.520008,
      "long": 13.404954,
      "accuracy_radius": 50
    },
    "heading": 90,
    "period": 3600
  }
}
```

---

### Matrix

**Static Location:**
```json
{
  "type": "m.room.message",
  "content": {
    "msgtype": "m.location",
    "body": "My location",
    "geo_uri": "geo:52.520008,13.404954",
    "info": {
      "thumbnail_url": "mxc://example.org/map_preview",
      "thumbnail_info": {
        "mimetype": "image/png",
        "w": 300,
        "h": 300
      }
    }
  }
}
```

**Live Location (MSC3489):**
```json
{
  "type": "m.beacon_info",
  "state_key": "@alice:example.org",
  "content": {
    "description": "My live location",
    "live": true,
    "timeout": 3600000,
    "m.ts": 1704067200000,
    "m.asset": {
      "type": "m.self"
    }
  }
}

// Location updates sent as:
{
  "type": "m.beacon",
  "content": {
    "m.relates_to": {
      "rel_type": "m.reference",
      "event_id": "$beacon_info_event_id"
    },
    "m.location": {
      "uri": "geo:52.520008,13.404954;u=20"
    },
    "m.ts": 1704067260000
  }
}
```

---

### WhatsApp

**Static Location:**
```protobuf
message LocationMessage {
  optional double  degreesLatitude = 1;
  optional double  degreesLongitude = 2;
  optional string  name = 3;            // Place name
  optional string  address = 4;         // Street address
  optional string  url = 5;             // Maps URL
  optional bool    isLive = 6;
  optional uint32  accuracyInMeters = 7;
  optional float   speedInMps = 8;
  optional uint32  degreesClockwiseFromMagneticNorth = 9;
  optional string  comment = 11;
  optional bytes   jpegThumbnail = 16;  // Map preview
  optional ContextInfo contextInfo = 17;
}
```

**Live Location:**
```protobuf
message LiveLocationMessage {
  optional double  degreesLatitude = 1;
  optional double  degreesLongitude = 2;
  optional uint32  accuracyInMeters = 3;
  optional float   speedInMps = 4;
  optional uint32  degreesClockwiseFromMagneticNorth = 5;
  optional string  caption = 6;
  optional int64   sequenceNumber = 7;
  optional uint32  timeOffset = 8;      // Seconds from start
  optional bytes   jpegThumbnail = 16;
  optional ContextInfo contextInfo = 17;
}
```

---

## 9. Voting / Poll Message

### Signal

Signal doesn't have a native poll feature in the public protocol.

---

### Telegram

```tl
poll#86e18161 
  id:long 
  flags:# 
  closed:flags.0?true 
  public_voters:flags.1?true 
  multiple_choice:flags.2?true 
  quiz:flags.3?true 
  question:TextWithEntities 
  answers:Vector<PollAnswer> 
  close_period:flags.4?int 
  close_date:flags.5?int 
  = Poll;

pollAnswer#ff16e2ca 
  text:TextWithEntities 
  option:bytes 
  = PollAnswer;

pollResults#7adf2420 
  flags:# 
  min:flags.0?true 
  results:flags.1?Vector<PollAnswerVoters> 
  total_voters:flags.2?int 
  recent_voters:flags.3?Vector<Peer> 
  solution:flags.4?string 
  solution_entities:flags.4?Vector<MessageEntity> 
  = PollResults;

messageMediaPoll#4bd6e798 
  poll:Poll 
  results:PollResults 
  = MessageMedia;
```

Example:
```json
{
  "media": {
    "_": "messageMediaPoll",
    "poll": {
      "id": 5142579120000000,
      "question": {"text": "What's your favorite color?"},
      "answers": [
        {"text": {"text": "Red"}, "option": "0"},
        {"text": {"text": "Blue"}, "option": "1"},
        {"text": {"text": "Green"}, "option": "2"}
      ],
      "multiple_choice": false,
      "quiz": false
    },
    "results": {
      "total_voters": 42,
      "results": [
        {"option": "0", "voters": 15, "chosen": true},
        {"option": "1", "voters": 20},
        {"option": "2", "voters": 7}
      ]
    }
  }
}
```

---

### Matrix

Matrix polls use MSC3381:

**Poll Start:**
```json
{
  "type": "m.poll.start",
  "content": {
    "m.poll": {
      "kind": "m.disclosed",
      "max_selections": 1,
      "question": {
        "m.text": "What's your favorite color?"
      },
      "answers": [
        {"id": "red", "m.text": "Red"},
        {"id": "blue", "m.text": "Blue"},
        {"id": "green", "m.text": "Green"}
      ]
    },
    "m.text": "What's your favorite color?\n1. Red\n2. Blue\n3. Green"
  }
}
```

**Poll Response:**
```json
{
  "type": "m.poll.response",
  "content": {
    "m.relates_to": {
      "rel_type": "m.reference",
      "event_id": "$poll_start_event_id"
    },
    "m.selections": ["blue"]
  }
}
```

**Poll End:**
```json
{
  "type": "m.poll.end",
  "content": {
    "m.relates_to": {
      "rel_type": "m.reference",
      "event_id": "$poll_start_event_id"
    },
    "m.poll.results": {
      "red": 15,
      "blue": 20,
      "green": 7
    },
    "m.text": "Poll closed. Blue wins with 20 votes!"
  }
}
```

---

### WhatsApp

```protobuf
message PollCreationMessage {
  optional bytes   encKey = 1;          // Encryption key for votes
  optional string  name = 2;            // Poll question
  repeated Option  options = 3;
  optional uint32  selectableOptionsCount = 4;  // Max selections
  optional ContextInfo contextInfo = 5;
  
  message Option {
    optional string optionName = 1;
  }
}

message PollUpdateMessage {
  optional MessageKey pollCreationMessageKey = 1;  // Reference to poll
  optional PollVote   vote = 2;
  optional PollMetadata metadata = 3;
  optional int64   senderTimestampMs = 4;
  
  message PollVote {
    repeated bytes selectedOptions = 1;  // SHA256 of selected option names
  }
}
```

---

## 10. User Mentions in Text

### Signal

Signal uses `BodyRange` with `mentionAci`:

```protobuf
message BodyRange {
  optional uint32 start = 1;
  optional uint32 length = 2;
  optional string mentionAci = 4;       // UUID of mentioned user
}

message DataMessage {
  optional string body = 1;             // Contains placeholder like "￼"
  repeated BodyRange bodyRanges = 22;
}
```

Example:
```json
{
  "body": "Hello ￼, how are you?",
  "bodyRanges": [
    {"start": 6, "length": 1, "mentionAci": "abc123-def456-..."}
  ]
}
```

---

### Telegram

```tl
messageEntityMention#fa04579d offset:int length:int = MessageEntity;
messageEntityMentionName#dc7b1140 offset:int length:int user_id:long = MessageEntity;

inputMessageEntityMentionName#208e68c9 
  offset:int 
  length:int 
  user_id:InputUser 
  = MessageEntity;
```

Example:
```json
{
  "message": "Hello @alice, how are you?",
  "entities": [
    {"_": "messageEntityMention", "offset": 6, "length": 6}
  ]
}

// Or with resolved user ID:
{
  "message": "Hello Alice, how are you?",
  "entities": [
    {"_": "messageEntityMentionName", "offset": 6, "length": 5, "user_id": 123456789}
  ]
}
```

---

### Matrix

Matrix uses special HTML pills for mentions:

```json
{
  "type": "m.room.message",
  "content": {
    "msgtype": "m.text",
    "body": "Hello @alice:example.org, how are you?",
    "format": "org.matrix.custom.html",
    "formatted_body": "Hello <a href=\"https://matrix.to/#/@alice:example.org\">Alice</a>, how are you?"
  }
}
```

The Matrix ID format is `@localpart:server.domain`.

---

### WhatsApp

```protobuf
message ContextInfo {
  repeated string mentionedJid = 15;    // List of mentioned JIDs
}

message ExtendedTextMessage {
  optional string text = 1;             // Contains @phone_number
  optional ContextInfo contextInfo = 17;
}
```

Example:
```json
{
  "extendedTextMessage": {
    "text": "Hello @14155551234, how are you?",
    "contextInfo": {
      "mentionedJid": ["14155551234@s.whatsapp.net"]
    }
  }
}
```

WhatsApp JID format: `phone_number@s.whatsapp.net` (users) or `id@g.us` (groups).

---

## Summary Comparison Table

| Feature | Signal | Telegram | Matrix | WhatsApp |
|---------|--------|----------|--------|----------|
| **Protocol** | Protobuf | TL/MTProto | JSON | Protobuf |
| **Text Message** | DataMessage.body | message.message | m.room.message (m.text) | Message.conversation |
| **Images** | AttachmentPointer | messageMediaPhoto | m.image + mxc:// URL | ImageMessage |
| **Documents** | AttachmentPointer | messageMediaDocument | m.file | DocumentMessage |
| **Formatting** | BodyRange (style) | MessageEntity | HTML formatted_body | Markdown in text |
| **Reactions** | Reaction message | messageReactions | m.reaction (annotation) | ReactionMessage |
| **Replies** | Quote | MessageReplyHeader | m.relates_to.m.in_reply_to | ContextInfo.quotedMessage |
| **Location** | Not native | messageMediaGeo | m.location (geo_uri) | LocationMessage |
| **Live Location** | Not native | messageMediaGeoLive | m.beacon_info + m.beacon | LiveLocationMessage |
| **Polls** | Not native | poll + pollResults | m.poll.start/response/end | PollCreationMessage |
| **User Mentions** | BodyRange.mentionAci | messageEntityMentionName | HTML pills with matrix.to | ContextInfo.mentionedJid |
| **User ID Format** | UUID (ACI) | Numeric user_id | @localpart:server | phone@s.whatsapp.net |

---

## Notes on Availability

- **Signal**: The protocol is open source, but the exact .proto files are distributed across multiple repositories and versions.

- **Telegram**: The TL schema is publicly documented at [core.telegram.org/schema](https://core.telegram.org/schema).

- **Matrix**: The specification is fully open at [spec.matrix.org](https://spec.matrix.org), including all event types and their JSON schemas.

- **WhatsApp**: While the protocol is proprietary, the protobuf schemas have been reverse-engineered and are available in community projects. WhatsApp uses the Signal Protocol for encryption but has its own message format layer.
