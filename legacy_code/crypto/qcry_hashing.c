#include <qaullib/qcry_hashing.h>
#include <memory.h>
#include <stdlib.h>

#include <mbedtls/sha512.h>
#include <mbedtls/sha256.h>
#include <mbedtls/base64.h>
#include <mbedtls/platform.h>

#include "qcry_helper.h"
#include "qcry_context.h"

int qcry_hashing_init(struct qcry_hash_ctx *ctx, unsigned int hash, const char *salt)
{
    int err_no = QCRY_STATUS_MALLOC_FAIL; // Default error

    memset(ctx, 0, sizeof(struct qcry_hash_ctx));
    ctx->hash = hash;

    /* Copy our salt if one exists */
    if(salt != NULL) {
        size_t salt_len = strlen(salt) + 1;
        ctx->salt = (char*) malloc(sizeof(char) * salt_len);
        if(!ctx->salt)
            goto failed;
        strcpy(ctx->salt, salt);
    }

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

    /* Set magic number and prep for return */
    ctx->mgno = MAGICK_NO;
    err_no = QCRY_STATUS_OK;

    /* If errors occur we need to recover the damage */
    failed:
    if(err_no != 0)
        free(ctx->salt);
    if(err_no != 0)
        free(ctx->curr_bfr);

    return (err_no != 0) ? err_no : QCRY_STATUS_OK;
}

int qcry_hashing_free(struct qcry_hash_ctx *ctx)
{
    CHECK_SANE

    /* Free memory and hash backend */
    free(ctx->curr_bfr);
    free(ctx->salt);
    switch(ctx->hash) {
        case QCRY_HASH_SHA512:
            mbedtls_sha512_free((mbedtls_sha512_context*) ctx->md_ctx);
            break;

        case QCRY_HASH_SHA256:
            mbedtls_sha256_free((mbedtls_sha256_context*) ctx->md_ctx);
            break;

        default:
            return QCRY_STATUS_INVALID_CTX;
    }

    return QCRY_STATUS_OK;
}

size_t qcry_hashing_getlen(unsigned int hash)
{
    return 256;
}

/** Append a message to the buffer **/
int qcry_hashing_append(struct qcry_hash_ctx *ctx, const char *msg)
{
    CHECK_SANE

    size_t prev_s = ctx->bfr_s;

    /* Check that our buffer is big enough to handle new data */
    if(ctx->bfr_s < strlen(msg)) {
        if(prev_s == 0 && ctx->salt) {
            ctx->bfr_s += ((strlen(msg) + strlen(ctx->salt)) * 2);
        } else {
            ctx->bfr_s += (strlen(msg) * 2);
        }

        /* Make a new clean buffer */
        char *tmp = (char*) calloc(sizeof(char), ctx->bfr_s);
        if(tmp == NULL) return QCRY_STATUS_MALLOC_FAIL;

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

    return QCRY_STATUS_OK;
}

/** Read the current state of the buffer **/
const char *qcry_hashing_buffer(struct qcry_hash_ctx *ctx)
{
    /** Check sane macro can't apply because we want to return string */
    if(ctx->mgno != 3) return NULL;
    return ctx->curr_bfr;
}

/** Build the buffer to a hash and encode it into a return buffer */
int qcry_hashing_build(struct qcry_hash_ctx *ctx, unsigned int encoding, char *(*buffer))
{
    CHECK_SANE

    /* Cut down buffer size to required only */
    unsigned char *tmpsrc = calloc(sizeof(unsigned char), ctx->bfr_occ);
    memcpy(tmpsrc, ctx->curr_bfr, ctx->bfr_occ);

    size_t hash_len;
    switch(ctx->hash) {
        case QCRY_HASH_SHA256: hash_len = QCRY_SHA256_LENGTH; break;
        case QCRY_HASH_SHA512: hash_len = QCRY_SHA512_LENGTH; break;
        default: return QCRY_STATUS_INVALID_CTX;
    }

    /** Provide a buffer output for any hash function */
    unsigned char *output = (unsigned char*) malloc(sizeof(unsigned char) * hash_len);

    /** Use the apropriate function depending on what hash function we require **/
    switch(ctx->hash) {
        case QCRY_HASH_SHA512:
            /* Start hash transaction */
            mbedtls_sha512_starts((mbedtls_sha512_context*) ctx->md_ctx, 0);

            /* Append our buffer */
            mbedtls_sha512_update((mbedtls_sha512_context*) ctx->md_ctx, tmpsrc, ctx->bfr_occ);

            /* Finish transaction and write into buffer */
            mbedtls_sha512_finish((mbedtls_sha512_context*) ctx->md_ctx, output);
            break;

        case QCRY_HASH_SHA256:
            mbedtls_sha256_starts((mbedtls_sha256_context*) ctx->md_ctx, 0);

            /* Append our buffer */
            mbedtls_sha256_update((mbedtls_sha256_context*) ctx->md_ctx, tmpsrc, ctx->bfr_occ);

            /* Finish transaction and write into buffer */
            mbedtls_sha256_finish((mbedtls_sha256_context*) ctx->md_ctx, output);
            break;

        default:
            ret = QCRY_STATUS_INVALID_CTX;
            goto failed;
    }

    /** Select output encoding and process buffer **/
    switch(encoding) {
        case QCRY_HASH_BASE64:
            {
                /* Determine base64 size and malloc buffer for return */
                int base64_len = qcry_base64_enclength((int) hash_len);
                (*buffer) = (char*) calloc(sizeof(char), (unsigned int) base64_len);

                /* Call the mbedtls base64 function */
                size_t bw;
                ret = mbedtls_base64_encode((unsigned char*) *buffer,
                                      (unsigned int) base64_len,
                                      &bw, output, hash_len + 1); // Consider \0 !

                if(ret != 0)
                    return QCRY_STATUS_ENCODE_FAILED;
            }

            break;

        default:
            ret = QCRY_STATUS_INVALID_CTX;
            goto failed;
    }


    failed:

    /* Free temp buffer */
    free(tmpsrc);
    free(output);
    return (ret != 0) ? ret : QCRY_STATUS_OK;

}

int qcry_hashing_clear(struct qcry_hash_ctx *ctx)
{
    free(ctx->curr_bfr);
    ctx->bfr_s = MIN_BFR_S;
    ctx->bfr_occ = 0;

    ctx->curr_bfr = (char*) calloc(sizeof(char), ctx->bfr_s);
    if(!ctx->curr_bfr) return QCRY_STATUS_MALLOC_FAIL;

    return QCRY_STATUS_OK;
}
