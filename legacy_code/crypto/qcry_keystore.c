/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include <stdlib.h>
#include <memory.h>
#include <mbedtls/platform.h>
#include "qcry_keystore.h"
#include "qcry_helper.h"

typedef struct key_entry {
    char                *username; // Not vital but nice to know
    char                *fp;
    mbedtls_pk_context  *pub;
};

qcry_ks_ctx *keystore;

/* Forward decloare load function for public keys */
//int load_pubkey(mbedtls_pk_context **pub, const char *path, const char *fingerprint);

#define CHECK_SANE \
    if( !(keystore->keylist != NULL && keystore->max > 0 && keystore->key_path) ) \
        return QCRY_STATUS_INVALID_CTX;

int qcry_ks_init(const char *path, struct qcry_usr_id **known, int entries)
{
    int ret;
    keystore = (qcry_ks_ctx*) calloc(sizeof(qcry_ks_ctx), 1);
    if(keystore == NULL) return QCRY_STATUS_MALLOC_FAIL;

    size_t len = strlen(path) + 1;
    keystore->key_path = (char*) malloc(sizeof(char) * len);
    if(keystore->key_path == NULL) return QCRY_STATUS_MALLOC_FAIL;

    strcpy(keystore->key_path, path);

    /* Prepare some space for keys */
    keystore->keylist = (struct key_entry**) malloc(MIN_BFR_S * sizeof(struct key_entry*));
    if(keystore->keylist == NULL) return QCRY_STATUS_MALLOC_FAIL;

    memset(keystore->keylist, 0, MIN_BFR_S * sizeof(struct key_entry*));
    keystore->keys = 0;
    keystore->max = 8;

    /* Check that fingerprints exists */
    if(known == NULL) goto exit;

    /* Go and load all the keys */
    int i;
    for(i = 0; i < entries; i++) {

        /* Get the fingerprint pointer for easier handling */
        struct qcry_usr_id *id = known[i];

        /* Load the apropriate key from the keystore */
        mbedtls_pk_context *pub;
        qcry_load_pubkey(&pub, keystore->key_path, id->username);

        /* Save the fingerprint in our collection */
        ret = qcry_ks_save(pub, id->fingerprint, id->username);
        if(ret != 0) return ret;
    }

    exit:
    return QCRY_STATUS_OK;
}

int qcry_ks_save(mbedtls_pk_context *pub, const char *fingerprint, const char *username)
{
    CHECK_SANE

    struct key_entry *entry = (struct key_entry*) malloc(sizeof(struct key_entry) * 1);
    entry->pub = pub;

    /* Allocate and copy fingerprint */
    size_t fplen = strlen(fingerprint) + 1;
    entry->fp = (char*) malloc(sizeof(char) * fplen);
    if(entry->fp == NULL) return QCRY_STATUS_MALLOC_FAIL;
    strcpy(entry->fp, fingerprint);

    /* Allocate and copy username */
    size_t usrlen = strlen(username) + 1;
    entry->username = (char*) malloc(sizeof(char) * usrlen);
    if(entry->username == NULL) return QCRY_STATUS_MALLOC_FAIL;
    strcpy(entry->username, username);

    /* Check if we have to increase our buffer */
    if(keystore->keys >= keystore->max) {
        keystore->max += 5;
        keystore->keylist = (struct key_entry**) realloc(keystore->keylist, sizeof(struct key_entry) * keystore->max); // Make it 5 bigger. Why 5? Dunno
        if(keystore->keylist == NULL) return QCRY_STATUS_MALLOC_FAIL;
    }

    /* Safely asign the new key entry to our list :) */
    keystore->keylist[keystore->keys++] = entry;
    return QCRY_STATUS_OK;
}

int qcry_ks_getusername(char *(*username), const char *fingerprint)
{
    CHECK_SANE

    /*
     * Write some predictable data into the pointer to check
     * for if we can't find a key to match this request
     */
    (*username) = NULL;

    /* Loop through our collection of known public keys */
    int i;
    for(i = 0; i < keystore->keys; i++) {

        /* Compare the fingerprints for a match */
        if(strcmp(keystore->keylist[i]->fp, fingerprint) == 0) {

            /* Assign the pointer and goto end of function */
            (*username) = keystore->keylist[i]->username;
            goto exit;
        }
    }

    /* Exit label for a default return code of 0 */
    exit:
    return QCRY_STATUS_OK;
}

int qcry_ks_getkey(mbedtls_pk_context *(*pub), const char *fingerprint)
{
    CHECK_SANE

    /*
     * Write some predictable data into the pointer to check
     * for if we can't find a key to match this request
     */
    (*pub) = NULL;

    /* Loop through our collection of known public keys */
    int i;
    for(i = 0; i < keystore->keys; i++) {

        /* Compare the fingerprints for a match */
        if(strcmp(keystore->keylist[i]->fp, fingerprint) == 0) {

            /* Assign the pointer and goto end of function */
            (*pub) = keystore->keylist[i]->pub;
            goto exit;
        }
    }

    /* Exit label for a default return code of 0 */
    exit:
    return QCRY_STATUS_OK;
}

int qcry_ks_free()
{
    CHECK_SANE

    /* Make sure all keys on disk exist */
    int i;
    for(i = 0; i < keystore->keys; i++) {
        mbedtls_pk_free(keystore->keylist[i]->pub);
        free((keystore->keylist[i]->fp));
    }

    free(keystore->key_path);
    return QCRY_STATUS_OK;
}
