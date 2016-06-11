//
// Created by spacekookie on 11/06/16.
//

#ifndef QAUL_QCRY_KEYS_H
#define QAUL_QCRY_KEYS_H

#include "mbedtls/entropy.h"
#include "mbedtls/ctr_drbg.h"

/**
 * Describes a key. Is used to generate different lengths
 * and patterns of keys for various crypto processes in libqaul.
 *
 * Not all of them may be in use. Do not ignore warnings thrown by
 * functions that take qcry_key_t as parameter as they might give
 * indication of the void-ness of a key type.
 */
typedef enum {
    AES,
    ECC, /* Main cryptographic workhorse */
    RSA
} qcry_key_t;

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
} qcry_key_context;

int qcry_key_init(qcry_key_context *context);

/**
 * Function that creates a key based on a few parameters passed in
 * by the key context and key type. Fills an output buffer with data.
 *
 * Will return != 0 if buffer is too small. If "quiet" flag is set on context
 * all errors will be ignored.
 */
int qcry_key_gen(qcry_key_context *context, qcry_key_t *type, unsigned char *buf);

#endif //QAUL_QCRY_KEYS_H
