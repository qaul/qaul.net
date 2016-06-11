//
// Created by spacekookie on 11/06/16.
//

#ifndef QAUL_QCRY_KEYS_H
#define QAUL_QCRY_KEYS_H

#include "mbedtls/entropy.h"
#include "mbedtls/ctr_drbg.h"

#include "qcry_helper.h"

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

#endif //QAUL_QCRY_KEYS_H
