/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QLCRY_H
#define QAUL_QLCRY_H

#include <qaul/qlformat.h>


/**
 * Start a new crypto session for a specific mode.
 *
 * What needs to be considered is that only operations supported
 * by this mode can be performed on the session.
 *
 * @param ctx
 * @param mode
 * @return
 */
int qlcry_start_session(qlcry_ctx *ctx, ql_cipher_t mode);

/**
 * Add a new participant to a session. Will throw an error if
 * the provided keypair is incompatible with the selected mode
 *
 * @param ctx
 * @param user
 * @param keypair
 * @return
 */
int ql_cry_add_participant(qlcry_ctx *ctx, ql_user *user, ql_keypair *keypair);

/**
 * Remove a participant from a session again. This means 
 * @param ctx
 * @param user
 * @return
 */
int ql_cry_remove_participant(qlcry_ctx *ctx, ql_user *user);

int ql_cry_stop_session(qlcry_ctx *ctx);

int ql_cry_sign_data(qlcry_ctx *ctx);

int ql_cry_verify_data(qlcry_ctx *ctx);

int ql_cry_encrypt_data(qlcry_ctx *ctx);

int ql_cry_decrypt_data(qlcry_ctx *ctx);


#endif //QAUL_QLCRY_H
