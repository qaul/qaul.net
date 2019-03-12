/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include <qaul/error.h>

#include <qaul/mod/crypto.h>
#include <qaul/utils/arrays.h>

#include <mbedtls/ctr_drbg.h>
#include <mbedtls/entropy.h>
#include <mbedtls/md_internal.h>
#include <mbedtls/md.h>

#include <stdlib.h>
#include <string.h>



ql_error_t start_session(qlcry_session_ctx *ctx, ql_cipher_t mode, ql_user_internal *owner)
{
    /* Check if a valid mode was provided */
    if(mode != (PK_RSA || ECDSA || AES256)) {
        return INVALID_PARAMETERS;
    }

    /* Validate other inputs */
    CHECK(owner, INVALID_PARAMETERS)
    CHECK(ctx, INVALID_PARAMETERS)

    /* Zero provided ctx pointer */
    memset(ctx, 0, sizeof(qlcry_session_ctx));

    /* Setup data */
    ctx->mode = mode;
    ctx->buffer_length = 0;
    ctx->buffer_type = INVALID;

    /* Setup random */
    ctx->random = (mbedtls_ctr_drbg_context*) malloc(sizeof(mbedtls_ctr_drbg_context));
    if(ctx->random == NULL) return NOT_INITIALISED;
    mbedtls_ctr_drbg_init(ctx->random);

    /* Setup entropy */
    ctx->entropy = (mbedtls_entropy_context*) malloc(sizeof(mbedtls_entropy_context));
    if(ctx->entropy == NULL) return NOT_INITIALISED;
    mbedtls_entropy_init(ctx->entropy);

    /* Seed the random number generator for a user-by-user basis */
    int ret = mbedtls_ctr_drbg_seed(ctx->random, mbedtls_entropy_func, ctx->entropy,
                                    (const unsigned char*) owner->username, strlen(owner->username));
    if(ret != 0) return ERROR;

    /* Initialise participants array */
    ctx->no_p = 0;
    ctx->array_p = 2;
    ctx->participants = (ql_user**) calloc(sizeof(ql_user_external*), ctx->array_p);
    if(ctx->participants == NULL) return MEMORY_ALLOCATION_FAILED;

    /* Save the owner information */
    ctx->owner = owner;

    return SUCCESS;
}

ql_error_t ql_cry_stop_session(qlcry_session_ctx *ctx)
{
    CHECK(ctx, INVALID_PARAMETERS)
    INITIALISED(ctx)

    mbedtls_ctr_drbg_free(ctx->random);
    mbedtls_entropy_free(ctx->entropy);
    free(ctx->participants);
    ql_cry_clear_buffer(ctx);
    free(ctx);
}


ql_error_t ql_cry_finalise(qlcry_session_ctx *ctx)
{
    const int ret = NOT_INITIALISED;
    CHECK(ctx, ret);
    CHECK(ctx->owner, ret);
    CHECK(ctx->random, ret);
    CHECK(ctx->entropy, ret);

    if(ctx->no_p <= 0) {
        return ERROR;
    }

    if(ctx->no_p > ctx->array_p) {
        return ERROR;
    }

    if(ctx->mode == NONE) {
        return NOT_INITIALISED;
    }

    /* All looks good :) */
    ctx->initialised = QL_MODULE_INITIALISED;
    return SUCCESS;
}


ql_error_t ql_cry_add_participant(qlcry_session_ctx *ctx, ql_user_external *user)
{
    CHECK(ctx, INVALID_PARAMETERS)
    CHECK(user, INVALID_PARAMETERS)

    int ret = ql_cry_clear_buffer(ctx);
    if(ret) return ERROR;

    /* Make sure the user isn't already participating */
    for(int i = 0; i < ctx->no_p; i++) {
        if(strcmp(ctx->participants[i]->fingerprint, user->fingerprint) == 0) {
            return INVALID_DATA;
        }
    }

    /* Check if the user keypair is compatible */
    if(ctx->mode != user->pubkey->type) {
        return INVALID_PARAMETERS;
    }

    /* Make sure we have space for participants */
    ret = qlutils_resize_array((void**) &ctx->participants, sizeof(ql_user_external*), ctx->no_p, &ctx->array_p);
    if(ret != SUCCESS) return ret;

    /* Now it's safe to add the participant */
    ctx->participants[ctx->no_p++] = user;
    return SUCCESS;
}


