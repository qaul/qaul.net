# Chat Service HTTP-API Interface

The chat service consists of two models, the `chat_room` and the `chat_message`. First there needs to be created a `chat_room` with as many participants as desired. Afterwards one can send a `chat_message` to a `chat_room`.

## Chat Models

Chat room model

```json
{
    "chat_room": {
        "create_time": "2020-06-22T13:19:41.143402587Z",
        "id": "CHAT_ROOM_ID",
        "name": "CHAT ROOM NAME",
        "users": [
            "USER_ID",
            "USER_ID"
        ]
    }
}
```

Chat message model

```json
{
    "chat_message": [
        {
            "content": "TEXT_MESSAGE",
            "id": "CHAT_MESSAGE_ID",
            "room": {
                "Id": "CHAT_ROOM_ID"
            },
            "sender": "USER_ID",
            "timestamp": "2020-06-22T13:26:37.478641925Z"
        }
    ]
}
```

## Chat Room

### Create Chat Room

To create a chat room one can send an array of user id's and an optional chat room name.

`POST /http/chat_room`

Request payload:

```json
{
    "name": "CHAT ROOM NAME",
    "users": [
        "USER_ID",
        "USER_ID"
    ]
}
```

Response payload:

```json
{
    "chat_room":{
        "id":"CHAT_ROOM_ID",
        "users":[
            "USER_ID",
            "USER_ID",
            "SENDER_USER_ID"
        ],
        "name":"CHAT ROOM NAME",
        "create_time":"2020-06-22T14:21:24.311261408Z"
    }
}
```


### Modify Chat Room

Change

`PATCH /http/chat_room/CHAT_ROOM_ID`

Request payload: 
Only the changed fields are sent in the diff format.

```json
{
    "id": "CHAT_ROOM_ID",
    "set": {
        "name": "My Room Name"
    }
}
```

Response payload: same as create chat room.


### Get Chat Room

Retrieve the information of a specific chat room by it's id.

`GET /http/chat_room/CHAT_ROOM_ID`

Response payload: same as create chat room.


### List all Chat Rooms

`GET /http/chat_room`


Response payload: Array of chat rooms

```json
{
    "chat_rooms": [
        "ARRAY_OF_CHAT_ROOM_OBJECTS"
    ]
}
```

## Chat Message

### Create a Chat Message

`POST /http/chat_message`

Request payload:

```json
{
    "room": "CHAT_ROOM_ID",
    "text": "MESSAGE_TEXT"
}
```

Response payload: returns created chat message

```json
{
    "chat_message": [
        {
            "content": "MESSAGE_TEXT",
            "id": "CHAT_MESSAGE_ID",
            "room": {
                "Id": "CHAT_ROOM_ID"
            },
            "sender": "USER_ID",
            "timestamp": "2020-06-22T16:46:03.317992177Z"
        }
    ]
}
```

### List all Chat Messages of a Specific Chat Room

`GET /http/chat_message?chat_room=CHAT_ROOM_ID`


Response payload: Array of chat messages

```json
{
    "chat_message": [
        "ARRAY_OF_CHAT_MESSAGE_OBJECTS"
    ]
}
```
