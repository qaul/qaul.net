#include "qcry_context.h"

#include <memory.h>
#include <malloc.h>

#define MAGICK_NO 3

int qcry_context_init(qcry_usr_ctx *ctx, unsigned char *usr_name, qcry_ciph_t ciph_t)
{
    memset(ctx, 0, sizeof(qcry_usr_ctx));
    ctx->usr_name = usr_name;
    ctx->ciph_t = ciph_t;

    ctx->pk_ctx = (mbedtls_pk_context*) malloc(sizeof(mbedtls_pk_context));
    if(!ctx->pk_ctx) return QCRY_STATUS_MALLOC_FAIL;
    mbedtls_pk_init(ctx->pk_ctx);

    ctx->mgno = MAGICK_NO;
}

int qcry_context_attach_keys(qcry_usr_ctx *ctx, unsigned char *usr_key_pri, unsigned char *usr_key_pub)
{
    CHECK_SANE

    size_t pri_len = strlen(usr_key_pri);
    size_t pub_len = strlen(usr_key_pub);

    if(pri_len != QCRY_KEY_LEN[ctx->ciph_t] || pub_len != QCRY_KEY_LEN[ctx->ciph_t] ) {
        return QCRY_STATUS_INVALID_KEYS;
    }

    ctx->usr_key_pri = (unsigned char *) calloc(sizeof(unsigned char), QCRY_KEY_LEN[ctx->ciph_t]);
    if(!ctx->usr_key_pri) return QCRY_STATUS_MALLOC_FAIL;

    ctx->usr_key_pub = (unsigned char *) calloc(sizeof(unsigned char), QCRY_KEY_LEN[ctx->ciph_t]);
    if(!ctx->usr_key_pri) {
        free(ctx->usr_key_pri);
        return QCRY_STATUS_MALLOC_FAIL;
    }

    return QCRY_STATUS_OK;
}

int qcry_encrypt_ictx(qcry_usr_ctx *ctx, const char *msg, size_t ilen, unsigned char *(*ciph))
{
    /* Check context is valid */
    CHECK_SANE

    /* Then check our keys are valid */
    CHECK_KEYS

    mbedtls_pk_get_len(ctx);

    mbedtls_pk_encrypt(ctx->pk_ctx, msg, ilen, *ciph, 15);
}

int qcry_decrypt_ictx(qcry_usr_ctx *ctx, const char *ciph, size_t ilen, unsigned char *(*msg))
{
    /* Check context is valid */
    CHECK_SANE

    /* Then check our keys are valid */
    CHECK_KEYS

}
