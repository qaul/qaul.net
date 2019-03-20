Test and Use REST API
=====================

The REST API is implemented in Rust, using rocket.io as web server.

Install Rust
------------

Open the rust web site https://rustup.rs and execute the command in a 
shell and further follow displayed steps:

    curl https://sh.rustup.rs -sSf | sh

Start REST API
--------------

Change into qaul-rest directory and run the following command to run the
program in Rust:

	cargo run

### API definition

- GET /users - Get all known users
- GET /users/<user_id> - Get user information
- PUT /users - Create a new user
- POST /users/<user_id> - Update user information
- POST /users/<user_id>/login - Authenticate as user (receive token)
- POST /users/<user_id>/logout - De-authenticate as user (hand-in token)
- GET /messages - Get all messages for current user
- GET /messages/?filter[user]=foo - Get messages for current user from/to - foo user
- GET /messages/<message_id> - Get messages for current user from/to foo - user
- PUT /messages/ create a new message, user, text, etc. inside model
- GET /files - Get all known files
- GET /files/<file_id> - Get file with id
- PUT /file - Add a new file
- GET /files/<file_id>/binary - Get file with id
- GET /interfaces -
- GET /interfaces/<interface_id> -
- POST /interfaces/<interface_id> -
- GET /network -
- GET /network/<network_id> -
- GET /binaries -
- GET /binaries/<binary_id> -
- GET /binaries/<binary_id>/binary -

### to discuss
- // PUT /voip/<user> - Voip stuff
- // DELETE /voip/<user> - Voip stuff
- // POST /voip/accept - Voip stuff
- // POST /voip/reject - Voip stuff

### Not yet implemented/ decided

- Websockets?
- `json:api` ? `json-ld` ?