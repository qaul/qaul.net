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


/// Some helpful macros
#define CHECK(field) { if(field == NULL) return QLSTATUS_INVALID_PARAMETERS; }


int qlcry_start_session(qlcry_session_ctx *ctx, ql_cipher_t mode, ql_user *owner)
{
    /* Check if a valid mode was provided */
    if(mode != (PK_RSA || ECDSA || AES256)) {
        return QLSTATUS_INVALID_PARAMETERS;
    }

    /* Validate other inputs */
    CHECK(owner)
    CHECK(ctx)

    /* Zero provided ctx pointer */
    memset(ctx, 0, sizeof(qlcry_session_ctx));

    /* Setup data */
    ctx->mode = mode;

    /* Setup random */
    ctx->random = (mbedtls_ctr_drbg_context*) malloc(sizeof(mbedtls_ctr_drbg_context));
    if(ctx->random == NULL) return QSTATUS_MALLOC_FAILED;
    mbedtls_ctr_drbg_init(ctx->random);

    /* Setup entropy */
    ctx->entropy = (mbedtls_entropy_context*) malloc(sizeof(mbedtls_entropy_context));
    if(ctx->entropy == NULL) return QSTATUS_MALLOC_FAILED;
    mbedtls_entropy_init(ctx->entropy);

    /* Seed the random number generator for a user-by-user basis */
    int ret = mbedtls_ctr_drbg_seed(ctx->random, mbedtls_entropy_func, ctx->entropy,
                                    (const unsigned char*) owner->username, strlen(owner->username));
    if(ret != 0) return QLSTATUS_ERROR;  // FIXME: How to handle module-internal error codes?

    /* Initialise participants array */
    ctx->no_p = 0;
    ctx->array_p = 2;
    ctx->participants = (ql_user**) calloc(sizeof(ql_user*), ctx->array_p);
    if(ctx->participants == NULL) return QSTATUS_MALLOC_FAILED;

    /* Save the owner information */
    ctx->owner = owner;

    return QLSTATUS_SUCCESS;
}