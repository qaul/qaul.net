//
// Created by spacekookie on 27/06/16.
//

#ifndef QAUL_QCRY_CONTEXT_H
#define QAUL_QCRY_CONTEXT_H

/************************************************************************
 ***
 ***
 ***
 ***
 ***
 ***
 ***
 ***
 ************************************************************************/

#include <mbedtls/entropy.h>
#include <mbedtls/aes.h>
#include <mbedtls/pk.h>
#include <mbedtls/ctr_drbg.h>
#include <qaullib/qcry_hashing.h>
#include <stdbool.h>
#include <time.h>

#include "qcry_helper.h"

/** Describes the context in which a crypto context is held */
typedef enum {
    PK_RSA  = (1 << 1),
    ECDSA   = (1 << 2),
    AES256  = (1 << 3)
} qcry_ciph_t;


/**
 * A target for communication or verification. Contains every bit
 * of information that can be known about another user on the
 * network at THAT time.
 *
 * The types field is a bitset that can be used to select multiple ciphers
 */
typedef struct {

    /* Some basic identify information */
    const char              *username;
    const char              *fingerprint;
    mbedtls_pk_context      *public;

    /* Some target metadata */
    mbedtls_aes_context     *symm;
    qcry_ciph_t             types;
    short                   mgno;
} qcry_trgt_t;

/**
 * A user context contains all the identify information required for
 * successful communication and verification to other users on the
 * network.
 *
 * In addition to identify information and cipher metadata it also
 * contains'a list of "targets". A target is a recipient or a source
 * of communication and is thus always connected to a public key and
 * sometimes a symmetric cipher that can be used to encrypt long data.
 */
typedef struct qcry_usr_ctx {
    short               mgno;

    /* Store identify information */
    const char          *username;
    char                *fingerprint;
    time_t              birthdate;
    mbedtls_pk_context  *private, *public;
    qcry_ciph_t         ciph_t;
    unsigned int        ciph_len;

    /* Target data for this context */
    qcry_trgt_t         **trgts;
    unsigned int        usd_trgt;
    unsigned int        max_trgt;

    /* Seeds and entropy contexts */
    mbedtls_ctr_drbg_context    *ctr_drbg;
    mbedtls_entropy_context     *entropy;
} qcry_usr_ctx;

/* Used to check if initialisation was done on a target */
//#define CHECK_TARGET(ctx, trgt_no)  \
//    { if(ctx->trgts[trgt_no]->mgno != MAGICK_NO) return QCRY_STATUS_INVALID_TARGET; }

/* Helper macro to remove a target and all its allocated child heap memory */
//#define CLEAR_TARGET(ciph_t, trgt) \
//    { if(ciph_t == AES256) { \
//        mbedtls_aes_free(trgt->ctx.sk); \
//        free(trgt->ctx.sk); \
//        free(trgt->d.sk->sh_key_pri); \
//        free(trgt->d.sk); \
//    } else { \
//        mbedtls_pk_free(trgt->ctx.pk); \
//        free(trgt->ctx.pk); \
//        free(trgt->d.pk->usr_key_pub); \
//        free(trgt->d.pk->fp); \
//        free(trgt->d.pk); } \
//    free(trgt); }


// TODO: Change this into a macro!
static int QCRY_KEY_LEN[] = { 2048, 192, 256 };

//#define CHECK_KEYLEN(type, key) { \
//    size_t len = strlen(key); \
//    if(len != QCRY_KEY_LEN[type] return QCRY_STATUS_INVALID_KEYS ) }


/**
 * Initialises context for a username and a cipher type
 */
int qcry_context_init(qcry_usr_ctx *ctx, const char *usr_name, qcry_ciph_t ciph_t);
int qcry_context_free(qcry_usr_ctx *ctx);

/**
 * Attaches a key-pair to this user context which makes it actually operational.
 * If previous key-pairs were already present (for whatever reason) they will
 * be FREED from their suffering in memory!
 *
 * @param ctx
 * @param pub
 * @param pri
 * @return
 */
int qcry_context_attach(qcry_usr_ctx *ctx, mbedtls_pk_context *pub, mbedtls_pk_context *pri);

int qcry_context_add_trgt(qcry_usr_ctx *ctx, qcry_trgt_t *trgt, qcry_ciph_t ciph_t);

int qcry_context_remove_trgt(qcry_usr_ctx *ctx, unsigned int trgt_no);

int qcry_context_signmsg(qcry_usr_ctx *ctx, const char *msg, unsigned char *(*sign));

int qcry_context_verifymsg(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *msg, const unsigned char *sign, bool *ok);

/**
 * Small utility to create a target with a fingerprint safely.
 * So far this function will only create targets for public key
 * cryptography.
 *
 * @param trgt
 * @param fingerprint
 * @return
 */
int qcry_context_mktarget(qcry_trgt_t *(*trgt), const char *fingerprint);

//int qcry_sign_trgt(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *msg, size_t ilen, unsigned char *(*sign));
//int qcry_verify_trgt(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *ciph, size_t ilen, bool *ok);
//int qcry_encrypt_trgt(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *msg, size_t ilen, unsigned char *(*ciph));
//int qcry_decrypt_trgt(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *ciph, size_t ilen, unsigned char *(*msg));

#endif //QAUL_QCRY_CONTEXT_H
