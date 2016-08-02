//
// Created by spacekookie on 18/07/16.
//

#ifndef QAUL_QCRY_PLAYGROUND_H
#define QAUL_QCRY_PLAYGROUND_H


#include "mbedtls/entropy.h"
#include "mbedtls/ctr_drbg.h"
#include "mbedtls/bignum.h"
#include "mbedtls/x509.h"
#include "mbedtls/rsa.h"

#include <stdio.h>
#include <string.h>
#include <mbedtls/error.h>
#include <mbedtls/platform.h>

/** Generates a new PK context for this user */
int qcry_key_generate(mbedtls_pk_context **key, const char *pers);

/** Saves a PK context into a public and private key file */
int qcry_key_write(mbedtls_pk_context *key, const char *path, const char *username);

int qcry_key_destroy(mbedtls_pk_context *key);

/** Loads a key from disk into a PK context */
int qcry_key_load(mbedtls_pk_context **pub, mbedtls_pk_context **pri, const char *path, const char *username);

int qcry_generate_key();

int sign_msg(mbedtls_pk_context *pri, const char *msgfile);
//int sign_msg(mbedtls_pk_context *pri, const char *message, char *(*signature), size_t *sign_len);


int verify_msg(mbedtls_pk_context *pub, const char *signfile);

int qcry_sign_with_key();

#endif //QAUL_QCRY_PLAYGROUND_H
