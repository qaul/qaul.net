/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef QAUL_QCRY_KEYS_H
#define QAUL_QCRY_KEYS_H

#include <mbedtls/ctr_drbg.h>
#include <mbedtls/entropy.h>
#include <mbedtls/pk.h>

/************************************************************************************************
*** This is a key and random data generator. It includes functions to create tokens,
***  random entropy data, symmetric keys as well as public/ private keys.
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
 * This special function will generate RSA keys from a specified key generator context. The context
 * is required for the entropy source and random seed generation. The provided reference will
 * be malloced so don't forget to free it again when you're done with it!
 *
 * @param ctx The key generator context
 * @param mul Combined key part that needs to be written to disk in two parts
 * @param pers A personal seed
 * @return
 */
int qcry_keys_rsagen(qcry_keys_context *ctx, mbedtls_pk_context *(*mul), const char *pers);

/**
* Function that creates a key ased on a few parameters passed in
 * by the key context and key type. Fills an output buffer with data.
 *
 * Will return != 0 if buffer is too small. If "quiet" flag is set on context
 * all errors will be ignored.
 *
 * @param context
 * @param type
 * @param buf
 * @return
 */
int qcry_keys_gen(qcry_keys_context *context, short type, unsigned char *buf);

/**
 * Same as qcry_keys_gen with the difference that it mallocs memory for you.
 * Don't forget to free memory again!
 *
 * @param context
 * @param type
 * @param buf
 * @return
 */
int qcry_keys_gen_m(qcry_keys_context *context, short type, unsigned char *(*buf));

/**
 * A function that lets you generate arbitrary lengths of random data.
 * Very useful for token creation.
 *
 * Don't forget to free memory again!
 *
 * @param context
 * @param length
 * @param buf
 * @return
 */
int qcry_keys_gen_r(qcry_keys_context *context, unsigned int length, unsigned char *(*buf));

/** Frees a key context and all neccessary sub-data */
int qcry_keys_free(qcry_keys_context *context);



#endif //QAUL_QCRY_KEYS_H
