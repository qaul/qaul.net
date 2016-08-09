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
    char                *pf;
    mbedtls_pk_context  *pub;
};

qcry_ks_ctx *keystore;

/* Forward decloare load function for public keys */
int load_pubkey(mbedtls_pk_context **pub, const char *path, const char *fingerprint);

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
    for(int i = 0; i < entries; i++) {

        /* Get the fingerprint pointer for easier handling */
        struct qcry_usr_id *id = known[i];

        /* Load the apropriate key from the keystore */
        mbedtls_pk_context *pub;
        load_pubkey(&pub, keystore->key_path, id->username);

        /* Save the fingerprint in our collection */
        ret = qcry_ks_save(pub, id->fingerprint);
        if(ret != 0) return ret;
    }

    exit:
    return QCRY_STATUS_OK;
}

int qcry_ks_save(mbedtls_pk_context *pub, const char *fingerprint)
{
    CHECK_SANE

    struct key_entry *entry = (struct key_entry*) malloc(sizeof(struct key_entry) * 1);
    entry->pub = pub;

    size_t len = strlen(fingerprint) + 1;
    entry->pf = (char*) malloc(sizeof(char) * len);
    strcpy(entry->pf, fingerprint);

    /* Check if we have to increase our buffer */
    if(keystore->keys >= keystore->max) {
        realloc(keystore->keylist, keystore->max += 5); // Make it 5 bigger. Why 5? Dunno
        if(keystore->keylist == NULL) return QCRY_STATUS_MALLOC_FAIL;
    }

    /* Safely asign the new key entry to our list :) */
    keystore->keylist[keystore->keys++] = entry;
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
    for(int i = 0; i < keystore->keys; i++) {

        /* Compare the fingerprints for a match */
        if(strcmp(keystore->keylist[i]->pf, fingerprint) == 0) {

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
    for(int i = 0; i < keystore->keys; i++) {
        mbedtls_pk_free(keystore->keylist[i]->pub);
        free((keystore->keylist[i]->pf));
    }

    free(keystore->key_path);
    return QCRY_STATUS_OK;
}


/************ PRIVATE UTILITY FUNCTIONS BELOW ************/


int load_pubkey(mbedtls_pk_context **pub, const char *path, const char *fp)
{
    int ret = 0;

    /*** Malloc space for the pub and pri key values on heap ***/
    (*pub) = (mbedtls_pk_context*) calloc(sizeof(mbedtls_pk_context),  1);
    if(*pub == NULL) {
        ret = EXIT_FAILURE;
        goto cleanup;
    }

    /* Prepare path variables */
    size_t p_s = strlen(path);
    char pub_pth[512];

    /* Init new key context */
    mbedtls_pk_init(*pub);

    /* Build public key path (according to slashy-ness) */
    if(strcmp(&path[p_s - 1], "/") != 0)    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s/%s.pub", path, fp);
    else                                    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s%s.pub", path, fp);

    /*** Read keys off disk and initialise the contexts ***/
    mbedtls_printf("[KEYSTORE] Parsing key for %s...", fp);
    fflush(stdout);

    ret = mbedtls_pk_parse_public_keyfile(*pub, pub_pth);
    if(ret != 0) {
        mbedtls_printf("FAILED! mbedtls_pk_parse_public_keyfile returned 0x%04x\n", ret);
        goto cleanup;
    }

    mbedtls_printf("OK\n");
    return 0;

    cleanup:
    mbedtls_pk_free(*pub);
    return ret;
}