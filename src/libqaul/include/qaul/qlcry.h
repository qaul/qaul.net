/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QLCRY_H
#define QAUL_QLCRY_H

#include <qaul/qlformat.h>
#include <stdlib.h>


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
 * the provided keypair is incompatible with the selected mode.
 *
 * Adding a new participant resets the operation buffer.
 *
 * @param ctx
 * @param user
 * @param keypair
 * @return
 */
int ql_cry_add_participant(qlcry_ctx *ctx, ql_user *user, ql_keypair *keypair);

/**
 * Remove a participant from a session again.
 *
 * This means that all future messages encrypted via this session
 * will no longer be readable by the removed user.
 *
 * @param ctx
 * @param user
 * @return
 */
int ql_cry_remove_participant(qlcry_ctx *ctx, ql_user *user);

/**
 * Stop the current session
 *
 * @param ctx
 * @return
 */
int ql_cry_stop_session(qlcry_ctx *ctx);

/**
 * Sign a piece of data from the owner. The result is stored
 * in the context operation buffer
 *
 * @param ctx
 * @return
 */
int ql_cry_sign_data(qlcry_ctx *ctx);

/**
 * Veirfy a piece of data from a remote participant. The result is stored
 * in the context operation buffer
 *
 * @param ctx
 * @return
 */
int ql_cry_verify_data(qlcry_ctx *ctx, ql_user *user);

/**
 * Encrypt a piece of data for each participant in the session. The result is stored
 * in the context operation buffer
 *
 * @param ctx
 * @return
 */
int ql_cry_encrypt_data(qlcry_ctx *ctx);

/**
 * Decrypt a piece of data from any participant in the session. The result is stored
 * in the context operation buffer
 *
 * @param ctx
 * @return
 */
int ql_cry_decrypt_data(qlcry_ctx *ctx);

/**
 * Query the length and type of the operation buffer.
 * Do this before reading out the buffer to avoid context faults
 *
 * Usage:
 *
 * ```C
 * size_t length = 0;
 * ql_operation_t op = NULL;
 * ql_cry_query_buffer(session, &length, &op);
 * ```
 *
 * @param ctx
 * @param length
 * @param op
 * @return
 */
int ql_cry_query_buffer(qlcry_ctx *ctx, size_t *length, ql_operation_t *op);

/**
 * Read out the session operational buffer. It is important to have metadata
 * about the operations via @ql_cry_query_buffer first
 *
 * Usage:
 *
 * ```C
 * ql_crypto_result **buffer = NULL;
 * ql_cry_get_buffer(session, &buffer);
 * for(int i = 0; i < length; i++) {
 *   switch(op) {
 *     // ...
 *   }
 * }
 * ```
 *
 * @param ctx
 * @param buffer
 * @return
 */
int ql_cry_get_buffer(qlcry_ctx *ctx, ql_crypto_result ***buffer);

#endif //QAUL_QLCRY_H
