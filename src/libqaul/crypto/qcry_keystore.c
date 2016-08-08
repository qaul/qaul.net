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
int load_keypair(mbedtls_pk_context **pub, const char *path, const char *username);

#define CHECK_SANE \
    if( !(keystore->keylist != NULL && keystore->max > 0 && keystore->key_path) ) \
        return QCRY_STATUS_INVALID_CTX;

int qcry_ks_init(const char *path, const char **fingerprints, int prints)
{
    keystore = (qcry_ks_ctx*) calloc(sizeof(qcry_ks_ctx), 1);

    size_t len = strlen(path) + 1;
    keystore->key_path = (char*) malloc(sizeof(char) * len);
    strcpy(keystore->key_path, path);

    /* Prepare some space for keys */
    keystore->keylist = (struct key_entry**) malloc(MIN_BFR_S * sizeof(struct key_entry*));
    memset(keystore->keylist, 0, MIN_BFR_S * sizeof(struct key_entry*));
    keystore->keys = 0;
    keystore->max = 8;

    /* Go and load all the keys */
    int i;
    for(i = 0; i < prints; i++) {

    }


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

    return QCRY_STATUS_OK;
}

int qcry_ks_getkey(mbedtls_pk_context *(*pub), const char *fingerprint)
{
    CHECK_SANE



    for(int i = 0; i < keystore->keys; i++) {
        if(strcmp(keystore->keylist[i]->pf, fingerprint) == 0) {
            (*pub) = keystore->keylist[i]->pub;
            break;
        }
    }

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


/*******************************************************************/


int load_keypair(mbedtls_pk_context **pub, const char *path, const char *fp)
{
    int ret = 0;

    /*** Malloc space for the pub and pri key values on heap ***/
    (*pub) = (mbedtls_pk_context*) calloc(sizeof(mbedtls_pk_context),  1);
    if(*pub == NULL) {
        ret = EXIT_FAILURE;
        goto cleanup;
    }

    /*** Initialise the key contexts properly ***/
    mbedtls_pk_init(*pub);

    /*** Construct the required file names ***/
    char pri_pth[512];
    char pub_pth[512];

    size_t p_s = strlen(path);
    size_t u_s = strlen(fp);

    /* Build public key path */
    if(strcmp(&path[p_s - 1], "/") != 0)    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s/00_%s.pub", path, fp);
    else                                    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s00_%s.pub", path, fp);

    /*** Read keys off disk and initialise the contexts ***/
    mbedtls_printf("Parsing public key %s...", fp);
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