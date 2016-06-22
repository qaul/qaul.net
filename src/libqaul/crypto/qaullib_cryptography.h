/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QCRY_CONTEXT_
#define _QCRY_CONTEXT_


/***************** QCRY ERROR CODES *****************/

#define QCRY_STATUS_OK                  0
#define QCRY_STATUS_ERROR               1
#define QCRY_STATUS_FATAL               2

#define QCRY_STATUS_NOT_IMPLEMENTED     3

#define QCRY_STATUS_KEYGEN_FAILED       4
#define QCRY_STATUS_BFR_TOO_SMALL       10
#define QCRY_STATUS_INVALID_PARAMS      12

#define QCRY_STATUS_SEED_FAILED         14
#define QCRY_STATUS_CTX_INVALID         16
#define QCRY_STATUS_INVALID_TARGET      18
#define QCRY_STATUS_INVALID_KEYS        19

#define QCRY_STATUS_INVALID_KEYS        21
#define QCRY_STATUS_MALLOC_FAIL         22
#define QCRY_STATUS_KEY_BUSY            24

/***************** QCRY ERROR CODES *****************/

/* Flags used by the key generators */
#define QCRY_KEYS_AES             100
#define QCRY_KEYS_ECC             120
#define QCRY_KEYS_RSA             140
#define QCRY_KEYS_PREST_ON        180
#define QCRY_KEYS_PREST_OFF       230
#define QCRY_KEYS_QUIET_ON        350
#define QCRY_KEYS_QUIET_OFF       370
#define QCRY_KEYS_PERM_ON         410
#define QCRY_KEYS_PERM_OFF        415
#define QCRY_CIPH_CBC             425

/* Generic helper macros for all crypto code */

#define MAGICK_NO   3
#define MIN_BFR_S   4
#define MAX_TIMEOUT 500
#define TIME_SLEEP  50

/**
 * This macro checks if a buffer is full and should be increased in size.
 * Additionally it checks if a buffer is WAY too big for it's occupancy and
 * shrinks it. This macro should get used whenever things are
 * stored or removed from list buffers
 */
#define CHECK_BUFFER(type, bfr, max_s, curr_s) \
    { if(curr_s >= max_s) { \
        max_s *= 2; type *tmp = calloc(sizeof(type), max_s); \
        if(!tmp) return QCRY_STATUS_MALLOC_FAIL; \
        memcpy(tmp, bfr, sizeof(type) * curr_s); \
        free(bfr); \
        bfr = tmp; \
    } else if(curr_s * 3<= max_s) { max_s /= 2 ; \
    if(max_s < MIN_BFR_S) max_s = MIN_BFR_S; \
        type *tmp = calloc(sizeof(type), max_s); \
        if(!tmp) return QCRY_STATUS_MALLOC_FAIL; \
        memcpy(tmp, bfr, \
        sizeof(type) * curr_s); \
        free(bfr); \
        bfr = tmp; } \
    }

/**
 * create @a string from @a hash
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int QCry_HashToString(unsigned char *hash, char *string);

/**
 * reconverts a hash string @a string to the @a hash
 *
 * @retval 1 on success
 * @retval 0 on error
 */
int QCry_StringToHash(char *string, unsigned char *hash);


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

/************************************************************************************************
 ***
 ***
 ***
 ***
 ***
 ***
 ************************************************************************************************/

typedef struct {
    void            *dispatcher;


    unsigned int    max_conc;
    short           magno;
} qcry_arbit_ctx;

typedef struct {
    unsigned int        *sess_id;
    unsigned char       token[128];
} qcry_arbit_token;

int qcry_arbit_init(qcry_arbit_ctx *ctx, unsigned int max_concurrent);

int qcry_arbit_start(qcry_arbit_ctx *ctx, char *self, char *trgt, qcry_arbit_token *(*token));

int qcry_arbit_stop(qcry_arbit_ctx *ctx, qcry_arbit_token *token);

/************************************************************************************************
 ***
 ***
 ***
 ***
 ***
 ***
 ************************************************************************************************/


// TODO: These fields are depreciated now!
// TODO: Add key sizes to names
#define QCRY_KEYS_KL_AES 256
#define QCRY_KEYS_KL_ECC 192
#define QCRY_KEYS_KL_RSA 4096


/** Struct that includes the entropy and random seed generators for key
 * generation. This context can be kept between different accesses but should
 * be flushed from time to time (much scientific measurement of time).
 *
 * pr, Prediction resistence
 * mseed, define a manual seed
 * perm, Errors become warnings
 * quiet, warnings will not be logged
 */
typedef struct {
    mbedtls_entropy_context     entropy;
    mbedtls_ctr_drbg_context    rand;
    short                       pr, mseed, perm, quiet;
} qcry_keys_context;

/** Initialises a context with "sane default" settings */
int qcry_keys_init(qcry_keys_context *context);

int qcry_keys_init_all(qcry_keys_context *context, short pr, short mseed, short perm, short quiet);

/**
 * Function that creates a key ased on a few parameters passed in
 * by the key context and key type. Fills an output buffer with data.
 *
 * Will return != 0 if buffer is too small. If "quiet" flag is set on context
 * all errors will be ignored.
 */
int qcry_keys_gen(qcry_keys_context *context, short type, unsigned char *buf);

int qcry_keys_gen_m(qcry_keys_context *context, short type, unsigned char *(*buf));

/** Frees a key context and all neccessary sub-data */
int qcry_keys_free(qcry_keys_context *context);



#endif // _QCRY_CONTEXT_

