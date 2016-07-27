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


int qcry_key_generate(mbedtls_pk_context **key);

int qcry_generate_key();

int sign_msg(mbedtls_pk_context *key, const char *msgfile);

int verify_msg(mbedtls_pk_context *key, const char *signfile);

int qcry_sign_with_key();

#endif //QAUL_QCRY_PLAYGROUND_H
