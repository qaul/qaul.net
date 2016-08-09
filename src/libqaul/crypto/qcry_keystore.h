/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <mbedtls/pk.h>

#ifndef QAUL_QCRY_KEYSTORE_H
#define QAUL_QCRY_KEYSTORE_H

/******************************************************************************
 *
 * This keystore maps fingerprints to public key files and allows to load and
 * parse keyfiles and return pk_contexts to the arbiter in order to start a
 * communication chain.
 *
 * To initialise the keystore you need to provide a list of usr_ids. A usr_id
 * in this context is a mapping of a username (for the key file name) to a
 * cryptographic fingerprint which is used in all other parts of the qcry-API
 * for identification
 *
 ******************************************************************************/

/**
 * This struct is used to easily provide a list of usernames
 * and known fingerprints to the keystore to load keys from
 * disk and initialise all known public keys more easily.
 */
typedef struct qcry_usr_id {
    char    *fingerprint;
    char    *username;
} qcry_usr_id;

struct key_entry;

typedef struct qcry_ks_ctx {
    char                *key_path;
    struct key_entry    **keylist;
    unsigned int        keys, max;
} qcry_ks_ctx;

int qcry_ks_init(const char *path, struct qcry_usr_id **known, int entries);

int qcry_ks_save(mbedtls_pk_context *pub, const char *fingerprint, const char *username);

int qcry_ks_getkey(mbedtls_pk_context *(*pub), const char *fingerprint);

int qcry_ks_getusername(char *(*username), const char *fingerprint);

int qcry_ks_free();

#endif //QAUL_QCRY_KEYSTORE_H