ql_error_t ql_cry_remove_participant(qlcry_session_ctx *ctx, ql_user_external *user)
{
    CHECK(ctx, INVALID_PARAMETERS)
    CHECK(user, INVALID_PARAMETERS)

    int ret = ql_cry_clear_buffer(ctx);
    if(ret) return ret;

    /* Make sure the user isn't already participating */
    for(int i = 0; i < ctx->no_p; i++) {
        if(strcmp(ctx->participants[i]->fingerprint, user->fingerprint) == 0) {

            ctx->participants[i] = NULL;
            ctx->no_p--;

            qlutils_compact_array((void**) ctx->participants, ctx->array_p);
            return SUCCESS;
        }
    }

    return INVALID_PARAMETERS;
}


ql_error_t ql_cry_clear_buffer(qlcry_session_ctx *ctx)
{
//    CHECK(ctx, QLSTATUS_INVALID_PARAMETERS)
//    INITIALISED(ctx)
//
//    // FIXME: Use crypto_result_free
//    for(int i = 0; i < ctx->buffer_length; i++) {
//        free((void*) ctx->buffer[i]->fp);
//        free(ctx->buffer[i]->data);
//        free(ctx->buffer[i]);
//    }
//
//    free(ctx->buffer);
//    ctx->buffer_length = 0;
//    ctx->buffer_type = INVALID;
}


ql_error_t ql_cry_query_buffer(qlcry_session_ctx *ctx, size_t *length, ql_operation_t *op)
{
//    CHECK(ctx, QLSTATUS_INVALID_PARAMETERS)
//    INITIALISED(ctx)
//
//    *length = ctx->buffer_length;
//    *op = ctx->buffer_type;
//    return QLSTATUS_SUCCESS;
}


ql_error_t ql_cry_get_buffer(qlcry_session_ctx *ctx, ql_crypto_result ***buffer)
{
//    CHECK(ctx, QLSTATUS_INVALID_PARAMETERS)
//    INITIALISED(ctx)
//    *buffer = ctx->buffer;
//    return QLSTATUS_SUCCESS;
}


/**** Actual cryptography functions below ****/

ql_error_t ql_cry_sign_data(qlcry_session_ctx *ctx, const char *msg)
{
//    CHECK(ctx, QLSTATUS_INVALID_PARAMETERS)
//    INITIALISED(ctx)
//
//    /* Creating a few variables to use */
//    // FIXME: This should be declared elsewhere
//    unsigned char hash_buf[QAUL_SIGN_HASH_LEN];
//    unsigned char sign_buf[QAUL_SIGNATURE_LEN];
//    size_t olen = 0;
//
//    /* Hash the message to sign it */
//    mbedtls_md_context_t md_ctx;
//    const mbedtls_md_info_t *md_info = mbedtls_md_info_from_type(MBEDTLS_MD_SHA256);
//
//    mbedtls_md_init( &md_ctx );
//    int ret = mbedtls_md_setup( &md_ctx, md_info, 0 );
//    if(ret != 0) {
//        printf("An error occured setting up digest environment: %d!\n", ret);
//        return ret;
//    }
//
//    /** Compute SHA-256 digest of message for signature */
//    md_info->starts_func(md_ctx.md_ctx);
//    md_info->update_func(md_ctx.md_ctx, (const unsigned char*) msg, strlen(msg) + 1);
//    md_info->finish_func(md_ctx.md_ctx, hash_buf);
//
//    mbedtls_pk_context *private = (mbedtls_pk_context*) ctx->owner->keypair->sec;
//
//    /** Compute signature with message digest and private key */
//    ret = mbedtls_pk_sign(private, MBEDTLS_MD_SHA256,
//                          hash_buf, 0, sign_buf, &olen,
//                          mbedtls_ctr_drbg_random, ctx->random);
//
//    if(olen != QAUL_SIGNATURE_LEN)
//        printf("[WARNING] Signature length doesn't match for message '%s'...Misalignment probable!", msg);
//    if(ret != 0)
//        return QCRY_STATUS_ERROR;
//
//    /* Allocate some space on the buffer to return the result */
//    ctx->buffer = calloc(1, sizeof(ql_crypto_result));
//    if(ctx->buffer == NULL) return QLSTATUS_MALLOC_FAILED;
//
//    ql_crypto_result res;
//    res.data = (unsigned char*) calloc(sizeof(unsigned char), olen);
//    if(res.data == NULL) return QCRY_STATUS_MALLOC_FAIL;
//    memcpy(res.data, sign_buf, olen);
//    memcpy((void*) res.fp, ctx->owner->fingerprint, sizeof(ctx->owner->fingerprint));
//    res.length = olen;
//
//    return QLSTATUS_SUCCESS;
}