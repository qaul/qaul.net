# JSON:API
The majority of endpoints use [JSON:API](https://jsonapi.org/). More information 
about the requirements and structure of JSON:API can be found in the 
[JSON:API](https://jsonapi.org/format/) docs. 

## Request Body
JSON:API documents will occupy the entire body of their requests and responses

## `Content-Type`
The content type field MUST be `application/vnd.api+json` or the document will
not be parsed.

## Errors
If the request's `Content-Type` header indicates that a request contains a JSON:API
document. This will happen for ALL endpoints, even endpoints that don't support 
JSON:API so these errors can pop up in unexpected places.

### Invalid Media Type Parameters
**Status:** 415 _Unsupported Media Type_

As required by the [JSON:API docs](https://jsonapi.org/format/#content-negotiation-clients)
the server returns this when the client specifies any media type parameters with the 
`Content-Type` header

### No Acceptable Type
**Status:** 416 _Not Acceptable_

The client's `Accept` header contained a JSON:API media type and all the instances
of that media type are modified with media type parameters. This server returns this
as required by the [JSON:API docs](https://jsonapi.org/format/#content-negotiation-clients)

### Deserialization Error
**Status:** 400 _Bad Request_

The server tried to parse the body as a JSON:API document and failed. Check the
details of the error message for more information.

### No Document
**Status:** 400 _Bad Request_

This error will ONLY occur on JSON:API endpoints and indicates that no JSON:API
document was provided despite the endpoint requiring one. In practice this
error will only happen when the `Content-Type` header is not `application/vnd.api+json`.
