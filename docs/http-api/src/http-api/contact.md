# Contact HTTP-API Interface

The contact api reflects all users on all remote nodes. The users on the local node are not shown.
The Model of the contact is the same as the user model.

## Contact Model

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


## Query Contacts

Get an array of all discovered users on all remote nodes.

`GET /http/contact`


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
