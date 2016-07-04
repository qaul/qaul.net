#include <qaullib/qcry_hashing.h>
#include <memory.h>
#include <stdlib.h>
#include <mbedtls/sha512.h>
#include <mbedtls/sha256.h>
#include "qcry_helper.h"
#include "qcry_context.h"

int qcry_hashing_init(struct qcry_hash_ctx *ctx, unsigned int hash, const char *salt)
{
    int err_no = 0;

    memset(ctx, 0, sizeof(struct qcry_hash_ctx));
    ctx->hash = hash;

    /* Copy our salt */
    ctx->salt = malloc(sizeof(char) * strlen(salt));
    if(!ctx->salt)
        goto failed;
    strcpy(ctx->salt, salt);

    /* Initialise a default buffer size */
    ctx->bfr_s = MIN_BFR_S;
    ctx->bfr_occ = 0;
    ctx->curr_bfr = (char*) calloc(sizeof(char), ctx->bfr_s);
    if(!ctx->curr_bfr)
        goto failed;

    /* Setup hashing mbed context */
    switch(ctx->hash) {
        /* If SHA512 was chosen as a hash function */
        case QCRY_HASH_SHA512:
            ctx->md_ctx = (mbedtls_sha512_context*) malloc(sizeof(mbedtls_sha512_context));
            if(!ctx->md_ctx)
                goto failed;

            /* After checking memory: initialise! */
            mbedtls_sha512_init(ctx->md_ctx);
            break;

        /* If SHA256 was chosen as a hash function */
        case QCRY_HASH_SHA256:
            ctx->md_ctx = (mbedtls_sha256_context*) malloc(sizeof(mbedtls_sha256_context));
            if(!ctx->md_ctx)
                goto failed;

            /* After checking memory: initialise! */
            mbedtls_sha256_init(ctx->md_ctx);
            break;

        /* This means an invalid hash ID code */
        default:
            err_no = QCRY_STATUS_INVALID_PARAMS;
            goto failed;
    }

    /* Set magic number and return */
    ctx->mgno = MAGICK_NO;
    return QCRY_STATUS_OK;

    /* If errors occur we need to recover the damage */
    failed:
    free(ctx->salt);
    free(ctx->curr_bfr);
    return (err_no != 0) ? err_no : QCRY_STATUS_MALLOC_FAIL;
}

int qcry_hashing_free(struct qcry_hash_ctx *ctx)
{
    free(ctx->salt);
    free(ctx->curr_bfr);
}

/** Append a message to the buffer **/
int qcry_hashing_append(struct qcry_hash_ctx *ctx, const char *msg)
{
    CHECK_SANE

    int prev_s = ctx->bfr_s;

    /* Check that our buffer is big enough to handle new data */
    if(ctx->bfr_s < strlen(msg)) {
        if(prev_s == 0 && ctx->salt) {
            ctx->bfr_s += ((strlen(msg) + strlen(ctx->salt)) * 2);
        } else {
            ctx->bfr_s += (strlen(msg) * 2);
        }

        /* Make a new clean buffer */
        char *tmp = (char*) calloc(sizeof(char), ctx->bfr_s);

        /* Prepend the salt if it's the first block and we have one */
        if(prev_s == 0 && ctx->salt) {
            ctx->bfr_occ += strlen(ctx->salt + 2);
            strcpy(tmp, ctx->salt);
            strcat(tmp, "::");
        }

        /* Move data from old to new */
        if(ctx->curr_bfr) strcpy(tmp, ctx->curr_bfr);

        /* Move buffer ptr and clear old */
        free(ctx->curr_bfr);
        ctx->curr_bfr = tmp;
    }

    /** Append our message and increase the occupancy counter */
    ctx->bfr_occ += strlen(msg);
    strcat(ctx->curr_bfr, msg);
}

/** Read the current state of the buffer **/
const char *qcry_hashing_buffer(struct qcry_hash_ctx *ctx)
{
    if(ctx->mgno != 3) return NULL;
    return ctx->curr_bfr;
}

/** Build the buffer to a hash and encode it into a return buffer */
int qcry_hashing_build(struct qcry_hash_ctx *ctx, unsigned char *(*buffer))
{

}

