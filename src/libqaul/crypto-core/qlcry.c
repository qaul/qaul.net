/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include <qaul/qlcry.h>
#include <qaul/qlerror.h>

#include <stdlib.h>
#include <string.h>

#include <mbedtls/ctr_drbg.h>
#include <mbedtls/entropy.h>
#include <qaul/qlutils.h>


/// Some helpful macros


int qlcry_start_session(qlcry_session_ctx *ctx, ql_cipher_t mode, ql_user *owner)
{
    /* Check if a valid mode was provided */
    if(mode != (PK_RSA || ECDSA || AES256)) {
        return QLSTATUS_INVALID_PARAMETERS;
    }

    /* Validate other inputs */
    CHECK(owner, QLSTATUS_INVALID_PARAMETERS)
    CHECK(ctx, QLSTATUS_INVALID_PARAMETERS)

    /* Zero provided ctx pointer */
    memset(ctx, 0, sizeof(qlcry_session_ctx));

    /* Setup data */
    ctx->mode = mode;

    /* Setup random */
    ctx->random = (mbedtls_ctr_drbg_context*) malloc(sizeof(mbedtls_ctr_drbg_context));
    if(ctx->random == NULL) return QLSTATUS_NOT_INITIALISED;
    mbedtls_ctr_drbg_init(ctx->random);

    /* Setup entropy */
    ctx->entropy = (mbedtls_entropy_context*) malloc(sizeof(mbedtls_entropy_context));
    if(ctx->entropy == NULL) return QLSTATUS_NOT_INITIALISED;
    mbedtls_entropy_init(ctx->entropy);

    /* Seed the random number generator for a user-by-user basis */
    int ret = mbedtls_ctr_drbg_seed(ctx->random, mbedtls_entropy_func, ctx->entropy,
                                    (const unsigned char*) owner->username, strlen(owner->username));
    if(ret != 0) return QLSTATUS_ERROR;  // FIXME: How to handle module-internal error codes?

    /* Initialise participants array */
    ctx->no_p = 0;
    ctx->array_p = 2;
    ctx->participants = (ql_user**) calloc(sizeof(ql_user*), ctx->array_p);
    if(ctx->participants == NULL) return QLSTATUS_MALLOC_FAILED;

    /* Save the owner information */
    ctx->owner = owner;

    return QLSTATUS_SUCCESS;
}


int ql_cry_finalise(qlcry_session_ctx *ctx)
{
    const int ret = QLSTATUS_NOT_INITIALISED;
    CHECK(ctx, ret);
    CHECK(ctx->owner, ret);
    CHECK(ctx->random, ret);
    CHECK(ctx->entropy, ret);

    if(ctx->no_p <= 0) {
        return ret;
    }

    if(ctx->no_p > ctx->array_p) {
        return QLSTATUS_ERROR;
    }

    if(ctx->mode == NONE) {
        return QLSTATUS_NOT_INITIALISED;
    }

    /* All looks good :) */
    ctx->initialised = QL_MODULE_INITIALISED;
    return QLSTATUS_SUCCESS;
}


int ql_cry_add_participant(qlcry_session_ctx *ctx, ql_user *user)
{
    CHECK(ctx, QLSTATUS_INVALID_PARAMETERS)
    CHECK(user, QLSTATUS_INVALID_PARAMETERS)

    /* Make sure the user isn't already participating */
    for(int i = 0; i < ctx->no_p; i++) {
        if(strcmp(ctx->participants[i]->fingerprint, user->fingerprint) == 0) {
            return QLSTATUS_DUPLIATE_DATA;
        }
    }

    /* Check if the user keypair is compatible */
    if(ctx->mode != user->keypair->type) {
        return QLSTATUS_INVALID_PARAMETERS;
    }

    /* Make sure we have space for participants */
    int ret = qlutils_resize_array((void**) &ctx->participants, sizeof(ql_user*), ctx->no_p, &ctx->array_p);
    if(ret != QLSTATUS_SUCCESS) return ret;

    /* Now it's safe to add the participant */
    ctx->participants[ctx->no_p++] = user;
    return QLSTATUS_SUCCESS;
}