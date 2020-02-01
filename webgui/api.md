# API

Every HTTP endpoint must be namespaced by `/api/`.
Every Record MUST have an `id`, and the combination of `id` and `type` *MUST* be the global identify of the record.

This is the list of models:
- user
- chatRoom
- directMessage

## user

**get all users**
`/api/users`
Either must return all users or throw HTTP 500. May be useful in the beginning, but probably should not be used by the frontend later.

**search users**

It must be possible to specify `filter[searchTerm]=XX` as query param to search users.
It must be possible to specify `page[limit]=X` to limit the number of returned users.

In combination this can be used to search for users:

`/api/users?filter[searchTerm]=XX&page[limit]=30`
Must return up to 30 users that match ther search term XX

This MUST be possible without an Authorization header to search for the user during login.

**get a single user**
`/api/users/X`
Must return the user with id X.

## chatRoom

**get all chat rooms**

`/api/chat-rooms`
Must return all chat rooms of the current user
This will probably only be used for debuggung purposes. The frontend will mostly specify additional query params to only load the recent chat rooms.

**get recent chat rooms**

It must be possible to specify `sort=last-activity` as query param to sort the chat rooms.
It must be possible to specify `limit=X` to limit the number of returned chatrooms.

In combination this can be used to load the most recent chat rooms:

`/api/chat-rooms?sort=last-activity&limit=10`
-> Must return the last 10 


**get a single chat room**

`/api/chat-rooms/X`
Must return the chat room with id X.

**chat room attributes**

A chat room has the following attributes:
- `kind`: is either `direct` or `group`
- `name`: when `"kind": "group"` this can be an abitary string

A chat room has the following relationships:
- `members`: to many `user`. If `"kind": "direct"` the list of users should be specified with `data`, and the other party should be included. If `"kind": "group"` only the links should be sent.
- `messages`: to many `chat-message`. The frontend will *not* load this relation or only for debuggung. It will instead use direct filtering on the `chatMessage`. No `data` should be sent.
- `last-message`: a short reference to the last message used for the preview. This SHOULD usually be sideloaded with `included`.

**example chat room response (for a single chat room)**
```
{
  "data": {
    "id": "room-1",
    "type":"chat-rooms",
    "attributes": {
      "kind": "group",
      "name": "qaulnet developers"
    },
    "relationships": {
      "members": {
        "self": "/chat-rooms/room-1/relationships/members",
        "related": "/chat-rooms/room-1/members"
      },
      "messages": {
        "self": "/chat-rooms/room-1/relationships/members",
        "related": "/chat-rooms/room-1/members"
      },
      "last-message": {
        "data": { "type": "chat-message", id: "message-1" }
      }
    }
  },
  "included": [{
    "type": "chat-message",
    "id": "message-1",
    ... as specified below in the chatMessage section ...
  }]
}
```

## chatMessage

**get all chat messages**
`/api/chat-messages`

It should not be possible to load all chat messages. So this must return a `HTTP 400`.

**loading the last messages of a chat room**

It must be possible to specify `filter[room]=X` to filter messages to a gived chat room.
It must be possible to specify `sort=timestamp` as query param to sort the messages.
It must be possible to specify `page[limit]=X` to limit the number of returned messages.
It must be possible to specify `page[offset]=X` to specify a offset.

With this it is possible to load the 10 most recent messages:

`/api/chat-messages?filter[room]=room-1&sort=timestamp&page[limit]=10`

and to load the next 10 messages before them when the user scolls up:

`/api/chat-messages?filter[room]=room-1&sort=timestamp&page[limit]=10&page[offset]=10`

**get a single chat message**

`/api/chat-messages/X`
Must return the chat room with id X.

**chat message attributes**

A chat message has the following attributes:
- `message`: this can be an abitary string
- `timestamp`: a timestamp formatted as ISO8601 when the message was first seen by the backend. This should not be a remote timestamp but always a locally created one.
- `unread`: a boolean. It will always be false for messages sent by the user itself, and otherwise always true until set to false by the frontend.

A chat message has the following relationships:
- `room`: to one `chat-rooms`. The `id` should be specified by the backend but the target record should not be included.
- `sender`: to one `user`. The `id` should be specified by the backend but the target record should not be included.

**example chat message response (for a filtered response with only 1 record in the response)**

```
{
  "data": [{
    "type:" "chat-message",
    "id": "msg-1",
    "attributes": {
      "message": "Welcome to our new group",
      "timestamp": "2020-02-01T18:56:14.368Z",
      "unread": true
    },
    "relationships": {
      "room": {
        "data": { "type": "chat-room", "id": "room-1" }
      },
      "sender": {
        "data": { "type": "user", "id": "user2" }
      }
    }
  }]
}
```

**marking a chat message as read**

`HTTP PATCH /api/chat-message/msg-1` will be called with a `chat-message` resource object where `"unread": false` is. Other changes must either return a `HTTP 400` or must be rolled back and the correct end result must be returned with a `HTTP 200`. This should never happen.
If there have been no other changes then the response can be `HTTP 204`.

Example request payload. The result should be `HTTP 204` and no body.

```
{
  "data": {
    "type:" "chat-message",
    "id": "msg-1",
    "attributes": {
      "message": "Welcome to our new group",
      "timestamp": "2020-02-01T18:56:14.368Z",
      "unread": false
    },
    "relationships": {
      "room": {
        "data": { "type": "chat-room", "id": "room-1" }
      },
      "sender": {
        "data": { "type": "user", "id": "user2" }
      }
    }
  }
}
```

**creating a new message**

`HTTP POST /api/chat-message` will be called with a `chat-message` resource object.
The `timestamp` will be there, but MAY be corrected by the backend.
The `sender` will always be the current user or null/not sent, if not an `HTTP 400` should be returned. If the `sender` is not sent or null the backend should find the correct user based on the authorization and add it.
There will always be a `room`, if not a `HTTP 400` should be returned.
There will be no `id` so the backend, so the backend should respond with the full resource object.
This response will always include the `sender`, even if it was added by the backend.

Example request payload:

```
{
  "data": {
    "type:" "chat-message",
    "attributes": {
      "message": "Nice to be here",
      "timestamp": "2020-02-01T19:59:14.368Z",
      "unread": false
    },
    "relationships": {
      "room": {
        "data": { "type": "chat-room", "id": "room-1" }
      }
    }
  }
}
```

Example response payload:
```
{
  "data": {
    "type:" "chat-message",
    "attributes": {
      "message": "Nice to be here",
      "timestamp": "2020-02-01T19:59:14.368Z",
      "unread": false
    },
    "relationships": {
      "room": {
        "data": { "type": "chat-room", "id": "room-1" }
      },
      "sender": {
        "data": { "type": "user", "id": "user1" }
      }
    }
  }
}
```


