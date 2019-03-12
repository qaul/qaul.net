#ifndef QAUL_QAUL_H
#define QAUL_QAUL_H


#include <glob.h>
#include <qaul/error.h>
#include <qaul/query.h>


typedef struct qaul_auth_token {};
typedef struct qaul_configuration {};
typedef struct qaul_network {};

typedef struct qaul_user_list {};
typedef struct qaul_msg_list {};
typedef struct qaul_file_list {};

typedef struct qaul_user {};
typedef struct qaul_message {};
typedef struct qaul_file {};


/**
 * Defines what type of OS is used for qaul,net
 */
typedef enum qaul_os {
    LINUX, MACOS, WINDOWS
};


/**
 *
 */
typedef struct qaul {
    void *inner;
    enum qaul_os os;
    const char *home_path;
    const char *resource_path;
} qaul;


/**
 * Initialise the qaul library for a specific operating system, a home path and resource path.
 *
 * The resource path can be used either for a native front-end or the
 * webserver web source which then serves the html files
 *
 * @param state
 * @param os
 * @param home
 * @param resources
 * @return
 */
ql_error_t ql_initialise(struct qaul **state, enum qaul_os os, const char *home, const char *resources);


/**
 *
 * @param state
 * @return
 */
ql_error_t ql_shutdown(struct qaul *state);


/**
 *
 * @param state
 * @param username
 * @param passphrase
 * @return
 */
ql_error_t ql_create_user(struct qaul *state, const char *username, const char *passphrase);


/**
 *
 * @param state
 * @param username
 * @return
 */
ql_error_t ql_delete_user(struct qaul *state, const char *username);


/**
 *
 * @param state
 * @param username
 * @param passphrase
 * @param token
 * @return
 */
ql_error_t ql_login(struct qaul *state, const char *username, const char *passphrase, struct qaul_auth_token **token);


/**
 *
 * @param state
 * @param token
 * @return
 */
ql_error_t ql_logout(struct qaul *state, struct qaul_auth_token *token);


/**
 * Get libary configuration
 *
 * @param state
 * @param config
 * @return
 */
ql_error_t ql_get_configuration(struct qaul *state, struct qaul_configuration *config);


/**
 * Set library configuration
 *
 * @param state
 * @param config
 * @return
 */
ql_error_t ql_set_configuration(struct qaul *state, struct qaul_configuration config);


/**
 *
 * @param state
 * @param token
 * @param query
 * @param length
 * @param list
 * @return
 */
ql_error_t ql_get_users(struct qaul *state, struct qaul_auth_token *token, struct qaul_query *query, size_t *length, struct qaul_user_list *list);


/**
 *
 * @param state
 * @param token
 * @param key
 * @param value
 * @return
 */
ql_error_t ql_get_user_data(struct qaul *state, struct qaul_auth_token *token, const char *key, void **value);


/**
 *
 * @param state
 * @param token
 * @param key
 * @param value
 * @return
 */
ql_error_t ql_set_user_data(struct qaul *state, struct qaul_auth_token *token, const char *key, void *value);


/**
 *
 * @param state
 * @param token
 * @param recipient
 * @param message
 * @return
 */
ql_error_t ql_send_message(struct qaul *state, struct qaul_auth_token *token, const char *recipient, const char *message);


/**
 *
 * @param state
 * @param token
 * @param name
 * @param data
 * @return
 */
ql_error_t ql_get_file_meta(struct qaul *state, struct qaul_auth_token *token, const char *name, void **data);


/**
 *
 * @param state
 * @param token
 * @param id
 * @param file
 * @return
 */
ql_error_t ql_add_file(struct qaul *state, struct qaul_auth_token *token, const char *id, struct qaul_file file);


/**
 *
 * @param state
 * @param token
 * @param id
 * @return
 */
ql_error_t ql_delete_file(struct qaul *state, struct qaul_auth_token *token, const char *id);


/**
 *
 * @param state
 * @param token
 * @param id
 * @return
 */
ql_error_t ql_download_file(struct qaul *state, struct qaul_auth_token *token, const char *id);


/**
 *
 * @param state
 * @param username
 * @return
 */
ql_error_t ql_init_call(struct qaul *state, const char *username);


/**
 *
 * @param state
 * @param username
 * @return
 */
ql_error_t ql_end_call(struct qaul *state, const char *username);


/**
 *
 * @param state
 * @return
 */
ql_error_t ql_accept_call(struct qaul *state);


/**
 *
 * @param state
 * @return
 */
ql_error_t ql_reject_call(struct qaul *state);


/**
 *
 * @param state
 * @param network
 * @return
 */
ql_error_t ql_get_network(struct qaul *state, struct qaul_network *network);


/**
 *
 * @param state
 * @param network
 * @return
 */
ql_error_t ql_configure_network(struct qaul *state, struct qaul_network network);


/**
 *
 * @param state
 * @return
 */
ql_error_t ql_get_binaries(struct qaul *state);

#endif // QAUL_QAUL_H