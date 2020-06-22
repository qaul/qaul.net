# JSON-RPC API

**The JSON-RPC API consists of a single http endpoint. Over which all RPC interaction with the qaul.net daemon happens.**


## General Concept

The JSON-RPC API's prefix is `/rpc`.
All requests are sent as POST requests to this endpoint and will return with http response code 200.

### Request

The general JSON-RPC JSON structure looks like this:

```json
{
    "id": "RANDOM_REQUEST_ID",
    "kind": "MODEL_NAME",
    "method": "METHOD_NAME",
    "data": {
        "field_name": "FIELD_VALUE",
        "field_name": "FIELD_VALUE"
    },
    "auth": {
        "id": "USER_ID",
        "token": "SESSION_TOKEN"
    }
}
```

The method name can have one of those values:

* _create_: When you're creating a new entry.
* _get_: To request one entry by ID.
* _modify_: To modify an entry.
* _delete_: To delete one entry by ID.
* _list_: To receive a list of queried entries.

There are some special authentication methods:

* _login_: to start an authenticated session
* _logout_: to terminate an authenticated session
* _validate_: to validate a session token


#### Request Data Payload

In order to create or modify an entry we send a JSON payload with the request.

To create an entry we send _POST_ a request with the content of some of the fields. Please be aware, that some fields are mandatory and that not all the fields can be set during the creation of an entry. To find out about the specific use please check the documentation of the model.

Create entry payload example:

```JSON
{
    "field_name": "FIELD_VALUE"
}
```

To modify and entry we send a _PATCH_ request with only the modified fields in a specific structure. A _set_ structure to set or modify a field, and an _unset_ structure to delete the content of a field.

Modify entry payload example:

```JSON
{
    "field_name": { "set": "FIELD_VALUE" },
    "field_name": "unset"
}
```

### Response

The general response payload structure looks like this:

```json
{
    "id": "RANDOM_REQUEST_ID",
    "kind": "MODEL_NAME",
    "method": "METHOD_NAME",
    "data": {
        "field_name": "FIELD_VALUE"
    }
}
```

The "data" field has several specific structures that are documented in the following.


#### Response Data Payload per Action

The response payloads are model specific and should be checked in the model documentations. however there are some general rules how a payload looks.

When you're requesting a _list_ of entries, you're receiving the following structure:

```json
{
    "id": "RANDOM_REQUEST_ID",
    "kind": "MODEL_NAME",
    "method": "METHOD_NAME",
    "data": {
        "model_name": [
            {
                "id": "ITEM_ID",
                "field_name": "FIELD_CONTENT"
            },
            {
                "id": "ITEM_ID",
                "field_name": "FIELD_CONTENT"
            }
        ]
    }
}
```

When you _get_ one specific entry by it's ID, or when you _create_ or _modify_ an entry you're receiving the following structure:

```json
{
    "id": "RANDOM_REQUEST_ID",
    "kind": "MODEL_NAME",
    "method": "METHOD_NAME",
    "data": {
        "model_name": {
            "id": "ITEM_ID",
            "field_name": "FIELD_CONTENT"
        }
    }
}
```

Sometimes you're only receiving a success message as data payload.

```json
{
    "id": "RANDOM_REQUEST_ID",
    "kind": "MODEL_NAME",
    "method": "METHOD_NAME",
    "data": {
        "type": "success"
    }
}
```

#### Error Response Data Payload

On error you're receiving an error message as data payload.

```json
{
    "id": "RANDOM_REQUEST_ID",
    "kind": "MODEL_NAME",
    "method": "METHOD_NAME",
    "data": {
        "error": "ERROR_MESSAGE"
    }
}
```
