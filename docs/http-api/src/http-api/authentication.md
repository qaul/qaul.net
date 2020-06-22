# Authentication HTTP-API Interface

Each client needs to authenticate a session in order to be able to interact with the qaul.net daemon and send the session authentication in the http `Authorization` header.

## Http Authorization Header

The session authentication needs to be sent `Authorization` field of the http request header.

```json
Authorization:{"id":"USER_ID","token":"SESSION_TOKEN"}
```


## Unauthenticated Functions

There are a few HTTP-API calls can be made without sending a session authorization. This is the list of the calls. If a session authorization header is sent, it will be ignored.

* `GET /http/user`: get a list of all users on this node.
* `POST /http/user`: create a new user.
* `GET /http/user/ID`: get the information of a specific user by it's `ID`.
* `POST /http/auth`: create a new authenticated session.


## Login

In order to create an authentication session, the user needs to login.

Login request: 

`POST /http/auth`

```json
{
    "id": "USER_ID",
    "pw": "PASSWORD"
}
```

Login response payload:

```json
{
    "auth": {
        "id": "USER_ID",
        "token": "AUTHENTICATION_TOKEN"
    }
}
```

## Logout

To end and logout of an authenticated session you must send the following request:

`DELETE /http/auth`

On success it returns with http code 204.


## Check whether a Session Authentication Token is still Valid

To check whether an authenticated session is still valid, you can send the following request:

`GET /http/auth`

It returns http code 204 on success and 401 on failure.
