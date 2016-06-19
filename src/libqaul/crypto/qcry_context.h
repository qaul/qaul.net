*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QCRY_CONTEXT_
#define _QCRY_CONTEXT_

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
    mbedtls_pk_context  *pk_ctx;
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
    qcry_ciph_t type;

    union data
    {
        qcry_sk_taret   *sk;
        qcry_pk_target  *pk;
    } data;

} qcry_trgt_t;


typedef struct {
    unsigned char       *usr_key_pri;
    unsigned char       *usr_name;
    unsigned char       *usr_fp;
    short               mgno;

    /* Target data for this context */
    qcry_trgt_t         *trgts;
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

/**
 * Initialises context for a username and a cipher type
 */
int qcry_context_init(qcry_usr_ctx *ctx, unsigned char *usr_name, qcry_ciph_t ciph_t);

/**
 * Attaches a public/ private keypair to a context
 */
int qcry_context_attach_keys(qcry_usr_ctx *ctx, unsigned char *usr_key_pri, unsigned char *usr_key_pub);

int qcry_encrypt_ictx(qcry_usr_ctx *ctx, const char *msg, size_t ilen, unsigned char *(*ciph));

int qcry_decrypt_ictx(qcry_usr_ctx *ctx, const char *ciph, size_t ilen, unsigned char *(*msg));

#endif // _QCRY_CONTEXT_

