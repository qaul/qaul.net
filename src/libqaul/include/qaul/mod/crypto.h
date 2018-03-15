/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QLCRY_H
#define QAUL_QLCRY_H

#include <qaul/mod/structures.h>
#include <qaul/error.h>
#include <stdlib.h>


/**
 * Start a new crypto session for a specific mode.
 *
 * Only operations supported by the mode selected can later
 * then be involved without throwing an UNSUPPORTED error.
 *
 * @param ctx A pointer to some space where a session can be started
 * @param mode The mode chosen for this session
 * @param owner The user who initiates this session (home user)
 * @return
 */
ql_error_t qlcry_start_session(qlcry_session_ctx *ctx, ql_cipher_t mode, ql_user_internal *owner);


/**
 * Add a new participant to a session. Will throw an error if
 * the provided keypair is incompatible with the selected mode.
 *
 * Adding a new participant resets the operation buffer.
 *
 * @param ctx
 * @param user
 * @return
 */
ql_error_t ql_cry_add_participant(qlcry_session_ctx *ctx, ql_user_external *user);


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
ql_error_t ql_cry_remove_participant(qlcry_session_ctx *ctx, ql_user_external *user);


/**
 * This function needs to be called before actual operations are possible
 *
 * It checks all provided data for validity and makes sure that
 * during operation no more errors can occur. It will set the
 * session context with QL_MODULE_INITIALISED.
 *
 * If this function is not called before invoking operations an error
 * will be thrown.
 *
 * @param ctx
 * @return
 */
ql_error_t ql_cry_finalise(qlcry_session_ctx *ctx);


/**
 * Stop the current session
 *
 * @param ctx
 * @return
 */
ql_error_t ql_cry_stop_session(qlcry_session_ctx *ctx);


/**
 * Sign a piece of data from the owner. The result is stored
 * in the context operation buffer
 *
 * @param ctx
 * @param msg
 * @return
 */
ql_error_t ql_cry_sign_data(qlcry_session_ctx *ctx, const char *msg);


/**
 * Veirfy a piece of data from a remote participant. The result is stored
 * in the context operation buffer
 *
 * @param ctx
 * @return
 */
ql_error_t ql_cry_verify_data(qlcry_session_ctx *ctx, ql_user_external *user);


/**
 * Encrypt a piece of data for each participant in the session. The result is stored
 * in the context operation buffer
 *
 * @param ctx
 * @return
 */
ql_error_t ql_cry_encrypt_data(qlcry_session_ctx *ctx);


/**
 * Decrypt a piece of data from any participant in the session. The result is stored
 * in the context operation buffer
 *
 * @param ctx
 * @return
 */
ql_error_t ql_cry_decrypt_data(qlcry_session_ctx *ctx);


/**
 * Query the length and type of the operation buffer.
 * Do this before reading out the buffer to avoid context faults
 *
 * Usage:
 *
 * ```C
 * size_t length = 0;
 * ql_operation_t op = INVALID;
 * ql_cry_query_buffer(session, &length, &op);
 * ```
 *
 * @param ctx
 * @param length
 * @param op
 * @return
 */
ql_error_t ql_cry_query_buffer(qlcry_session_ctx *ctx, size_t *length, ql_operation_t *op);


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
ql_error_t ql_cry_get_buffer(qlcry_session_ctx *ctx, ql_crypto_result ***buffer);


/**
 * A simple helper function to clear the operation buffer of a session.
 *
 * Most notibly this function is invoked by [[ql_cry_add_participant]]
 * and [[ql_cry_remove_participant]] to make sure that everybody in the
 * session get's the same data
 *
 * @param ctx
 * @return
 */
ql_error_t ql_cry_clear_buffer(qlcry_session_ctx *ctx);

#endif //QAUL_QLCRY_H
