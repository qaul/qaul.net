
typedef struct qaul {
    int bla;
} qaul;


typedef struct ql_auth_token {

};

unsigned int ql_initialise(struct qaul **state);

unsigned int ql_shutdown(struct qaul *state);

unsigned int ql_create_user(struct qaul *state, const char *username, const char *passphrase);
unsigned int ql_delete_user(struct qaul *state, const char *username);

unsigned int ql_login(struct qaul *state, const char *username, const char *passphrase, struct ql_auth_token **token);
unsigned int ql_logout(struct qaul *state, struct ql_auth_token *token);

/** Get libary configuration */
unsigned int ql_get_configuration(struct qaul *state, const char *key, void **value);

/** Set library configuration */
unsigned int ql_set_configuration(struct qaul *state, const char *key, void *value);

unsigned int ql_get_users(struct qaul *state, struct ql_auth_token *token, struct ql_query *query, size_t *length, void **data);
unsigned int ql_get_user_data(struct qaul *state, struct ql_auth_token *token, const char *key, void **value);
unsigned int ql_set_user_data(struct qaul *state, struct ql_auth_token *token, const char *key, void *value);

unsigned int ql_get_messages(struct qaul *state, struct ql_auth_token *token, struct ql_query *query, size_t *length, void **data);
unsigned int ql_send_message(struct qaul *state, struct ql_auth_token *token, const char *recipient, const char *message);

unsigned int ql_get_all_files(struct qaul *state, struct ql_auth_token *token, struct ql_query *query, size_t *length, void **data);
unsigned int ql_get_file(struct qaul *state, struct ql_auth_token *token, const char *name);
unsigned int ql_get_file_meta(struct qaul *state, struct ql_auth_token *token, const char *name, void **data);

unsigned int ql_add_file(struct qaul *state, struct ql_auth_token *token, const char *id);
unsigned int ql_delete_file(struct qaul *state, struct ql_auth_token *token, const char *id);
unsigned int ql_download_file(struct qaul *state, struct ql_auth_token *token, const char *id);

unsigned int ql_init_call(struct qaul *state, const char *username);
unsigned int ql_end_call(struct qaul *state, const char *username);
unsigned int ql_accept_call(struct qaul *state);
unsigned int ql_reject_call(struct qaul *state);

unsigned int ql_get_network(struct qaul *state);
unsigned int ql_configure_network(struct qaul *state);
unsigned int ql_get_binaries(struct qaul *state);


/*
 * === Open problems ===
 *
 * - Keep state in a struct (Provide state in functions)
 * - Define return values for actions (global codes)
 * - Gate actions with authentication tokens
 * - Figure out how to return data
 *   - qaul.net configuration
 *   - User configuration
 *   - Message lists
 *   - File data
 *   - File metadata
 *   - Network data
 *
 * Look at what data needs to be returned/ provided to functions. Either define
 * some structs to "transport" the data from the API or (if too complex) use
 * messagepack to do the same.
 *
 *
 */