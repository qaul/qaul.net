# HTTP API

**The HTTP API uses specific http URLs, http request codes and http response codes for each function. It is the API used by the webGUI.**


## General Concept

The HTTP API's prefix is `/http`.

Model names and field id's use snake case in singular. 
URL's always use the model name in singular.
If the return is an array of multiple objects or object id's then the field name is in plural.


### Request

#### URL Concept

The general URL concept looks like this:
`/http/model_name/ID?QUERY_STRING`

To list all entries:
`GET /http/model_name`

To query the entries:
`GET /http/model_name?QUERY_STRING`

To create a new entry:
`POST /http/model_name`

To get a single entry:
`GET /http/model_name/ID`

To modify an entry:
`PATCH /http/model_name/ID`

To delete an entry:
`DELETE /http/model_name/ID`


#### Authentication

The Authentication Token is sent in the `Authorization` field of the http request header.

```json
Authorization:{"id":"USER_ID","token":"SESSION_TOKEN"}
```

#### Payload

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

#### HTTP Response Codes

HTTP response codes on success:

* 200 (OK): default response code, when everything went well and you're receiving a payload.
* 204 (No Content): response when there is no payload, which you get on delete and sometimes on modify too.

HTTP response codes on error:

* 400 (Bad Request): wrong user input (and any other unhandled error)
* 401 (Unauthorized): if the user is not authorized to perform this task. This either means that you're authorization token is invalid or that you're trying to access some other users data.
* 500 (Internal Server Error): The server was not able to parse the request payload.

#### Response Payload per Action

The response payloads are model specific and should be checked in the model documentations. however there are some general rules how a payload looks.

When you're receiving a list of entries, you're receiving the following structure:

```json
{
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
```

When you get one specific entry by it's ID, or when you create a new entry you're receiving the following structure:

```json
{
    "model_name": {
        "id": "ITEM_ID",
        "field_name": "FIELD_CONTENT"
    }
}
```


#### Error Response Payload

On error you're receiving an error message as payload.

```json
{
    "error": "ERROR_MESSAGE"
}
```
