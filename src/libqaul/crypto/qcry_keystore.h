/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "../mbedtls/include/mbedtls/pk.h"

#ifndef QAUL_QCRY_KEYSTORE_H
#define QAUL_QCRY_KEYSTORE_H

/************************************************************************************************
*** This keystore maps fingerprints to public key files and allows to load and
***   parse keyfiles and return pk_contexts to the arbiter in order to start a
***   communication chain.
***
***
************************************************************************************************/

struct key_entry;

typedef struct qcry_ks_ctx {
    char                *key_path;
    struct key_entry    **keylist;
    unsigned int        keys, max;
} qcry_ks_ctx;

int qcry_ks_init(const char *path, const char **fingerprints, int prints);

int qcry_ks_save(mbedtls_pk_context *pub, const char *fingerprint);

int qcry_ks_getkey(mbedtls_pk_context *(*pub), const char *fingerprint);

int qcry_ks_free();

#endif //QAUL_QCRY_KEYSTORE_H
