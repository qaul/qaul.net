/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QLAUTH_H
#define QAUL_QLAUTH_H

#include <qaul/qaul.h>
#include <qaul/error.h>

#include <qaul/mod/structures.h>


/**
 * Initialise the authentication module. This calls
 * will malloc memory and return any errors encountered.
 *
 * @param ctx
 * @return
 */
ql_error_t qlauth_initialise(qlauth_ctx **ctx);


/**
 * De-initialise the authentication module and free the memory
 * previously malloced. Will forcibly end all still-open
 * sessions and free all memory.
 *
 * @param ctx
 * @return
 */
ql_error_t qlauth_stop(qlauth_ctx *ctx);


/**
 * Authenticate a user with their username and passphrase. Creates
 * a user authentication token that can then be verified again later.
 *
 * @param ctx
 * @param username
 * @param passphrase
 * @return
 */
ql_error_t qlauth_authenticate(qlauth_ctx *ctx, const char *username, const char *passphrase, struct qaul_auth_token **token);


/**
 * Verify that a given authentication token is valid. Returns
 * the associated user struct to said token. In case of error
 * the provided pointer is written with NULL.
 *
 * @param ctx
 * @param token
 * @return 0 in case of success
 */
ql_error_t qlauth_verify(qlauth_ctx *ctx, struct qaul_auth_token *token, ql_user **user);


/**
 * Hand in an authentication token and invalidate all
 * future transactions done with this token.
 *
 * @param ctx
 * @param token
 * @return
 */
ql_error_t qlauth_deauthenticate(qlauth_ctx *ctx, struct qaul_auth_token *token);


/**
 * Create a new user with a username and passphrase.
 *
 * This function is blocking and runs quite a while. It seeds a new random number
 * generator, then creates a new keypair
 *
 * @param ctx
 * @param username
 * @param passphrase
 * @return
 */
ql_error_t qlauth_create_user(qlauth_ctx *ctx, const char *username, const char *passphrase);


/**
 * Change the passphrase for a user. Requires said user to
 * have authenticated first
 *
 * @param ctx
 * @param token
 * @param new_passphrase
 * @return
 */
ql_error_t qlauth_change_passphrase(qlauth_ctx *ctx, struct qaul_auth_token *token, const char *new_passphrase);


/**
 * Deletes a registered user.
 *
 * The user needs to be autenticated because this function then consumes the token
 *
 * @param ctx
 * @param token
 * @return
 */
ql_error_t qlauth_delete_user(qlauth_ctx *ctx, const char *username, struct qaul_auth_token *token);


/**
 * Get's the (public) user information associated with an auth token
 *
 * @param token
 * @param user
 * @return
 */
ql_error_t qlauth_get_user_info(qlauth_ctx *ctx, struct qaul_auth_token *token, struct qaul_user **user);


/**
 * Set's (public) information for a user associated with an auth token
 *
 * @param ctx
 * @param token
 * @param user
 * @return
 */
ql_error_t qlauth_set_user_info(qlauth_ctx *ctx, struct qaul_auth_token *token, struct qaul_user *user);

#endif //QAUL_QLAUTH_H
