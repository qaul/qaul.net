# User HTTP-API Interface

The user api reflects all user accounts on the local the node.
All information except password is available to other users.

## User Model

```json
{
    "user": {
        "avatar": null,
        "bio": {
            "key":"VALUE"
        },
        "display_name": null,
        "id": "USER_ID",
        "real_name": null,
        "services": []
    }
}
```


## Create User

To create a user, one must only send a password.

`POST /http/user`

Request payload:

```json
{
    "pw":"PASSWORD"
}
```

Response payload: user model

```json
{
    "user": {
        "avatar": null,
        "bio": {},
        "display_name": null,
        "id": "USER_ID",
        "real_name": null,
        "services": []
    }
}
```


### Modify User

`POST /http/user/USER_ID`

Request payload: 
Only the changed fields are sent in the diff format.

```json
{
    "display_name": {
        "set": "testuser"
    },
    "real_name": {
        "set": "My Real Name"
    }
}
```

Response: same as create user


## Get Information of a Specific User

`GET /http/user/USER_ID`

Response: same as create user


## Delete a User

`DELETE /http/user/USER_ID`

Request payload: 
There needs to be sent the field "purge", which can be set to the string values "true" or "false".
If the value is "true", all data of the user is deleted from the node. 

```json
{
    "purge": "true"
}
```

Response: empty response with http code 204


## Query Users

Get an array of all users on the local node.

`GET /http/user`


Response payload: array of users

```json
{
    "users": [
        {
            "avatar": null,
            "bio": {},
            "display_name": "nameofuser1",
            "id": "USER_ID",
            "real_name": "Real Name",
            "services": []
        },
        {
            "avatar": null,
            "bio": {},
            "display_name": null,
            "id": "USER_ID",
            "real_name": null,
            "services": []
        }
    ]
}
```
