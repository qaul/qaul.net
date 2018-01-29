



unsigned int ql_create_user(const char *username, const char *passphrase);
unsigned int ql_delete_user(const char *username);

unsigned int ql_login(const char *username, const char *passphras);
unsigned int ql_logout(const char *username);

unsigned int ql_get_configuration(const char *username, const char *key, void **value);
unsigned int ql_set_configuration(const char *username, const char *key, void *value);

unsigned int ql_get_users(const char *username, const char *query, usize *length, void **data);
unsigned int ql_get_user_data(const char *username, const char *key, void **value);
unsigned int ql_set_user_data(const char *username, const char *key, void *value);

unsigned int ql_get_messages( /* No idea yet */ );
unsigned int ql_send_message();

unsigned int ql_get_all_files();
unsigned int ql_get_file(const char *id);
unsigned int ql_get_file_meta(const char *id);

unsigned int ql_add_file(const char *id);
unsigned int ql_delete_file(const char *id);
unsigned int ql_download_file(const char *id);

unsigned int ql_init_call(const char *username);
unsigned int ql_end_call(const char *username);
unsigned int ql_accept_call();
unsigned int ql_reject_call();

unsigned int ql_get_network();
unsigned int ql_configure_network();
unsigned int ql_get_binaries();


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