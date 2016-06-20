/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QCRY_CONTEXT_
#define _QCRY_CONTEXT_

#include "mbedtls/aes.h"
#include "mbedtls/pk.h"
#include "mbedtls/ctr_drbg.h"

#include "qcry_helper.h"

/** Describes the context in which a crypto context is held */
typedef enum {
    PK_RSA = 0,
    ECDSA = 1,
    AES256 = 2
} qcry_ciph_t;

/* Data required to do public key crypto*/
typedef struct {
    unsigned char   *usr_key_pub;
    unsigned char   *fp;

    int             key_len;
    short           mgno;
} qcry_pk_target;

/* Data required to do sym key crypto*/
typedef struct {
    unsigned char      *sh_key_pri;
} qcry_sk_taret;

/** Combined structure to hold unionised data */
typedef struct {
    union d
    {
        qcry_sk_taret   *sk;
        qcry_pk_target  *pk;
    } d;

    union ctx {
        mbedtls_pk_context  *pk;
        mbedtls_aes_context *sk;
    } ctx;
    short               mgno;
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
    
    /* Seeds and entropy contexts */
    mbedtls_ctr_drbg_context    *ctr_drbg;
} qcry_usr_ctx;

/** Used to check if initialisation was done on a context */
#define CHECK_SANE \
    if(ctx->mgno != 3) return QCRY_STATUS_CTX_INVALID;

/* Used to check if initialisation was done on a target */
#define CHECK_TARGET(ctx, trgt_no)  \
    { if(ctx->trgts[trgt_no]->mgno != MAGICK_NO) return QCRY_STATUS_INVALID_TARGET; }

/* Helper macro to remove a target and all its allocated child heap memory */
#define CLEAR_TARGET(ciph_t, trgt) \
    { if(ciph_t == AES256) { \
        mbedtls_aes_free(trgt->ctx.sk); \
        free(trgt->ctx.sk); \
        free(trgt->d.sk->sh_key_pri); \
        free(trgt->d.sk); \
    } else { \
        mbedtls_pk_free(trgt->ctx.pk); \
        free(trgt->ctx.pk); \
        free(trgt->d.pk->usr_key_pub); \
        free(trgt->d.pk->fp); \
        free(trgt->d.pk); } \
    free(trgt); }


// TODO: Change this into a macro!
static int QCRY_KEY_LEN[] = { 2048, 192, 256 };

//#define CHECK_KEYLEN(type, key) { \
//    size_t len = strlen(key); \
//    if(len != QCRY_KEY_LEN[type] return QCRY_STATUS_INVALID_KEYS ) }


/**
 * Initialises context for a username and a cipher type
 */
int qcry_context_init(qcry_usr_ctx *ctx, unsigned char *usr_name, qcry_ciph_t ciph_t);

/**
 * Attaches a private key to a context. The key is validated and length matched
 */
int qcry_context_prk_attach(qcry_usr_ctx *ctx, const unsigned char *usr_key_pri);

/**
 * Detaches a private key from a context for whatever reason.
 */
int qcry_context_prk_detach(qcry_usr_ctx *ctx);

int qcry_context_add_trgt(qcry_usr_ctx *ctx, const qcry_trgt_t *trgt, qcry_ciph_t ciph_t, unsigned int *trgt_no);

int qcry_context_remove_trgt(qcry_usr_ctx *ctx, unsigned int *trgt_no);

/**
 * Use this function to encrypt messages against a target. This requires an initialised
 *  context and target to be present for the crypto to work.
 *
 *  A buffer MAY be allocated before usage but will usually want to be created and checked for you.
 *
 */
int qcry_encrypt_trgt(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *msg, size_t ilen, unsigned char *(*ciph));
int qcry_decrypt_trgt(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *ciph, size_t ilen, unsigned char *(*msg));

#endif // _QCRY_CONTEXT_

