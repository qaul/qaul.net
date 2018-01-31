
#include <glob.h>
#include <qaul/error.h>
#include <qaul/query.h>

/**
 *
 */
typedef struct qaul {
    void *inner;
    const char *home_path;
    const char *web_path;
} qaul;


typedef struct qaul_auth_token {};
typedef struct qaul_configuration {};
typedef struct qaul_network {};

typedef struct qaul_user_list {};
typedef struct qaul_msg_list {};
typedef struct qaul_file_list {};

typedef struct qaul_user {};
typedef struct qaul_message {};
typedef struct qaul_file {};


ql_error_t ql_initialise(struct qaul **state);
ql_error_t ql_shutdown(struct qaul *state);

ql_error_t ql_create_user(struct qaul *state, const char *username, const char *passphrase);
ql_error_t ql_delete_user(struct qaul *state, const char *username);

ql_error_t ql_login(struct qaul *state, const char *username, const char *passphrase, struct ql_auth_token **token);
ql_error_t ql_logout(struct qaul *state, struct ql_auth_token *token);

/** Get libary configuration */
ql_error_t ql_get_configuration(struct qaul *state, struct ql_configuration *config);

/** Set library configuration */
ql_error_t ql_set_configuration(struct qaul *state, struct ql_configuration config);

ql_error_t ql_get_users(struct qaul *state, struct ql_auth_token *token, struct ql_query *query, size_t *length, struct ql_user_list *list);
ql_error_t ql_get_user_data(struct qaul *state, struct ql_auth_token *token, const char *key, void **value);
ql_error_t ql_set_user_data(struct qaul *state, struct ql_auth_token *token, const char *key, void *value);

ql_error_t ql_get_messages(struct qaul *state, struct ql_auth_token *token, struct ql_query *query, size_t *length, struct ql_msg_list *list);
ql_error_t ql_send_message(struct qaul *state, struct ql_auth_token *token, const char *recipient, const char *message);

ql_error_t ql_get_files(struct qaul *state, struct ql_auth_token *token, struct ql_query *query, size_t *length, struct ql_file_list *list);
ql_error_t ql_get_file_meta(struct qaul *state, struct ql_auth_token *token, const char *name, void **data);

ql_error_t ql_add_file(struct qaul *state, struct ql_auth_token *token, const char *id, struct ql_file file);
ql_error_t ql_delete_file(struct qaul *state, struct ql_auth_token *token, const char *id);
ql_error_t ql_download_file(struct qaul *state, struct ql_auth_token *token, const char *id);

ql_error_t ql_init_call(struct qaul *state, const char *username);
ql_error_t ql_end_call(struct qaul *state, const char *username);
ql_error_t ql_accept_call(struct qaul *state);
ql_error_t ql_reject_call(struct qaul *state);

ql_error_t ql_get_network(struct qaul *state, struct ql_network *network);
ql_error_t ql_configure_network(struct qaul *state, struct ql_network network);
ql_error_t ql_get_binaries(struct qaul *state);
