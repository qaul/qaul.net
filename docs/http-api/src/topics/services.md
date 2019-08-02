# Services
At its core qaul.net provides relatively little user facing functionality, it
is a tool from which other things a built. Most of this functionality comes in 
the form of services, pluggable extensions that leverage qual.net's networking
capabilities.

## Service Apis
Services can expose their own HTTP Apis which may, like the core service, use
JSON:API or may use something completely different. These apis are mounted under
the service name. So for example, consider a service `foo` that exposes an
endpoint `/bar`. When using the api you would access this endpoint like
`/foo/bar`.

## Errors
There are a few errors associated with the mounting of services that it is 
important to be aware of.

### No Path
**Status:** 400 _Bad Request_

This error occurs when you request the root endpoint, `/`. To use the api
please specifiy what endpoint you intend to access.

### No Service
**Status:** 404 _Not Found_

The service you're requesting does not exist. If you're pretty sure it should
exist, check that the service is registered.

### Service Not Authorized
**Status:** 403 _Forbidden_

The user you're representing does not run this service and as such the service
cannot be allowed to access that users data.
