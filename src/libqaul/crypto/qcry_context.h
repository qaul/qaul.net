/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QCRY_CONTEXT_
#define _QCRY_CONTEXT_

#include <mbedtls/aes.h>
#include "qcry_helper.h"
#include "mbedtls/pk.h"

/** Describes the context in which a crypto context is held */
typedef enum {
    RSA_ECDSA = 0,
    ECC_ECDSA = 1,
    AES256 = 2

} qcry_ciph_t;

/* Data required to do public key crypto*/
typedef struct {
    unsigned char       *usr_key_pub;
    unsigned char       *fp;
    short               mgno;
} qcry_pk_target;

/* Data required to do sym key crypto*/
typedef struct {
    unsigned char      *sh_key_pri;
} qcry_sk_taret;

/** Combined structure to hold unionised data */
typedef struct {
    union data
    {
        qcry_sk_taret   *sk;
        qcry_pk_target  *pk;
    } data;

    union mbed_ctx {
        mbedtls_pk_context  *pk_ctx;
        mbedtls_aes_context *sk_ctx;
    } mbed_ctx;

} qcry_trgt_t;


typedef struct {

    /* Store private key and key context */
    unsigned char       *usr_key_pri;
    unsigned int        use_ctr;
    qcry_ciph_t         ciph_t;

    /* Metadata about a uesr */
    unsigned char       *usr_name;
    unsigned char       *usr_fp;
    short               mgno;

    /* Target data for this context */
    qcry_trgt_t         **trgts;
    unsigned int        usd_trgt;
    unsigned int        max_trgt;
} qcry_usr_ctx;

#define CHECK_SANE \
    if(ctx->mgno != 3) return QCRY_STATUS_CTX_INVALID;

#define CHECK_KEYS \
    if(!(ctx->ciph_t == AES256 && ctx->usr_key_pri) || \
        !((ctx->ciph_t == RSA_ECDSA || ctx->ciph_t == ECC_ECDSA) && (ctx->usr_key_pub && ctx->usr_key_pri))) { \
            return QCRY_STATUS_INVALID_KEYS; \
        }

#define CLEAR_TARGET(ciph_t, trgt) \
    { if(ciph_t == AES256) { \
        mbedtls_aes_free(trgt->mbed_ctx.sk_ctx); \
        free(trgt->mbed_ctx.sk_ctx); \
        free(trgt->data.sk->sh_key_pri); \
        free(trgt->data.sk); \
    } else { \
        mbedtls_pk_free(trgt->mbed_ctx.pk_ctx); \
        free(trgt->mbed_ctx.pk_ctx); \
        free(trgt->data.pk->usr_key_pub); \
        free(trgt->data.pk->fp); \
        free(trgt->data.pk); } \
    free(trgt); }

/**
 * Initialises context for a username and a cipher type
 */
int qcry_context_init(qcry_usr_ctx *ctx, unsigned char *usr_name, qcry_ciph_t ciph_t);

/**
 * Attaches a private key to a context. The key is validated and length matched
 */
int qcry_context_prik_attach(qcry_usr_ctx *ctx, const unsigned char *usr_key_pri);

/**
 * Detaches a private key from a context for whatever reason.
 */
int qcry_context_prik_detach(qcry_usr_ctx *ctx);

int qcry_context_add_trgt(qcry_usr_ctx *ctx, const qcry_trgt_t *trgt, qcry_ciph_t ciph_t, unsigned int *trgt_no);

int qcry_context_remove_trgt(qcry_usr_ctx *ctx, unsigned int *trgt_no);

int qcry_encrypt_ictx(qcry_usr_ctx *ctx, const char *msg, size_t ilen, unsigned char *(*ciph));

int qcry_decrypt_ictx(qcry_usr_ctx *ctx, const char *ciph, size_t ilen, unsigned char *(*msg));

#endif // _QCRY_CONTEXT_

